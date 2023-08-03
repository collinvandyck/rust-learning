fn main() {}

#[cfg(test)]
mod tests {
    use sorting_1::sorters::BubbleSort;
    use sorting_1::Sorter;

    #[test]
    fn test_bubblesort() {
        let mut nums = [1, 5, 3, 4, 2];
        BubbleSort::sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }
}
