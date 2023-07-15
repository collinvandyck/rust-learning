pub enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    /// noop
    /// addx 3
    /// addx -5
    pub fn parse(line: String) -> Self {
        Self::Noop
    }
}
