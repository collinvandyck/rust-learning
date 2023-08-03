fn main() {}

#[cfg(test)]
mod tests {
    use sorting_1::sorters::BubbleSort;
    use sorting_1::Sorter;

    #[test]
    fn test_sorts() {
        test_harness("bubblesort", BubbleSort::sort);
    }

    fn test_harness<F>(name: &str, f: F)
    where
        F: Fn(&mut [i32]),
    {
        println!("Testing {name}");
        let mut nums = [1, 5, 3, 4, 2];
        f(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }
}
