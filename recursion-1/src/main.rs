use recursion::{Expandable, MappableFrame};

fn main() {
    println!("Hello, world!");
}

pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    LiteralInt(i64),
}

pub enum ExprFrame<A> {
    Add(A, A),
    Sub(A, A),
    Mul(A, A),
    LiteralInt(A),
}

impl Expandable for Expr {}

impl MappableFrame for ExprFrame<Expr> {
    type Frame<X> = ExprFrame<X>;
    fn map_frame<A, B>(input: Self::Frame<A>, mut f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            ExprFrame::Add(a1, a2) => ExprFrame::Add(f(a1), f(a2)),
            ExprFrame::Sub(a1, a2) => ExprFrame::Sub(f(a1), f(a2)),
            ExprFrame::Mul(a1, a2) => ExprFrame::Mul(f(a1), f(a2)),
            ExprFrame::LiteralInt(a1) => ExprFrame::LiteralInt(f(a1)),
        }
    }
}
