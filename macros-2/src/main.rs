fn main() {
    pdbg!("foobar");
    pdbg!(vec![1, 2, 3]);

    foo();
    bar();

    let x = 5;
    print_res!(x);
    print_res!(x);

    print_res!({
        let four = 4;
        let foo = || -> i32 { 3 + four };
        pdbg!(format!("foobar: {}", foo()))
    });

    test!(true; and false);
    test!(true; and true);
    test!(true; or false);
    test!(true; or true);
    test!(false; or false);

    print_res!(find_min!(1));
    print_res!(find_min!(1, 2));
    print_res!(find_min!(3, 2, 1));
}

new_func!(foo);
new_func!(bar);

#[macro_export]
macro_rules! find_min {
    ($x:expr) => {
        ($x)
    };
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}

#[macro_export]
macro_rules! test {
    ($lt:expr; and $rt:expr) => {
        println!("{:?} && {:?} == {:?}", $lt, $rt, $lt && $rt);
    };
    ($lt:expr; or $rt:expr) => {
        println!("{:?} || {:?} == {:?}", $lt, $rt, $lt || $rt);
    };
}

#[macro_export]
macro_rules! print_res {
    ($res:expr) => {
        println!("{} = {:#?}", stringify!($res), $res);
    };
}

#[macro_export]
macro_rules! new_func {
    ($name:ident) => {
        fn $name() {
            println!("{} called.", stringify!($name));
        }
    };
}

#[macro_export]
macro_rules! pdbg {
    ($v:expr) => {{
        println!("{:#?}", $v);
        $v
    }};
}

#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

#[macro_export]
macro_rules! say_goodbye {
    () => {
        println!("Goodbye!");
    };
}
