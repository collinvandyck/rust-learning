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
            let x = slice[i..slice.len()].iter().enumerate().min_by_key(|f| f.1);
            if let Some((idx, _)) = x {
                let idx = idx + i;
                slice.swap(i, idx);
            }
        }
    }
}

#[test]
fn test_selection_sort() {
    let mut nums = [3, 5];
    SelectionSort::sort(&mut nums);
    assert_eq!(nums, [3, 5]);
}
