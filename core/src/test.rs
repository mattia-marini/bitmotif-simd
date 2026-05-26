use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};

use crate::{
    compressed_motif2::CompactMotif,
    motifs::{Motif, generate_motifs},
};

pub fn order4() {
    let mut count = 0;
    let mut map = HashMap::with_hasher(FixedState::default());
    CompactMotif::<4>::enum_labelings(false, |m| {
        let fingerprint = m.fingerprint();
        // println!("{}", m);
        // println!("{:?}", fingerprint);
        if !map.contains_key(&fingerprint) {
            map.insert(m.fingerprint(), vec![]);
        }
        map.get_mut(&fingerprint).unwrap().push(m);
        count += 1;
    });

    println!("Found {} distinct fingerprints", map.len());
    for (fingerprint, motifs) in map.iter() {
        println!("{}", motifs.len());
        println!("{}", motifs[0]);
        // println!("{:?} has {} motifs", fingerprint, motifs.len());
    }

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

pub fn order5() {
    let mut count = 0;
    // let mut map = HashMap::with_hasher(FixedState::default());
    let mut set = HashSet::with_hasher(FixedState::default());
    let time = std::time::Instant::now();

    CompactMotif::<5>::enum_labelings(true, |m| {
        let fingerprint = m.fingerprint();
        // println!("{}", m);
        // println!("{:?}", fingerprint);
        if !set.contains(&fingerprint) {
            // map.insert(m.fingerprint(), vec![]);
            set.insert(fingerprint);
            count += 1;
        }
        // map.get_mut(&fingerprint).unwrap().push(m);
    });
    let elapsed = time.elapsed();

    println!(
        "Found {} distinct fingerprints in {}",
        count,
        elapsed.as_secs_f32()
    );
}
