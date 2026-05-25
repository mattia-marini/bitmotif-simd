#[inline(always)]
pub fn sort4<T>(arr: &mut [T; 4])
where
    T: PartialOrd,
{
    // Sorting network for 4 inputs

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
    // Sorting network for 5 inputs

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

/// A constant function to calculate factorial for the M parameter
const fn factorial(n: usize) -> usize {
    let mut res = 1;
    let mut i = 1;
    while i <= n {
        res *= i;
        i += 1;
    }
    res
}

const fn generate_permutations<const N: usize, const M: usize>(
    arr: &mut [u8; N],
    k: usize,
    out: &mut <[u8; N] as Permutable>::Output,
    count: &mut usize,
) where
    [u8; N]: Permutable<Output = [[u8; N]; M]>,
{
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

pub trait Permutable {
    type Output;
    const OUTPUT_SIZE: usize;
}

impl Permutable for [u8; 2] {
    const OUTPUT_SIZE: usize = factorial(2);
    type Output = [[u8; 2]; 2];
}
impl Permutable for [u8; 3] {
    const OUTPUT_SIZE: usize = factorial(3);
    type Output = [[u8; 3]; 6];
}
impl Permutable for [u8; 4] {
    const OUTPUT_SIZE: usize = factorial(4);
    type Output = [[u8; 4]; 24];
}
impl Permutable for [u8; 5] {
    const OUTPUT_SIZE: usize = factorial(5);
    type Output = [[u8; 5]; 120];
}

pub const fn get_permutations<const N: usize, const M: usize>(
    v: [u8; N],
) -> <[u8; N] as Permutable>::Output
where
    [u8; N]: Permutable<Output = [[u8; N]; M]>,
{
    // let mut permutations: <[u8; N] as Permutable>::Output;
    let mut permutations = [[0; N]; M];
    let mut current = v;
    let mut count = 0;
    generate_permutations(&mut current, N, &mut permutations, &mut count);
    permutations
}
