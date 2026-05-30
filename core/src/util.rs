#![allow(unused)]

#[inline(always)]
pub fn sort4<T>(arr: &mut [T; 4])
where
    T: PartialOrd,
{
    // Sorting network for 4 inputs (5 comparator)

    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    if arr[2] > arr[3] {
        arr.swap(2, 3);
    }

    if arr[0] > arr[2] {
        arr.swap(0, 2);
    }
    if arr[1] > arr[3] {
        arr.swap(1, 3);
    }

    if arr[1] > arr[2] {
        arr.swap(1, 2);
    }
}

pub fn sort_slice4<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    if arr[2] > arr[3] {
        arr.swap(2, 3);
    }

    if arr[0] > arr[2] {
        arr.swap(0, 2);
    }
    if arr[1] > arr[3] {
        arr.swap(1, 3);
    }

    if arr[1] > arr[2] {
        arr.swap(1, 2);
    }
}

#[inline(always)]
pub fn sort5<T>(arr: &mut [T; 5])
where
    T: PartialOrd,
{
    // Sorting network for 5 inputs (9 comparator)

    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    if arr[3] > arr[4] {
        arr.swap(3, 4);
    }

    if arr[2] > arr[4] {
        arr.swap(2, 4);
    }
    if arr[2] > arr[3] {
        arr.swap(2, 3);
    }

    if arr[0] > arr[3] {
        arr.swap(0, 3);
    }
    if arr[0] > arr[2] {
        arr.swap(0, 2);
    }

    if arr[1] > arr[4] {
        arr.swap(1, 4);
    }
    if arr[1] > arr[3] {
        arr.swap(1, 3);
    }

    if arr[1] > arr[2] {
        arr.swap(1, 2);
    }
}

#[inline(always)]
pub fn sort10<T>(arr: &mut [T; 10])
where
    T: PartialOrd,
{
    // Optimal sorting network for 10 inputs (29 comparators)

    // Layer 1
    if arr[0] > arr[1] {
        arr.swap(0, 1);
    }
    if arr[2] > arr[3] {
        arr.swap(2, 3);
    }
    if arr[4] > arr[5] {
        arr.swap(4, 5);
    }
    if arr[6] > arr[7] {
        arr.swap(6, 7);
    }
    if arr[8] > arr[9] {
        arr.swap(8, 9);
    }

    // Layer 2
    if arr[0] > arr[2] {
        arr.swap(0, 2);
    }
    if arr[1] > arr[3] {
        arr.swap(1, 3);
    }
    if arr[4] > arr[6] {
        arr.swap(4, 6);
    }
    if arr[5] > arr[7] {
        arr.swap(5, 7);
    }

    // Layer 3
    if arr[0] > arr[4] {
        arr.swap(0, 4);
    }
    if arr[1] > arr[5] {
        arr.swap(1, 5);
    }
    if arr[2] > arr[6] {
        arr.swap(2, 6);
    }
    if arr[3] > arr[7] {
        arr.swap(3, 7);
    }

    // Layer 4
    if arr[0] > arr[8] {
        arr.swap(0, 8);
    }
    if arr[1] > arr[9] {
        arr.swap(1, 9);
    }

    // Layer 5
    if arr[2] > arr[8] {
        arr.swap(2, 8);
    }
    if arr[3] > arr[9] {
        arr.swap(3, 9);
    }

    // Layer 6
    if arr[1] > arr[4] {
        arr.swap(1, 4);
    }
    if arr[3] > arr[6] {
        arr.swap(3, 6);
    }
    if arr[5] > arr[8] {
        arr.swap(5, 8);
    }

    // Layer 7
    if arr[1] > arr[2] {
        arr.swap(1, 2);
    }
    if arr[4] > arr[5] {
        arr.swap(4, 5);
    }
    if arr[6] > arr[8] {
        arr.swap(6, 8);
    }

    // Layer 8
    if arr[2] > arr[4] {
        arr.swap(2, 4);
    }
    if arr[3] > arr[5] {
        arr.swap(3, 5);
    }
    if arr[7] > arr[9] {
        arr.swap(7, 9);
    }

    // Layer 9
    if arr[3] > arr[4] {
        arr.swap(3, 4);
    }
    if arr[5] > arr[6] {
        arr.swap(5, 6);
    }

    // Layer 10
    if arr[6] > arr[7] {
        arr.swap(6, 7);
    }
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
