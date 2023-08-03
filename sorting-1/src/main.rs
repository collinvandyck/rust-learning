fn main() {}

#[cfg(test)]
mod tests {
    use sorting_1::sorters::*;
    use sorting_1::Sorter;

    #[test]
    fn test_sorts() {
        test_sorter::<BubbleSort>();
        test_sorter::<MergeSort>();
    }

    fn test_sorter<S>()
    where
        S: Sorter,
    {
        let name = S::name();
        println!("Testing {name}");
        let mut nums = [1, 5, 3, 2, 4];
        S::sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }
}
