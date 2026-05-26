use crate::compressed_motif2::{CMAssociated, CompactMotif, CompactMotifConfigurator};
use crate::util::{sort4, sort5};
use std::fmt::Debug;
use std::hash::Hash;

pub struct Fingerprint2;
pub struct Fingerprint3;

// pub trait CMAssociated {
//     type CMType;
// }
// #[hoist_mod(attr(duplicate_item(
//     struct_name     associated_type;
//     // [Fingerprint2]  [CompressedMotif2];
//     // [Fingerprint3]  [CompressedMotif3];
//     [Fingerprint4]  [CompressedMotif4];
//     [Fingerprint5]  [CompressedMotif5];
// )))]
// mod __ {
//
//     impl CMAssociated for struct_name {
//         type CMType = associated_type;
//     }
// }

impl From<CompactMotif<4>> for Fingerprint4 {
    fn from(cm: CompactMotif<4>) -> Self {
        let mut order_map =
            [0u8; <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::SIZE];
        for nodes in cm.iter_nodes() {
            for n in nodes {
                order_map[n] += 1 << (2 * (nodes.len() as usize - 2));
            }
        }
        sort4(&mut order_map);

        let mut inclusions = [0u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT];
        for e in cm {
            for inner in <Self as CMAssociated>::CMType::FULL_OVERLAPS[e] & cm {
                inclusions[inner] += 1;
            }
        }

        for e in cm {
            inclusions[e] -= 1;
        }

        Fingerprint4 {
            order_map,
            inclusions,
        }
    }
}

impl From<CompactMotif<5>> for Fingerprint5 {
    fn from(cm: CompactMotif<5>) -> Self {
        let mut order_map = [0u16; <Self as CMAssociated>::CMType::SIZE];

        for e in cm {
            let nodes = <Self as CMAssociated>::CMType::NODE_MAP[e];
            // order_map[nodes.len() as usize - 1] += 1;
            for n in nodes {
                order_map[n] += 1 << (3 * (nodes.len() as usize - 2));
                // splitting u16 in 3 parts for hyperedges of size 2,3 and 4. This will make hashing easy and fast
                // degrees[n] += 1;
            }
        }
        sort5(&mut order_map);

        let mut edges_props = [0u16; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT];
        // let mut full_overlaps = [0u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT];
        // let mut part_overlaps = [0u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT];
        // let mut inclusions = [0u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT];

        for e in cm {
            let full_overlaps = cm.full_ovelaps(e).edge_count() as u8 - 1; // -1, removing self
            let part_overlaps = cm.part_ovelaps(e).edge_count().count_ones() as u8 - 1;

            edges_props[e] = (full_overlaps as u16) | ((part_overlaps as u16) << 5);

            let mut inner_edges = cm.full_ovelaps(e);
            inner_edges.remove_edge(e);
            for inner in inner_edges {
                // len > 2 since there are not multiedges and we consider only full overlaps
                let len = <Self as CMAssociated>::CMType::NODE_MAP[e].len();
                if len < 3 {
                    panic!("Wrong len: {}", len);
                }
                edges_props[inner] += 1 << (10 + 2 * (len - 3));
            }
        }

        Fingerprint5 {
            order_map,
            edges_props,
        }
    }
}

pub struct Fingerprint4 {
    /// for each node, a histogram of the sizes of the edges it participates in. Sorted by node
    /// degree, then lexicographically by histogram
    order_map: [u8; 4],
    /// For each edge, the number of edges in which its fully contained
    inclusions: [u8; 11],
}

impl Fingerprint4 {
    const SIZE: usize = 4;
    const MAX_EDGE_COUNT: usize = 11;
}

impl Debug for Fingerprint4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rv = String::new();
        rv += format!("Order map: \n").as_str();
        for i in 0..<Self as CMAssociated>::CMType::SIZE {
            let order2 = (self.order_map[i] >> 0) & ((1 << 2) - 1);
            let order3 = (self.order_map[i] >> 2) & ((1 << 2) - 1);
            let order4 = (self.order_map[i] >> 4) & ((1 << 2) - 1);
            rv += format!("\t {:?}\n", [order2, order3, order4]).as_str();
        }
        rv += format!("Inclusions: {:?}\n", self.inclusions).as_str();
        f.write_str(rv.as_str())
    }
}

impl Hash for Fingerprint4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut inclusions_buckets = [0u8; Self::MAX_EDGE_COUNT];

        for i in 0..Self::MAX_EDGE_COUNT {
            inclusions_buckets[self.inclusions[i] as usize] += 1;
        }

        for i in 0..Self::MAX_EDGE_COUNT {
            inclusions_buckets[i].hash(state);
        }

        for i in 0..Self::SIZE {
            self.order_map[i].hash(state);
        }
    }
}

