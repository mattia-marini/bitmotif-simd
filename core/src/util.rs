#![allow(unused)]

use std::{fs, path::Path};

use hashbrown::HashMap;

use crate::{compressed_motif::CompactMotif, fingerprint::Fingerprint5};

#[macro_export]
macro_rules! iter_hyperedges {
    ($node_count: expr, $range: expr , |$edge:ident, $edge_size:ident, $edge_idx:ident| $body:block ) => {{
        let min: usize = *$range.start();
        let max: usize = *$range.end();
        assert!(min < max);
        assert!(max <= $node_count);

        let mut target_size = min;
        let mut edge_idx = 0;
        while target_size <= max {
            let mut curr_edge: [usize; $node_count] = [0; $node_count];
            let mut positions: [usize; $node_count] = [0; $node_count];
            let mut stack_size = 1;

            while stack_size > 0 {
                let stack_index = stack_size - 1;

                if stack_index == target_size {
                    let $edge = curr_edge;
                    let $edge_size = target_size;
                    let $edge_idx = edge_idx;

                    $body;
                    edge_idx += 1;
                    stack_size -= 1;
                } else {
                    if positions[stack_index] < $node_count - target_size + stack_size {
                        curr_edge[stack_index] = positions[stack_index];
                        positions[stack_index] += 1;

                        if stack_index + 1 < $node_count {
                            positions[stack_index + 1] = positions[stack_index];
                        }

                        stack_size += 1;
                    } else {
                        stack_size -= 1;
                    }
                }
            }
            target_size += 1;
        }
    }};
}

/// A constant function to calculate factorial for the M parameter
pub const fn factorial(n: usize) -> usize {
    let mut res = 1;
    let mut i = 1;
    while i <= n {
        res *= i;
        i += 1;
    }
    res
}

/// Computes the binomial coefficient (n choose m) at compile time.
pub const fn binomial_coefficient(n: usize, mut m: usize) -> usize {
    if m > n {
        return 0;
    }
    if m == 0 || m == n {
        return 1;
    }

    // Optimize using the symmetry property: (n choose m) == (n choose n - m)
    if m > n - m {
        m = n - m;
    }

    let mut res = 1;
    let mut i = 0;

    while i < m {
        // Safe from precision loss because the product of `i + 1`
        // consecutive integers is always divisible by `(i + 1)!`
        res = (res * (n - i)) / (i + 1);
        i += 1;
    }

    res
}

pub const fn max_hyperedge_count(
    node_count: usize,
    min_edge_size: usize,
    max_edge_size: usize,
) -> usize {
    let mut total = 0;
    let mut edge_size = min_edge_size;
    while edge_size <= max_edge_size {
        total += binomial_coefficient(node_count, edge_size);
        edge_size += 1;
    }
    total
}

const fn iota<const N: usize>() -> [usize; N] {
    let mut arr = [0; N];
    let mut i = 0;
    while i < N {
        arr[i] = i;
        i += 1;
    }
    arr
}

const fn generate_permutations<const N: usize, const M: usize>(
    arr: &mut [u8; N],
    k: usize,
    out: &mut [[u8; N]; M],
    count: &mut usize,
) {
    if k == 1 {
        out[*count] = *arr;
        *count += 1;
        return;
    }

    let mut i = 0;
    while i < k {
        generate_permutations(arr, k - 1, out, count);

        if k % 2 == 0 {
            // Swap i and k-1
            let tmp = arr[i];
            arr[i] = arr[k - 1];
            arr[k - 1] = tmp;
        } else {
            // Swap 0 and k-1
            let tmp = arr[0];
            arr[0] = arr[k - 1];
            arr[k - 1] = tmp;
        }
        i += 1;
    }
}

macro_rules! define_permutator {
    ($n:literal, $m:literal) => {
        impl Permutator<$n> {
            pub const fn get_permutations(v: [u8; $n]) -> [[u8; $n]; $m] {
                let mut permutations = [[0; $n]; $m];
                let mut current = v;
                let mut count = 0;
                generate_permutations(&mut current, $n, &mut permutations, &mut count);
                permutations
            }
        }
    };
}

define_permutator!(2, 2);
define_permutator!(3, 6);
define_permutator!(4, 24);
define_permutator!(5, 120);

pub struct BinPerm {
    pub container: usize,
}

pub struct BinPermIterator {
    container: usize,
    n: usize,
}

impl Iterator for BinPermIterator {
    type Item = BinPerm;

    fn next(&mut self) -> Option<Self::Item> {
        if self.container < factorial(self.n) {
            let perm = BinPerm {
                container: self.container,
            };
            self.container += 1;
            Some(perm)
        } else {
            None
        }
    }
}

impl From<usize> for BinPerm {
    fn from(value: usize) -> Self {
        Self { container: value }
    }
}

impl BinPerm {
    pub const fn new() -> Self {
        Self { container: 0 }
    }

    pub const fn from_usize(container: usize) -> Self {
        Self { container }
    }

    pub const fn encode<const N: usize>(mut arr: [u8; N]) -> Self {
        let mut inv = [0; N];
        let mut i = 0;
        while i < N {
            inv[arr[i] as usize] = i as u8;
            i += 1;
        }

        let mut res = 0;
        let mut base = 1;
        let mut n = N;

        while n > 1 {
            let s = arr[n - 1] as usize;
            res += s * base;
            base *= n;

            let j = inv[n - 1] as usize;

            let temp_arr = arr[n - 1];
            arr[n - 1] = arr[j];
            arr[j] = temp_arr;

            let temp_inv = inv[s];
            inv[s] = inv[n - 1];
            inv[n - 1] = temp_inv;

            n -= 1;
        }

        Self { container: res }
    }

    pub const fn decode<const N: usize>(&self) -> [u8; N] {
        let mut num = self.container;

        let mut res = [0; N];
        let mut i = 0;
        while i < N {
            res[i] = i as u8;
            i += 1;
        }

        let mut n = N;
        while n > 1 {
            let swap_idx = num % n;
            num /= n;

            res.swap(n - 1, swap_idx);
            n -= 1;
        }

        res
    }

    pub const fn iter_all<const N: usize>() -> BinPermIterator {
        BinPermIterator { container: 0, n: N }
    }
}

pub struct Permutator<const N: usize> {}

struct FlatBinData<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> FlatBinData<N> {
    pub fn new() -> Self {
        Self { data: [0; N] }
    }

    pub fn from_array(arr: [u8; N]) -> Self {
        Self { data: arr }
    }

    pub fn set_range(&mut self, start: usize, value: u8) {
        let bit_offset = start % 8;
        self.data[start / 8] &= (1 << bit_offset) - 1;
        self.data[start / 8] |= value << bit_offset;

        self.data[start / 8 + 1] &= 0xFF << bit_offset;
        self.data[start / 8 + 1] |= value >> (8 - bit_offset);
    }
}

const CACHE_DIR: &str = "cache";

pub fn save_to_file(
    v: HashMap<Fingerprint5, Vec<CompactMotif<5>>>,
    file_name: &str,
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

    let file_path = path.join(file_name);
    fs::write(file_path, bytes)?;

    Ok(())
}

pub fn load_from_file(
    file_name: &str,
) -> Result<HashMap<Fingerprint5, Vec<CompactMotif<5>>>, Box<dyn std::error::Error>> {
    let file_path = Path::new(CACHE_DIR).join(file_name);

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
