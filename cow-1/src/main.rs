use std::borrow::Cow;

fn main() {
    let b = Bar;
    println!("{}", b.hi());
}

#[derive(Clone)]
struct Bar;

trait Foo {
    fn hi(self) -> &'static str;
}

impl<'a, B> Foo for B
where
    B: Into<Cow<'a, Bar>>,
{
    fn hi(self) -> &'static str {
        let bar = self.into();

        // bar is either owned or borrowed:
        match bar {
            Cow::Owned(_) => "Owned",
            Cow::Borrowed(_) => "Borrowed",
        }
    }
}

/* Into<Cow> implementation */

impl<'a> From<Bar> for Cow<'a, Bar> {
    fn from(f: Bar) -> Cow<'a, Bar> {
        Cow::Owned(f)
    }
}

impl<'a> From<&'a Bar> for Cow<'a, Bar> {
    fn from(f: &'a Bar) -> Cow<'a, Bar> {
        Cow::Borrowed(f)
    }
}

