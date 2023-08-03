fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::thread_rng;
    use rand::Rng;
    use sorting_1::sorters::*;
    use sorting_1::Sorter;

    #[test]
    fn test_sorts() {
        test_sorter::<BubbleSort>();
        test_sorter::<MergeSort>();
        test_sorter::<InsertionSort>();
    }

    fn test_sorter<S>()
    where
        S: Sorter,
    {
        let rng = thread_rng();
        let mut nums = rng
            .sample_iter(&rand::distributions::Standard)
            .take(1_000)
            .collect::<Vec<i32>>();
        let mut sorted = nums.clone();
        sorted.sort();
        let name = S::name();
        println!("Testing {name}");
        let start = Instant::now();
        S::sort(&mut nums);
        let end = Instant::now();
        let dur = end.duration_since(start);
        println!("Dur: {}", dur.as_micros());
        assert_eq!(nums, sorted);
    }
}
