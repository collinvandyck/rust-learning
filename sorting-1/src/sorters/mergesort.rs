use std::fmt::Debug;

use crate::Sorter;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn name() -> &'static str {
        "mergesort"
    }
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy + Debug,
    {
        let mut buf = slice.to_vec();
        Self::do_sort(slice, &mut buf);
    }
}

impl MergeSort {
    fn do_sort<T>(slice: &mut [T], buf: &mut [T])
    where
        T: Ord + Copy + Debug,
    {
        if slice.len() <= 1 {
            return;
        }
        let mid = slice.len() / 2;
        Self::do_sort(&mut slice[..mid], buf);
        Self::do_sort(&mut slice[mid..], buf);
        let mut s = 0;
        let mut i = 0;
        let mut j = mid;
        while i < mid && j < slice.len() {
            buf[s] = if slice[j] < slice[i] {
                j += 1;
                slice[j - 1]
            } else {
                i += 1;
                slice[i - 1]
            };
            s += 1;
        }
        let (start, end) = if i < mid { (i, mid) } else { (j, slice.len()) };
        for x in start..end {
            buf[s] = slice[x];
            s += 1;
        }
        slice.copy_from_slice(&buf[..slice.len()]);
    }
}
