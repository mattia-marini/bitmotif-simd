use crate::compressed_motif2::{CMAssociated, CompactMotif, CompactMotifConfigurator};
use std::fmt::Debug;
use std::hash::Hash;

pub struct Fingerprint2;
pub struct Fingerprint3;

impl From<CompactMotif<4>> for Fingerprint4 {
    fn from(cm: CompactMotif<4>) -> Self {
        let mut order_map =
            [0u8; <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::SIZE];
        for nodes in cm.iter_nodes() {
            for n in nodes {
                order_map[n] += 1 << (2 * (nodes.len() as usize - 2));
            }
        }
        order_map.sort_unstable();
        // sort4(&mut order_map);

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
        let mut rv = Fingerprint5::new();
        rv.build_order_map(&cm);
        rv.build_edge_connection_map(&cm);
        rv
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Fingerprint5 {
    /// for each node, a histogram of the sizes of the edges it participates in. Sorted by node
    /// degree, then lexicographically by histogram
    order_map: [u16; <Self as CMAssociated>::CMType::SIZE],

    /// For each edge it contains the following information:
    /// bit 0-5:    the number of edges that are fully contained within it.
    /// bit 5-10:   the number of edges that overlap with it in at least one node (including itself)
    /// bit 10-15:  the number of edges in which its fully contained
    // edges_props: [u16; <Self as CMAssociated>::CMType::MAX_EDGE_COUNT],

    /// For each 2-edge it stores information about its connectivity.
    /// One could think of it as a small tensor with informations stored as follows:
    /// cover_tree[overlapping node set size][group id of the overlapping node set][overlapping edge size] = number of edges with this configuration
    edge_connection_map: (
        // [2x5b][1x4b][2 empty bits]
        [u16; CompactMotif::<5>::max_edge_count(2)],
        // [3x3b][3x3b][1x2b][12 empty bits]
        [u32; CompactMotif::<5>::max_edge_count(3)],
        // [4x1b][4x1b][4x1b][4 empty bits]
        [u16; CompactMotif::<5>::max_edge_count(4)],
    ),

    edge_connection_map_sizes: (usize, usize, usize),
}

// 1: 475833 order map + inclusions
// 2: 533783 1 + full overlaps
// 3: 547240 2 + part overlaps
impl Fingerprint5 {
    pub const SIZE: usize = <Self as CMAssociated>::CMType::SIZE;
    pub const MAX_EDGE_COUNT: usize = <Self as CMAssociated>::CMType::MAX_EDGE_COUNT;

    pub const FULL_OVERLAPS:
        <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::FullOverlapsType =
        <Self as CMAssociated>::CMType::FULL_OVERLAPS;

    pub const PART_OVERLAPS:
        <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::PartOverlapsType =
        <Self as CMAssociated>::CMType::PART_OVERLAPS;

    pub const NODE_MAP: <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::NodeMapType =
        <Self as CMAssociated>::CMType::NODE_MAP;

    pub const EDGE_MAP: <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::EdgeMapType =
        <Self as CMAssociated>::CMType::EDGE_MAP;

    pub const ADJ: <<Self as CMAssociated>::CMType as CompactMotifConfigurator>::AdjType =
        <Self as CMAssociated>::CMType::ADJ;
}

impl Fingerprint5 {
    pub fn new() -> Self {
        Self {
            order_map: [0u16; <Self as CMAssociated>::CMType::SIZE],
            edge_connection_map: (
                [0u16; CompactMotif::<5>::max_edge_count(2)],
                [0u32; CompactMotif::<5>::max_edge_count(3)],
                [0u16; CompactMotif::<5>::max_edge_count(4)],
            ),
            edge_connection_map_sizes: (0, 0, 0),
        }
    }

    pub fn build_order_map(&mut self, cm: &CompactMotif<5>) {
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
        order_map.sort_unstable();

        self.order_map = order_map;
    }

    pub fn build_edge_connection_map(&mut self, cm: &CompactMotif<5>) {
        let mut edge_connection_map_sizes = (0, 0, 0);
        let mut edge_connection_map = (
            [0u16; CompactMotif::<5>::max_edge_count(2)],
            [0u32; CompactMotif::<5>::max_edge_count(3)],
            [0u16; CompactMotif::<5>::max_edge_count(4)],
        );

        macro_rules! insert_cross_edge {
            ($e:ident, $size: literal, $output: expr) => {
                {
                    // let entry = 0;
                    let mut cross_edges = cm.part_ovelaps($e) & !cm.full_ovelaps($e);
                    cross_edges.remove_edge($e);

                    // const structs consider only edge with order >1, so if overlapping size is 1 we need to handle separately the group id
                    let mut group_idx_by_size = [0; $size + 1];
                    let mut single_edge_groups = [usize::MAX; Self::SIZE];
                    let mut groups_idx = [usize::MAX; Self::MAX_EDGE_COUNT];

                    let mut output = $output;

                    for inner in cross_edges {
                        let overlapping_nodes = Self::NODE_MAP[inner] & Self::NODE_MAP[$e];
                        let overlapping_size = overlapping_nodes.len() as usize;
                        let inner_hx_len = Self::NODE_MAP[inner].len() as usize;

                        if inner_hx_len == 5 {
                            continue;
                        }

                        let overlapping_group_idx = if overlapping_size == 1 {
                            let overlapping_node = overlapping_nodes.iter().next().unwrap();
                            if single_edge_groups[overlapping_node] == usize::MAX {
                                single_edge_groups[overlapping_node] = group_idx_by_size[0];
                                group_idx_by_size[0] += 1;
                                single_edge_groups[overlapping_node]
                            } else {
                                single_edge_groups[overlapping_node]
                            }
                        } else {
                            let edge_with_overlapping_nodes =
                                Self::EDGE_MAP[overlapping_nodes.nodes as usize] as usize;
                            if groups_idx[edge_with_overlapping_nodes] == usize::MAX {
                                groups_idx[edge_with_overlapping_nodes] =
                                    group_idx_by_size[overlapping_size - 1];
                                group_idx_by_size[overlapping_size - 1] += 1;
                                groups_idx[edge_with_overlapping_nodes]
                            } else {
                                groups_idx[edge_with_overlapping_nodes]
                            }
                        };

                        // entry += 1
                        //     << (OUTER_OFFSETS[overlapping_size - 1]
                        //         + BLOCK_SIZE[overlapping_size - 1] * overlapping_group_idx
                        //         + INNER_OFFSET[overlapping_size - 1][inner_hx_len - 2]);

                        output[OVERLAP_GROUP_OFFSET[overlapping_size - 1] + overlapping_group_idx] += 1 << INNER_OFFSET[overlapping_size - 1][inner_hx_len - 2];

                        // output[OVERLAP_GROUP_OFFSET[overlapping_size - 1]] += 1 << (INNER_OFFSET[overlapping_size - 1][inner_hx_len - 2]);


                    }

                    output
                }
            };
        }

        for e in cm {
            match Self::NODE_MAP[e].len() {
                2 => {
                    const OVERLAP_GROUP_OFFSET: [usize; 2] = [0, 2];

                    // const OUTER_OFFSETS: [usize; 2] = [0, 10];
                    // const BLOCK_SIZE: [usize; 2] = [5, 4];
                    const INNER_OFFSET: [[usize; 3]; 2] = [[0, 2, 4], [0, 0, 2]];

                    let mut edge_infos = insert_cross_edge!(e, 2, [0u8; 3]);
                    // sort_slice2(&mut edge_infos[0..2]);
                    edge_infos[0..2].sort_unstable();

                    let entry = ((edge_infos[0] as u16) << 0)
                        | ((edge_infos[1] as u16) << 5)
                        | ((edge_infos[2] as u16) << 10);
                    edge_connection_map.0[edge_connection_map_sizes.0] = entry;
                    edge_connection_map_sizes.0 += 1;
                }
                3 => {
                    const OVERLAP_GROUP_OFFSET: [usize; 3] = [0, 3, 6];

                    // const OUTER_OFFSETS: [usize; 3] = [0, 9, 18];
                    // const BLOCK_SIZE: [usize; 3] = [3, 3, 2];
                    const INNER_OFFSET: [[usize; 3]; 3] = [[0, 2, 3], [0, 0, 2], [0, 0, 0]];

                    let mut edge_infos = insert_cross_edge!(e, 3, [0u8; 7]);
                    // sort_slice3(&mut edge_infos[0..3]);
                    // sort_slice3(&mut edge_infos[3..6]);
                    edge_infos[0..3].sort_unstable();
                    edge_infos[3..6].sort_unstable();

                    let entry = (edge_infos[0] as u32) << 0
                        | (edge_infos[1] as u32) << 3
                        | (edge_infos[2] as u32) << 6
                        | (edge_infos[3] as u32) << 9
                        | (edge_infos[4] as u32) << 12
                        | (edge_infos[5] as u32) << 15
                        | (edge_infos[6] as u32) << 18;
                    edge_connection_map.1[edge_connection_map_sizes.1] = entry;
                    edge_connection_map_sizes.1 += 1;
                }
                4 => {
                    const OVERLAP_GROUP_OFFSET: [usize; 4] = [0, 4, 10, 14];

                    // const OUTER_OFFSETS: [usize; 4] = [0, 4, 10, 14];
                    // const BLOCK_SIZE: [usize; 4] = [1, 1, 1, 0];
                    const INNER_OFFSET: [[usize; 3]; 4] =
                        [[0, 1, 1], [0, 0, 1], [0, 0, 0], [0, 0, 0]];

                    let mut edge_infos = insert_cross_edge!(e, 4, [0u8; 15]);
                    // sort_slice4(&mut edge_infos[0..4]);
                    // sort_slice6(&mut edge_infos[4..10]);
                    // sort_slice4(&mut edge_infos[10..14]);
                    edge_infos[0..4].sort_unstable();
                    edge_infos[4..10].sort_unstable();
                    edge_infos[10..14].sort_unstable();
                    let entry = (edge_infos[0] as u16) << 0
                        | (edge_infos[1] as u16) << 1
                        | (edge_infos[2] as u16) << 2
                        | (edge_infos[3] as u16) << 3
                        | (edge_infos[4] as u16) << 4
                        | (edge_infos[5] as u16) << 5
                        | (edge_infos[6] as u16) << 6
                        | (edge_infos[7] as u16) << 7
                        | (edge_infos[8] as u16) << 8
                        | (edge_infos[9] as u16) << 9
                        | (edge_infos[10] as u16) << 10
                        | (edge_infos[11] as u16) << 11
                        | (edge_infos[12] as u16) << 12
                        | (edge_infos[13] as u16) << 13;
                    edge_connection_map.2[edge_connection_map_sizes.2] = entry;
                    edge_connection_map_sizes.2 += 1;
                }
                // 3 => {}
                // 4 => {}
                5 => {}
                _ => panic!("Wrong edge size"),
            }
        }

        edge_connection_map.0[..edge_connection_map_sizes.0].sort_unstable();
        edge_connection_map.1[0..edge_connection_map_sizes.1].sort_unstable();
        edge_connection_map.2[0..edge_connection_map_sizes.2].sort_unstable();
        // sort_net(&mut edge_connection_map.0[0..edge_connection_map_sizes.0]);
        // sort_net(&mut edge_connection_map.1[..edge_connection_map_sizes.1]);
        // sort_net(&mut edge_connection_map.2[..edge_connection_map_sizes.2]);

        self.edge_connection_map = edge_connection_map;
        self.edge_connection_map_sizes = edge_connection_map_sizes;
    }
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
        for i in 0..Self::SIZE {
            self.order_map[i].hash(state);
        }
        // let mut full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        // let mut part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        // let mut inclusions_buckets = [0u8; 1 << 5];
        //
        // for i in 0..Self::MAX_EDGE_COUNT {
        //     let full_count = (self.edges_props[i] >> 0) & ((1 << 5) - 1);
        //     let part_count = (self.edges_props[i] >> 5) & ((1 << 5) - 1);
        //     let incs_count = (self.edges_props[i] >> 10) & ((1 << 5) - 1);
        //
        //     full_overlaps_buckets[full_count as usize] += 1;
        //     part_overlaps_buckets[part_count as usize] += 1;
        //     inclusions_buckets[incs_count as usize] += 1;
        // }
        //
        // // hash the overlap/inclusion buckets
        // for i in 0..Self::MAX_EDGE_COUNT {
        //     full_overlaps_buckets[i].hash(state);
        //     part_overlaps_buckets[i].hash(state);
        // }
        //
        // for i in 0..(1 << 5) {
        //     inclusions_buckets[i].hash(state);
        // }

        for i in 0..self.edge_connection_map_sizes.0 {
            self.edge_connection_map.0[i].hash(state);
        }

        for i in 0..self.edge_connection_map_sizes.1 {
            self.edge_connection_map.1[i].hash(state);
        }
        for i in 0..self.edge_connection_map_sizes.2 {
            self.edge_connection_map.2[i].hash(state);
        }
    }
}

impl PartialEq for Fingerprint5 {
    fn eq(&self, other: &Self) -> bool {
        if self.order_map != other.order_map {
            return false;
        }
        // let mut full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        // let mut other_full_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        //
        // let mut part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        // let mut other_part_overlaps_buckets = [0u8; Self::MAX_EDGE_COUNT];
        //
        // let mut inclusions_buckets = [0u8; 1 << 5];
        // let mut other_inclusions_buckets = [0u8; 1 << 5];
        //
        // for i in 0..Self::MAX_EDGE_COUNT {
        //     let full_count = ((self.edges_props[i] >> 0) & ((1 << 5) - 1)) as usize;
        //     let part_count = ((self.edges_props[i] >> 5) & ((1 << 5) - 1)) as usize;
        //     let incs_count = ((self.edges_props[i] >> 10) & ((1 << 5) - 1)) as usize;
        //
        //     let full_count_o = ((other.edges_props[i] >> 0) & ((1 << 5) - 1)) as usize;
        //     let part_count_o = ((other.edges_props[i] >> 5) & ((1 << 5) - 1)) as usize;
        //     let incs_count_o = ((other.edges_props[i] >> 10) & ((1 << 5) - 1)) as usize;
        //
        //     full_overlaps_buckets[full_count] += 1;
        //     other_full_overlaps_buckets[full_count_o] += 1;
        //
        //     part_overlaps_buckets[part_count] += 1;
        //     other_part_overlaps_buckets[part_count_o] += 1;
        //
        //     inclusions_buckets[incs_count] += 1;
        //     other_inclusions_buckets[incs_count_o] += 1;
        // }
        //
        // for i in 0..Self::MAX_EDGE_COUNT {
        //     if full_overlaps_buckets[i] != other_full_overlaps_buckets[i] {
        //         return false;
        //     }
        //     if part_overlaps_buckets[i] != other_part_overlaps_buckets[i] {
        //         return false;
        //     }
        // }
        //
        // for i in 0..(1 << 5) {
        //     if inclusions_buckets[i] != other_inclusions_buckets[i] {
        //         return false;
        //     }
        // }

        // compare edge_connection_map as order-invariant multisets by sorting copies
        let self_conn2 = self.edge_connection_map.0;
        let other_conn2 = other.edge_connection_map.0;
        if self_conn2[0..self.edge_connection_map_sizes.0]
            != other_conn2[0..other.edge_connection_map_sizes.0]
        {
            return false;
        }

        let self_conn3 = self.edge_connection_map.1;
        let other_conn3 = other.edge_connection_map.1;
        if self_conn3[0..self.edge_connection_map_sizes.1]
            != other_conn3[0..other.edge_connection_map_sizes.1]
        {
            return false;
        }

        let self_conn4 = self.edge_connection_map.2;
        let other_conn4 = other.edge_connection_map.2;
        if self_conn4[0..self.edge_connection_map_sizes.2]
            != other_conn4[0..other.edge_connection_map_sizes.2]
        {
            return false;
        }

        true
    }
}

impl Eq for Fingerprint5 {}
