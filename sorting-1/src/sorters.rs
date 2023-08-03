use std::fmt::Debug;

use crate::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn name() -> &'static str {
        "bubblesort"
    }
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            for j in 0..slice.len() - 1 {
                if slice[j] > slice[i] {
                    slice.swap(i, j);
                }
            }
        }
    }
}

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn name() -> &'static str {
        "insertionsort"
    }
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}

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
