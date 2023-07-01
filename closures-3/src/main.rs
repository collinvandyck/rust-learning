fn main() {
    // closure here is a Fn
    call_twice(|| println!("Hello, world!"));

    /*
     * Note: this fails because f is a FnOnce
    let my_str = "Hello".to_string();
    let f = || drop(my_str);
    call_twice(f);
    */

    // FnMut with no args

    let mut i = 0;
    let mut incr = || {
        i += 1;
        println!("Ding! i is now: {}", i)
    };
    call_twice_mut(&mut incr);
    call_twice_mut(&mut incr);

    // FnMut with one arg

    let mut i = 0;
    let mut incr = |v: i32| {
        i += v;
        println!("Dong! i is now: {}", i)
    };
    call_twice_mut_with_arg(5, &mut incr);

    // using an inline mutable closure instead of predefining it.

    call_twice_mut_with_arg(5, &mut |v| {
        i += v;
        println!("Dang! i is now: {}", i)
    });

    let mut i = 0;
    another_mut_once_example(|| {
        i += 1;
        println!("hi: {}", i);
    });

    let mut i = 0;
    let func = || {
        i += 5;
        println!("wat: {}", i);
    };
    another_mut_once_example(func);
}

fn another_mut_once_example<F>(mut closure: F)
where
    F: FnMut(),
{
    for _ in 1..=5 {
        closure();
    }
}

fn call_twice_mut_with_arg<F>(arg: i32, closure: &mut F)
where
    F: FnMut(i32) -> (),
{
    closure(arg);
    closure(arg);
}

fn call_twice_mut<F>(closure: &mut F)
where
    F: FnMut(),
{
    closure();
    closure();
}
fn call_twice<F>(closure: F)
where
    F: Fn(),
{
    closure();
    closure();
}
