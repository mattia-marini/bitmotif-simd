use foldhash::fast::FixedState;
use itertools::Itertools;
use std::collections::VecDeque;
use std::hash::Hash;

use hashbrown::{HashMap, HashSet};

type Node = usize;
pub type UnweightedEdge = Vec<Node>;
pub type UnweightedHypergraph = Vec<UnweightedEdge>;

// Helper functions kept exactly as they are in your code for compatibility
fn relabel_unweighted(
    hg: &UnweightedHypergraph,
    mapping: &HashMap<Node, Node>,
) -> UnweightedHypergraph {
    let mut res: UnweightedHypergraph = hg
        .iter()
        .map(|nodes| {
            let mut new_nodes: Vec<Node> = nodes
                .iter()
                .map(|v| *mapping.get(v).expect("missing node in mapping"))
                .collect();
            new_nodes.sort_unstable();
            new_nodes
        })
        .collect();
    res.sort();
    res
}

pub fn enum_isomorphisms(hg: &UnweightedHypergraph, n: Option<usize>) -> Vec<UnweightedHypergraph> {
    let n = n.unwrap_or_else(|| {
        let mut distinct: HashSet<Node> = HashSet::new();
        for edge_nodes in hg.iter() {
            distinct.extend(edge_nodes.iter().copied());
        }
        distinct.len()
    });

    let base: Vec<Node> = (1..=n).collect();
    let mut labelings: Vec<UnweightedHypergraph> = Vec::new();

    for perm in base.iter().copied().permutations(n) {
        let mapping: HashMap<Node, Node> = (1..=n).zip(perm.into_iter()).collect();
        labelings.push(relabel_unweighted(hg, &mapping));
    }
    labelings
}

pub fn get_canonical_representative(
    hg: &UnweightedHypergraph,
    n: Option<usize>,
) -> UnweightedHypergraph {
    let n = n.unwrap_or_else(|| {
        let mut distinct: HashSet<Node> = HashSet::new();
        for edge_nodes in hg.iter() {
            distinct.extend(edge_nodes.iter().copied());
        }
        distinct.len()
    });

    let base: Vec<Node> = (1..=n).collect();
    let mut best: Option<UnweightedHypergraph> = None;
    for perm in base.iter().copied().permutations(n) {
        let mapping: HashMap<Node, Node> = (1..=n).zip(perm.into_iter()).collect();
        let rel = relabel_unweighted(hg, &mapping);
        match &best {
            None => best = Some(rel),
            Some(b) => {
                if rel < *b {
                    best = Some(rel);
                }
            }
        }
    }
    best.expect("at least one permutation exists")
}

/// Fixed, blazingly fast connectivity check working directly on bitmasks
#[inline(always)]
fn is_mask_connected(mask: u32, edges: &[UnweightedEdge], n: usize) -> bool {
    // 1. Fast path: count how many nodes actually appear in this mask
    let mut node_seen = 0u8;
    let mut temp = mask;
    while temp != 0 {
        let idx = temp.trailing_zeros() as usize;
        temp &= temp - 1;
        for &node in &edges[idx] {
            node_seen |= 1 << (node - 1);
        }
    }
    // If it doesn't span exactly all n nodes, reject immediately
    if node_seen.count_ones() as usize != n {
        return false;
    }

    // 2. Compute components using standard BFS/DFS over bitset layers
    let mut adj = [0u8; 6]; // Quick adjacency bitset for up to N=5
    let mut temp = mask;
    while temp != 0 {
        let idx = temp.trailing_zeros() as usize;
        temp &= temp - 1;
        let edge = &edges[idx];
        for i in 0..edge.len() {
            for j in (i + 1)..edge.len() {
                adj[edge[i]] |= 1 << edge[j];
                adj[edge[j]] |= 1 << edge[i];
            }
        }
    }

    // Start flood fill from node 1
    let mut visited = 0u8;
    let mut q = VecDeque::with_capacity(6);
    q.push_back(1);
    visited |= 1 << 1;

    while let Some(u) = q.pop_front() {
        let mut neighbors = adj[u];
        while neighbors != 0 {
            let v = neighbors.trailing_zeros() as usize;
            neighbors &= neighbors - 1;
            if (visited & (1 << v)) == 0 {
                visited |= 1 << v;
                q.push_back(v);
            }
        }
    }

    // Check if all nodes 1..=n were visited
    let target_mask = ((1 << (n + 1)) - 1) & !1;
    (visited & target_mask) == target_mask
}

