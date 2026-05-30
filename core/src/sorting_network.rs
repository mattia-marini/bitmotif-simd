#![allow(dead_code)]

#[inline(always)]
fn cswap<T: Ord>(arr: &mut [T], i: usize, j: usize) {
    if arr[i] > arr[j] {
        arr.swap(i, j);
    }
}

pub trait SortingNetwork<T: Ord, const N: usize> {
    fn network_sort(&mut self);
    fn network_sort_slice(&mut self);
}

macro_rules! impl_network {
    ($n:expr, { $(($i:expr, $j:expr));* $(;)? }) => {
        impl<T: Ord> SortingNetwork<T, $n> for [T; $n] {
            #[inline(always)]
            fn network_sort(&mut self) { self.network_sort_slice(); }

            #[inline(always)]
            fn network_sort_slice(&mut self) {
                debug_assert!(self.len() == $n);
                $(
                    cswap(self, $i, $j);
                )*
            }
        }
    };
}

impl_network!(3, {
    (0, 1);
    (1, 2);
    (0, 1);
});

impl_network!(4, {
    (0, 1);
    (2, 3);
    (0, 2);
    (1, 3);
    (1, 2);
});

impl_network!(5, {
    (0, 1);
    (3, 4);
    (2, 4);
    (2, 3);
    (0, 3);
    (0, 2);
    (1, 4);
    (1, 3);
    (1, 2);
});

impl_network!(6, {
    (1, 2);
    (4, 5);
    (0, 2);
    (3, 5);
    (0, 1);
    (3, 4);
    (2, 5);
    (0, 3);
    (1, 4);
    (2, 4);
    (1, 3);
    (2, 3);
});

impl_network!(7, {
    (0, 1);
    (2, 3);
    (4, 5);
    (0, 2);
    (1, 3);
    (1, 2);
    (4, 6);
    (5, 6);
    (0, 4);
    (1, 5);
    (2, 6);
    (1, 4);
    (3, 6);
    (2, 4);
    (3, 5);
    (3, 4);
});

impl_network!(8, {
    (0, 1);
    (2, 3);
    (4, 5);
    (6, 7);
    (0, 2);
    (1, 3);
    (4, 6);
    (5, 7);
    (1, 2);
    (5, 6);
    (0, 4);
    (3, 7);
    (1, 5);
    (2, 6);
    (2, 4);
    (3, 5);
    (1, 2);
    (5, 6);
    (3, 4);
});

impl_network!(9, {
    (0, 1);
    (3, 4);
    (6, 7);
    (1, 2);
    (4, 5);
    (7, 8);
    (0, 1);
    (3, 4);
    (6, 7);
    (0, 3);
    (1, 4);
    (2, 5);
    (3, 6);
    (4, 7);
    (5, 8);
    (0, 3);
    (1, 4);
    (2, 5);
    (1, 6);
    (2, 7);
    (1, 3);
    (4, 6);
    (5, 7);
    (2, 4);
    (3, 5);
    (2, 3);
    (4, 5);
});

impl_network!(10, {
    (0, 1);
    (2, 3);
    (4, 5);
    (6, 7);
    (8, 9);
    (0, 2);
    (1, 3);
    (4, 6);
    (5, 7);
    (0, 4);
    (1, 5);
    (2, 6);
    (3, 7);
    (0, 8);
    (1, 9);
    (2, 8);
    (3, 9);
    (1, 4);
    (3, 6);
    (5, 8);
    (1, 2);
    (4, 5);
    (7, 8);
    (3, 5);
    (6, 8);
    (2, 4);
    (5, 7);
    (3, 4);
    (5, 6);
});

// const NETWORK_FUNCTIONS: = [
// ]

pub fn sort_network<T: Ord>(v: &mut [T]) {
    match v.len() {
        0..=10 => {
            // SAFETY: The length of the array is guaranteed to be correct by the match arm.
            unsafe {
                let arr = v.as_mut_ptr().cast::<[T; 10]>();
                (*arr).network_sort_slice();
            }
        }
        _ => {
            v.sort_unstable();
        }
    }
}
