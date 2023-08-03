pub mod sorters;

pub trait Sorter {
    fn name() -> &'static str;
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Copy;
}
