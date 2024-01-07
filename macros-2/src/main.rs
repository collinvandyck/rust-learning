fn main() {
    say_hello!();
    say_goodbye!();
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