impl PartialEq for Fingerprint4 {
    fn eq(&self, other: &Self) -> bool {
        let mut inclusions_buckets = [0u8; Self::MAX_EDGE_COUNT];
        let mut other_inclusions_buckets = [0u8; Self::MAX_EDGE_COUNT];

        for i in 0..Self::MAX_EDGE_COUNT {
            inclusions_buckets[self.inclusions[i] as usize] += 1;
            other_inclusions_buckets[other.inclusions[i] as usize] += 1;
        }

        for i in 0..Self::MAX_EDGE_COUNT {
            if inclusions_buckets[i] != other_inclusions_buckets[i] {
                return false;
            }
        }
        if self.order_map != other.order_map {
            return false;
        }

        true
    }
}

impl Eq for Fingerprint4 {}

pub struct Fingerprint5 {
    /// for each node, a histogram of the sizes of the edges it participates in. Sorted by node
    /// degree, then lexicographically by histogram
    order_map: [u16; <Self as CMAssociated>::CMType::SIZE],

    /// For each edge it contains the following information:
    /// bit 0-5:    the number of edges that are fully contained within it.
    /// bit 5-10:   the number of edges that overlap with it in at least one node (including itself)
    /// bit 10-15:  the number of edges in which its fully contained
    edges_props: [u16; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT],
    // full_overlaps: [u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT],
    // part_overlaps: [u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT],
    // inclusions: [u8; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT],
}

// 1: 475833 order map + inclusions
// 2: 533783 1 + full overlaps
// 3: 547240 2 + part overlaps
impl Fingerprint5 {
    const SIZE: usize = <Self as CMAssociated>::CMType::SIZE;
    const MAX_EDGE_COUNT: usize = <Self as CMAssociated>::CMType::MAX_EDGE_COUNT;
}

impl Debug for Fingerprint5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rv = String::new();
        rv += format!("Order map: \n").as_str();
        for i in 0..<Self as CMAssociated>::CMType::SIZE {
            let order2 = (self.order_map[i] >> 0) & ((1 << 3) - 1);
            let order3 = (self.order_map[i] >> 3) & ((1 << 3) - 1);
            let order4 = (self.order_map[i] >> 6) & ((1 << 3) - 1);
            rv += format!("\t {:?}\n", [order2, order3, order4]).as_str();
        }

        // rv += format!("Full overlaps: {:?}\n", self.full_overlaps).as_str();
        // rv += format!("Part overlaps: {:?}\n", self.part_overlaps).as_str();
        // rv += format!("Inclusions: {:?}\n", self.inclusions).as_str();
        f.write_str(rv.as_str())
    }
}

impl Hash for Fingerprint5 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        let mut part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        let mut inclusions_buckets = [0u8; 1 << 5];

        for i in 0..Self::MAX_EDGE_COUNT {
            let full_count = (self.edges_props[i] >> 0) & ((1 << 5) - 1);
            let part_count = (self.edges_props[i] >> 5) & ((1 << 5) - 1);
            let incs_count = (self.edges_props[i] >> 10) & ((1 << 5) - 1);

            full_overlaps_buckets[full_count as usize] += 1;
            part_overlaps_buckets[part_count as usize] += 1;
            inclusions_buckets[incs_count as usize] += 1;
        }

        for i in 0..Self::MAX_EDGE_COUNT {
            full_overlaps_buckets[i].hash(state);
            part_overlaps_buckets[i].hash(state);
        }

        for i in 0..(1 << 5) {
            inclusions_buckets[i].hash(state);
        }

        for i in 0..Self::SIZE {
            self.order_map[i].hash(state);
        }
    }
}

impl PartialEq for Fingerprint5 {
    fn eq(&self, other: &Self) -> bool {
        let mut full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        let mut other_full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];

        let mut part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        let mut other_part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];

        let mut inclusions_buckets = [0u8; 1 << 5];
        let mut other_inclusions_buckets = [0u8; 1 << 5];

        for i in 0..Self::MAX_EDGE_COUNT {
            let full_count = ((self.edges_props[i] >> 0) & ((1 << 5) - 1)) as usize;
            let part_count = ((self.edges_props[i] >> 5) & ((1 << 5) - 1)) as usize;
            let incs_count = ((self.edges_props[i] >> 10) & ((1 << 5) - 1)) as usize;

            full_overlaps_buckets[full_count] += 1;
            other_full_overlaps_buckets[full_count] += 1;

            part_overlaps_buckets[part_count] += 1;
            other_part_overlaps_buckets[part_count] += 1;

            inclusions_buckets[incs_count] += 1;
            other_inclusions_buckets[incs_count] += 1;
        }

        for i in 0..Self::MAX_EDGE_COUNT {
            if full_overlaps_buckets[i] != other_full_overlaps_buckets[i] {
                return false;
            }
            if part_overlaps_buckets[i] != other_part_overlaps_buckets[i] {
                return false;
            }
        }

        for i in 0..(1 << 5) {
            if inclusions_buckets[i] != other_inclusions_buckets[i] {
                return false;
            }
        }
        if self.order_map != other.order_map {
            return false;
        }

        true
    }
}

impl Eq for Fingerprint5 {}
