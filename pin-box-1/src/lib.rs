#![allow(unused)]

use crossterm::style::Stylize;

/// given a pointer `$p` it prints:
///
/// 1. its address
/// 2. size of the thing it points to
macro_rules! print_ptr_addr_size {
    ($p:expr) => {
        format!("{:p}|{}b", $p, std::mem::size_of_val($p))
    };
}

/// given a pinned pointer `$p` it prints:
///
/// 1. its address
/// 2. size of the thing it points to
macro_rules! print_pin_addr_size {
    ($p:expr) => {
        format!("{:p}|{}b", $p, std::mem::size_of_val(&(*$p)))
    };
}

fn assert_three_equal<T: PartialEq + std::fmt::Debug>(a: &T, b: &T, c: &T) {
    assert_eq!(a, b, "a and b are not equal");
    assert_eq!(a, c, "a and c are not equal");
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crossterm::style::Stylize;
    use serial_test::serial;

    #[test]
    #[serial]
    fn print_ptr_addr_size() {
        let x = 100u8;
        let x_addr = ptr::addr_of!(x);
        println!(
            "x: {}, x_addr:   {}",
            x.to_string().blue().underlined(),
            format!("{:?}", x_addr).red().italic(),
        );

        let x_addr_2 = format!("{:p}", &x);
        println!(
            "x: {}, x_addr_2: {}",
            x.to_string().blue().underlined(),
            x_addr_2.red().italic().on_black()
        );

        let x_size = std::mem::size_of_val(&x);
        println!(
            "x: {}, x_size:   {}b",
            x.to_string().blue().underlined(),
            x_size.to_string().magenta().italic().on_black()
        );
    }
}
