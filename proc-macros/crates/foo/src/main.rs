// a trait we define here but would be defined closer to the derive macro in practice
trait Foo {
    fn foo(&self) -> String;
}

// deriving Foo implements Foo for the struct and also adds methods for each field.
#[derive(foo_derive::Foo)]
struct Person {
    name: &'static str,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Collin",
            age: 39,
        }
    }
}

fn main() {
    // just a generic person (me)
    let p = Person::default();

    // silly method added by the derive
    p.say_hello();

    // methods generated for each field in the struct
    p.debug_name();
    p.debug_age();

    // the derive implements Foo for the thing derived
    let p: &dyn Foo = &p;
    println!("Foo: {}", p.foo());
}
