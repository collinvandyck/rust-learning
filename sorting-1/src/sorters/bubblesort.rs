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
