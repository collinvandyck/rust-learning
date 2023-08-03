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
        Self::do_sort(slice, &mut buf, 0);
    }
}

impl MergeSort {
    fn do_sort<T>(slice: &mut [T], buf: &mut [T], depth: usize)
    where
        T: Ord + Copy + Debug,
    {
        if slice.len() <= 1 {
            return;
        }
        if slice.len() == 2 {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        let mid = slice.len() / 2;
        Self::do_sort(&mut slice[..mid], buf, depth + 1);
        Self::do_sort(&mut slice[mid..], buf, depth + 1);
        let mut h = 0;
        let mut i = 0;
        let mut j = mid;
        while i < mid && j < slice.len() {
            buf[h] = if slice[j] < slice[i] {
                j += 1;
                slice[j - 1]
            } else {
                i += 1;
                slice[i - 1]
            };
            h += 1;
        }
        if i < mid {
            for x in i..mid {
                buf[h] = slice[x];
                h += 1;
            }
        } else if j < slice.len() {
            for x in j..slice.len() {
                buf[h] = slice[x];
                h += 1;
            }
        }
        slice.copy_from_slice(&buf[..slice.len()]);
    }
}
