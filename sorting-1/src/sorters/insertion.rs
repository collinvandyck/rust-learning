use crate::Sorter;

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
