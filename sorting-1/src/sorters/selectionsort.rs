use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn name() -> &'static str {
        "selectionsort"
    }
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let mut small_idx = i;
            for i2 in (i + 1)..slice.len() {
                if slice[i2] < slice[small_idx] {
                    small_idx = i2;
                }
            }
            if small_idx != i {
                slice.swap(i, small_idx);
            }
        }
    }
}