/// Decodes a flat integer bitmask back into your required UnweightedHypergraph vector structure
fn decode_mask(mut mask: u32, edges: &[UnweightedEdge]) -> UnweightedHypergraph {
    let mut hg = Vec::with_capacity(mask.count_ones() as usize);
    while mask != 0 {
        let idx = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        hg.push(edges[idx].clone());
    }
    hg.sort();
    hg
}

#[derive(Clone)]
pub struct Hyperedge {
    nodes: Vec<usize>,
}

impl Hyperedge {
    pub fn new(mut nodes: Vec<usize>) -> Self {
        nodes.sort_unstable();
        Self { nodes }
    }
}

impl std::fmt::Debug for Hyperedge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.nodes).as_str())
    }
}

impl PartialOrd for Hyperedge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.nodes.len() < other.nodes.len() {
            return Some(std::cmp::Ordering::Less);
        } else if self.nodes.len() > other.nodes.len() {
            return Some(std::cmp::Ordering::Greater);
        }
        self.nodes.partial_cmp(&other.nodes)
    }
}

impl Ord for Hyperedge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nodes.cmp(&other.nodes)
    }
}

impl PartialEq for Hyperedge {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl Eq for Hyperedge {}

impl Hash for Hyperedge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for node in &self.nodes {
            node.hash(state);
        }
    }
}

#[derive(Clone)]
pub struct Motif {
    edges: Vec<Hyperedge>,
}

impl std::fmt::Debug for Motif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.edges).as_str())
    }
}

impl Motif {
    pub fn new(mut edges: Vec<Hyperedge>) -> Self {
        edges.sort_unstable();
        Self { edges }
    }

    pub fn from_vec(edges: Vec<Vec<usize>>) -> Self {
        let edges = edges
            .into_iter()
            .map(|e| Hyperedge::new(e))
            .collect::<Vec<Hyperedge>>();
        Self::new(edges)
    }
}

impl PartialOrd for Motif {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.edges.partial_cmp(&other.edges)
    }
}

impl Ord for Motif {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.edges.cmp(&other.edges)
    }
}

impl PartialEq for Motif {
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

impl Eq for Motif {}

impl Hash for Motif {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for edge in &self.edges {
            edge.hash(state);
        }
    }
}

/// FIXED: Generates motifs using streams to prevent Out Of Memory crashes
pub fn generate_motifs(n: usize) -> HashMap<Motif, Vec<Motif>, FixedState> {
    assert!(
        n >= 2 && n <= 5,
        "This optimized pipeline supports up to N=5"
    );

    let nodes: Vec<Node> = (1..=n).collect();
    let mut all_edges: UnweightedHypergraph = Vec::new();
    for r in (2..=n).rev() {
        all_edges.extend(nodes.iter().copied().combinations(r));
    }
    all_edges.sort();

    let total_subgraphs = 1usize << all_edges.len();
    let mut canonical_rep: HashSet<UnweightedHypergraph> = HashSet::new();

    // Loop through the powerset combinations sequentially without holding them in RAM
    for mask in 0..total_subgraphs {
        if is_mask_connected(mask as u32, &all_edges, n) {
            let g = decode_mask(mask as u32, &all_edges);
            canonical_rep.insert(get_canonical_representative(&g, Some(n)));
        }
    }

    let mut rep_map: HashMap<Motif, Vec<Motif>, FixedState> =
        HashMap::with_hasher(FixedState::default());

    for representative in canonical_rep.into_iter() {
        let mut v = Vec::new();
        for iso in enum_isomorphisms(&representative, Some(n)) {
            let iso = iso
                .into_iter()
                .map(|e| {
                    let edge = e.into_iter().map(|n| n - 1).collect::<Vec<usize>>();
                    Hyperedge::new(edge)
                })
                .collect::<Vec<Hyperedge>>();

            let iso = Motif::new(iso.clone());
            v.push(iso.clone());
        }
        let representative = representative
            .into_iter()
            .map(|e| {
                let edge = e.into_iter().map(|n| n - 1).collect::<Vec<usize>>();
                Hyperedge::new(edge)
            })
            .collect::<Vec<Hyperedge>>();

        let representative = Motif::new(representative.clone());
        rep_map.insert(representative, v);
    }

    // let mut reps: Vec<UnweightedHypergraph> = canonical_rep.into_iter().collect();
    // reps.sort();

    rep_map
}
