#![allow(unused)]

//! See: https://developerlife.com/2024/07/16/pin-box-dynamic-duo/
//!

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

    use crate::assert_three_equal;

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

        let x_addr_3 = print_ptr_addr_size!(&x);
        println!(
            "x: {}, x_addr_3: {}",
            x.to_string().blue().underlined(),
            x_addr_3.red().italic().on_black()
        );
    }

    #[test]
    #[serial]
    fn move_a_box() {
        let b_1 = Box::new(255u8);
        let b_1_addr = print_ptr_addr_size!(b_1.as_ref()); // pointee (heap)
        let b_1_ptr_addr = print_ptr_addr_size!(&b_1); // pointer (stack)
        println!(
            "1. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_1".green(),
            b_1.to_string().blue().underlined(),
            "b_1_addr".green(),
            b_1_addr.clone().magenta().italic().on_black(),
            "b_1_ptr_addr".green(),
            b_1_ptr_addr.clone().magenta().italic().on_black()
        );

        let b_2 = b_1;
        let b_2_addr = print_ptr_addr_size!(b_2.as_ref()); // pointee (heap)
        let b_2_ptr_addr = print_ptr_addr_size!(&b_2); // pointer (stack)
        println!(
            "2. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_2".green(),
            b_2.to_string().blue().underlined(),
            "b_2_addr".green(),
            b_2_addr.clone().magenta().italic().on_black(),
            "b_2_ptr_addr".green(),
            b_2_ptr_addr.clone().magenta().italic().on_black()
        );

        // the heap memory allocation does not change/move
        assert_eq!(b_1_addr, b_2_addr);
        // the stack address does change (boxes aka pointers have move)
        assert_ne!(b_1_ptr_addr, b_2_ptr_addr);
    }

    #[test]
    #[serial]
    fn swap_box_contents() {
        let mut b_1 = Box::new(100u8);
        let mut b_2 = Box::new(200u8);

        let og_b1_addr = print_ptr_addr_size!(b_1.as_ref());
        let og_b2_addr = print_ptr_addr_size!(b_2.as_ref());

        assert_eq!(*b_1, 100u8);
        assert_eq!(*b_2, 200u8);

        println!(
            "1. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_1".green(),
            b_1.to_string().blue().underlined(),
            "b_1_addr".green(),
            og_b1_addr.clone().red().italic().on_black(),
            "b_1_ptr_addr".green(),
            print_ptr_addr_size!(&b_1)
                .clone()
                .magenta()
                .italic()
                .on_black()
        );
        println!(
            "2. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_2".green(),
            b_2.to_string().blue().underlined(),
            "b_2_addr".green(),
            og_b2_addr.clone().red().italic().on_black(),
            "b_2_ptr_addr".green(),
            print_ptr_addr_size!(&b_2)
                .clone()
                .magenta()
                .italic()
                .on_black()
        );

        std::mem::swap(&mut b_1, &mut b_2);
        println!("{}", "Swapped b_1 and b_2".cyan().underlined());

        let new_b1_addr = print_ptr_addr_size!(b_1.as_ref());
        let new_b2_addr = print_ptr_addr_size!(b_2.as_ref());

        assert_eq!(*b_1, 200u8);
        assert_eq!(*b_2, 100u8);
        assert_eq!(og_b1_addr, new_b2_addr);
        assert_eq!(og_b2_addr, new_b1_addr);

        println!(
            "3. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_1".green(),
            b_1.to_string().blue().underlined(),
            "b_1_addr".green(),
            new_b1_addr.clone().red().italic().on_black(),
            "b_1_ptr_addr".green(),
            print_ptr_addr_size!(&b_1)
                .clone()
                .magenta()
                .italic()
                .on_black()
        );
        println!(
            "4. {}: {}, {} (pointee, heap): {}, {} (ptr, stack): {}",
            "b_2".green(),
            b_2.to_string().blue().underlined(),
            "b_2_addr".green(),
            new_b2_addr.clone().red().italic().on_black(),
            "b_2_ptr_addr".green(),
            print_ptr_addr_size!(&b_2)
                .clone()
                .magenta()
                .italic()
                .on_black()
        );
    }

    #[test]
    #[serial]
    fn box_and_pin_dynamic_duo() {
        let b_1 = Box::new(100u8);
        // pointee
        let b_1_addr = print_ptr_addr_size!(b_1.as_ref());
        println!("b_1_addr:   {b_1_addr}");
        let p_b_1 = std::boxed::Box::<u8>::into_pin(b_1);
        // pinned
        let p_b_1_addr = print_pin_addr_size!(p_b_1.as_ref());
        println!("p_b_1_addr: {p_b_1_addr}");

        let b_2 = p_b_1;
        let b_2_addr = print_pin_addr_size!(b_2);
        println!("b_2_addr:   {b_2_addr}");

        // pointee has not moved!
        assert_eq!(b_2_addr, b_1_addr);
        // pointer has not moved!
        assert_three_equal(&b_1_addr, &p_b_1_addr, &b_2_addr);
    }
}
