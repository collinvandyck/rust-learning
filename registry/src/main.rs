#![allow(unused)]

fn main() {
    println!("{:?}", Reg.bar());
}

pub struct Reg;

#[derive(Debug)]
pub struct Location {
    path: &'static str,
    key: &'static str,
}

macro_rules! generate {
    (
        $path:expr => {
            $(
                ($key:expr, $meth:ident)
            ),*
        }
    ) => {
        impl Reg {
        $(
            pub fn $meth(&self) -> Location {
                Location {
                    path: $path, key: $key
                }
            }
        )*
        }
    };
}

generate! {
    "PATH" => {
        ("bar", bar),
        ("foo", foo)
    }
}
