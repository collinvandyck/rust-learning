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
    }
}
