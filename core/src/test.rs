use std::{fs, path::Path};

use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use indicatif::ProgressIterator;
use rkyv::rancor::Error;

use crate::{
    compressed_motif2::{CompactMotif, CompactMotifConfigurator},
    fingerprint::Fingerprint5,
    motifs::{Motif, generate_motifs},
    util::BinPerm,
};

pub fn order4() {
    let mut count = 0;
    let mut map = HashMap::with_hasher(FixedState::default());

    CompactMotif::<4>::enum_labelings(false, |m| {
        let fingerprint = m.fingerprint();
        if !map.contains_key(&fingerprint) {
            map.insert(m.fingerprint(), (m, 0));
        }
        map.get_mut(&fingerprint).unwrap().1 += 1;
        count += 1;
    });

    for (fingerprint, (motif, count)) in map.iter().progress() {
        println!("Bucket {}", motif);
        println!(
            "Bucket size {}, iso count {}",
            count,
            motif.isomorphism_count()
        );
    }

    println!("Found {} distinct fingerprints", map.len());

    // let motifs4 = generate_motifs(4);
    // let mut labelings_to_cr = HashMap::with_hasher(FixedState::default());
    //
    // for (cr, labelings) in motifs4.into_iter() {
    //     for labeling in labelings {
    //         labelings_to_cr.insert(labeling, cr.clone());
    //     }
    // }
    // println!("labelings to cr len {}", labelings_to_cr.len());
    //
    // for (fingerprint, motifs) in map.iter() {
    //     let motifs_vec = motifs
    //         .into_iter()
    //         .map(|m| Motif::from_vec(m.to_vec()))
    //         .collect::<Vec<Motif>>();
    //
    //     let mut cr_group = 0;
    //     let mut cr_groups = HashSet::new();
    //     let mut motifs_to_cr = Vec::new();
    //     for motif in motifs_vec.iter() {
    //         let cr = labelings_to_cr.get(motif).unwrap();
    //         motifs_to_cr.push((motif, cr_group, cr));
    //         if !cr_groups.contains(cr) {
    //             cr_groups.insert(cr);
    //             cr_group += 1;
    //         }
    //     }
    //
    //     if cr_group > 1 {
    //         println!("CR group, motif");
    //         println!("{:?}", fingerprint);
    //         for (motif, cr_group, _cr) in motifs_to_cr.iter() {
    //             println!("{:?}, {:?}", cr_group, motif);
    //         }
    //     }
    //
    //     // let cr = motifs_vec.iter().min().unwrap();
    // }
    //
    // println!("Buckets len {}", map.len());
}

const CACHE_DIR: &str = "cache";
const FILE_NAME: &str = "order5clushing.bin";

pub fn save_to_file(
    v: HashMap<Fingerprint5, Vec<CompactMotif<5>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ensure the directory exists
    let path = Path::new(CACHE_DIR);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let v: Vec<(usize, Vec<_>)> = v
        .into_iter()
        .enumerate()
        .map(|(i, (_key, items))| {
            // Map the inner items to just the 'container' field
            let containers = items.into_iter().map(|m| m.container).collect();
            (i, containers)
        })
        .collect();

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&v)?;

    let file_path = path.join(FILE_NAME);
    fs::write(file_path, bytes)?;

    Ok(())
}

pub fn load_from_file()
-> Result<HashMap<Fingerprint5, Vec<CompactMotif<5>>>, Box<dyn std::error::Error>> {
    let file_path = Path::new(CACHE_DIR).join(FILE_NAME);

    let bytes = fs::read(file_path)?;
    let deserialized: Vec<(usize, Vec<u32>)> =
        rkyv::from_bytes::<Vec<(usize, Vec<u32>)>, rkyv::rancor::Error>(&bytes)?;

    // Reconstruct the HashMap
    let rv: HashMap<Fingerprint5, Vec<CompactMotif<5>>> = deserialized
        .into_iter()
        .map(|(key, containers)| {
            let motifs = containers
                .into_iter()
                .map(|container| CompactMotif::<5> { container })
                .collect::<Vec<CompactMotif<5>>>();
            (motifs[0].fingerprint(), motifs)
        })
        .collect();

    Ok(rv)
}

pub fn order5() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Path::new(CACHE_DIR).join(FILE_NAME);

    let mut clushing_motifs;
    if cache.exists() {
        clushing_motifs = load_from_file()?;
    } else {
        clushing_motifs = HashMap::new();
        let mut count = 0;
        let mut map = HashMap::with_hasher(FixedState::default());

        let time = std::time::Instant::now();
        CompactMotif::<5>::enum_labelings(true, |m| {
            let fingerprint = m.fingerprint();
            if !map.contains_key(&fingerprint) {
                map.insert(m.fingerprint(), (m, 0));
            }
            map.get_mut(&fingerprint).unwrap().1 += 1;
            count += 1;
        });
        println!("Time taken: {:?}", time.elapsed());

        let mut clushing_fingerprints = HashMap::new();
        for (fingerprint, (motif, count)) in map.into_iter() {
            if motif.isomorphism_count() != count {
                clushing_fingerprints.insert(fingerprint, vec![]);
            }
        }

        CompactMotif::<5>::enum_labelings(true, |m| {
            let fingerprint = m.fingerprint();
            clushing_fingerprints
                .entry(fingerprint)
                .and_modify(|v| v.push(m));
        });

        for (fingerprint, motifs) in clushing_fingerprints.into_iter() {
            let mut unique_motifs = HashSet::new();
            for motif in motifs {
                unique_motifs.insert(motif);
            }

            let mut iso_groups = HashMap::new();
            for motif in unique_motifs.clone().into_iter() {
                iso_groups.insert(motif, -1);
            }

            let mut curr_group = 0;
            for (motif) in unique_motifs.into_iter() {
                if *iso_groups.get(&motif).unwrap() != -1 {
                    continue;
                }
                motif.enum_isomorphism(|iso| {
                    *iso_groups.get_mut(&iso).unwrap() = curr_group;
                });
                curr_group += 1;
            }

            let mut rv = HashMap::new();
            for (motif, group) in iso_groups.into_iter() {
                rv.entry(group).or_insert(vec![]).push(motif);
            }
            clushing_motifs.insert(
                fingerprint,
                rv.into_iter().map(|(_group, motifs)| motifs[0]).collect(),
            );
        }

        save_to_file(clushing_motifs.clone())?;
    }

    println!("Found {} clushing buckets", clushing_motifs.len());

    let min = clushing_motifs
        .iter()
        .min_by_key(|(fingerprint, v)| v[0].edge_count());

    match min {
        Some((fingerprint, motifs)) => {
            println!("Clushing fingerprint {:?}", fingerprint);
            for m in motifs {
                println!("{}", m);
            }
        }
        None => println!("No clushing buckets found."),
    }

    Ok(())
}
