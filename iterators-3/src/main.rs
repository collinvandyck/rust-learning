fn main() {
    println!("Hello, world!");
    let mut nums = vec![1, 2, 3];
    nums.windows(2).for_each(|x| {
        dbg!(x);
    });
    nums.windows(1).for_each(|x| {
        dbg!(x);
    });

    nums.swap(0, 1);
    nums = dbg!(nums);

    nums.fill(0);
    dbg!(nums);
}
