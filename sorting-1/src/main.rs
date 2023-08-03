fn main() {}

#[cfg(test)]
mod tests {
    const NUMS: usize = 7_000;
    use std::time::Instant;

    use rand::thread_rng;
    use rand::Rng;
    use sorting_1::sorters::*;
    use sorting_1::Sorter;

    struct Res {
        name: &'static str,
        dur: u128,
    }

    #[test]
    fn test_sorts() {
        let mut res = vec![];
        res.push(test_sorter::<BubbleSort>());
        res.push(test_sorter::<MergeSort>());
        res.push(test_sorter::<InsertionSort>());
        res.sort_by(|a, b| a.dur.cmp(&b.dur));
        for r in res {
            println!("{}:\t{}us", r.name, r.dur);
        }
    }

    fn test_sorter<S>() -> Res
    where
        S: Sorter,
    {
        let rng = thread_rng();
        let mut nums = rng
            .sample_iter(&rand::distributions::Standard)
            .take(NUMS)
            .collect::<Vec<i32>>();
        let mut sorted = nums.clone();
        sorted.sort();
        let name = S::name();
        let start = Instant::now();
        S::sort(&mut nums);
        let end = Instant::now();
        let dur = end.duration_since(start);
        assert_eq!(nums, sorted, "{} failed to sort", name);
        Res {
            name,
            dur: dur.as_micros(),
        }
    }
}
