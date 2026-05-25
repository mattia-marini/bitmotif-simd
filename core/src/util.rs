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
