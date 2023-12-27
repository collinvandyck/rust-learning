use itertools::Itertools;

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", summarize_patterns(ex1, false));
    println!("p1in1 = {}", summarize_patterns(in1, false));
}

fn summarize_patterns(input: &str, smudges: bool) -> usize {
    if smudges {
        let pats = parse(input);
        let vs = pats.iter().map(|p| (p, p.mirrors())).collect_vec();
        vs.iter().map(|p| p.1).sum()
    } else {
        parse(input).iter().map(|p| p.mirrors()).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    rows: Vec<Stripe>,
    cols: Vec<Stripe>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stripe {
    chs: Vec<char>,
}

impl std::ops::Deref for Stripe {
    type Target = Vec<char>;
    fn deref(&self) -> &Self::Target {
        &self.chs
    }
}

impl Stripe {
    fn new(chs: impl Iterator<Item = char>) -> Self {
        let chs = chs.into_iter().collect_vec();
        Self { chs }
    }
}

impl Pattern {
    fn parse(input: &str) -> Pattern {
        let cells: Vec<Vec<char>> = input
            .lines()
            .map(|row| row.chars().collect_vec())
            .collect_vec();
        let rows = cells
            .iter()
            .map(|row| row.iter().copied())
            .map(Stripe::new)
            .collect_vec();
        let cols = (0..cells.first().map(|r| r.len()).unwrap_or_default())
            .map(|x| {
                cells
                    .iter()
                    .map(move |row| row.get(x).expect("bad col"))
                    .copied()
            })
            .map(Stripe::new)
            .collect_vec();
        Pattern { rows, cols }
    }
    fn mirrors(&self) -> usize {
        fn stripe_reflects(stripes: &[Stripe]) -> usize {
            (1..stripes.len())
                .map(|idx| {
                    let prev = stripes[0..idx].iter().rev();
                    let next = stripes[idx..].iter();
                    if prev.zip(next).all(|(a, b)| a == b) {
                        idx
                    } else {
                        0
                    }
                })
                .sum()
        }
        let hrz = stripe_reflects(&self.rows);
        let vrt = stripe_reflects(&self.cols);
        hrz * 100 + vrt
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_pt2() {
        let ex1 = include_str!("ex1.txt");
        let res = summarize_patterns(ex1, true);
        assert_eq!(res, 400);
    }

    #[test]
    #[traced_test]
    fn test_pt1() {
        let ex1 = include_str!("ex1.txt");
        let res = summarize_patterns(ex1, false);
        assert_eq!(res, 405);
        let in1 = include_str!("in1.txt");
        let res = summarize_patterns(in1, false);
        assert_eq!(res, 39939);
    }

    #[test]
    #[traced_test]
    fn test_ex1_mirrors() {
        let ex1 = include_str!("ex1.txt");
        let pats = parse(ex1);
        let mrs = pats[0].mirrors();
        assert_eq!(mrs, 5); // vert match at idx=5
        let mrs = pats[1].mirrors();
        assert_eq!(mrs, 400); // horiz match at idx=4
    }

    #[test]
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let pat = parse(ex1);
        assert_eq!(pat.len(), 2);
        let row: String = pat[0].rows[0].iter().collect();
        assert_eq!(row, "#.##..##.");
        let col: String = pat[0].cols[0].iter().collect();
        assert_eq!(col, "#.##..#");
        let row: String = pat[0].rows.last().expect("last").iter().collect();
        assert_eq!(row, "#.#.##.#.");
        let col: String = pat[0].cols.last().expect("last").iter().collect();
        assert_eq!(col, "..##...");
    }
}
