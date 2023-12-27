use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::Display;

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", summarize_patterns(ex1, false));
    println!("p1in1 = {}", summarize_patterns(in1, false));
    println!("p2ex1 = {}", summarize_patterns(ex1, true));
    println!("p2in1 = {}", summarize_patterns(in1, true));
}

fn summarize_patterns(input: &str, smudges: bool) -> usize {
    if smudges {
        parse(input)
            .iter()
            .par_bridge()
            .into_par_iter()
            .map(|p| {
                let orig = p.mirrors();
                p.permute()
                    .find_map(|p| p.mirrors().into_iter().find(|m| !orig.contains(m)))
                    .expect("no new reflection")
                    .val()
            })
            .sum()
    } else {
        parse(input)
            .iter()
            .map(|p| p.mirrors())
            .map(sum_mirrors)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern {
    rows: Vec<Stripe>,
    cols: Vec<Stripe>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .rows
            .iter()
            .map(|row| row.chs.iter().collect::<String>())
            .join("\n");
        write!(f, "{s}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    fn toggle(&mut self, idx: usize) {
        if let Some(ch) = self.chs.get_mut(idx) {
            if *ch == '.' {
                *ch = '#';
            } else {
                *ch = '.';
            }
        }
    }
}

impl Pattern {
    fn parse(input: &str) -> Pattern {
        let cells: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|row| row.trim().chars().collect_vec())
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
    fn permute(&self) -> impl Iterator<Item = Pattern> + '_ {
        (0..self.rows.len())
            .flat_map(|y| (0..self.cols.len()).map(move |x| (x, y)))
            .map(|(x, y)| self.clone().toggle(x, y))
    }
    fn mirrors(&self) -> Vec<Mirror> {
        fn stripe_reflects(stripes: &[Stripe]) -> impl Iterator<Item = usize> + '_ {
            (1..stripes.len()).filter(|idx| {
                let prev = stripes[0..*idx].iter().rev();
                let next = stripes[*idx..].iter();
                prev.zip(next).all(|(a, b)| a == b)
            })
        }
        stripe_reflects(&self.rows)
            .map(Mirror::from_row)
            .chain(stripe_reflects(&self.cols).map(Mirror::from_col))
            .collect_vec()
    }
    fn toggle(mut self, x: usize, y: usize) -> Self {
        self.rows.get_mut(y).expect("no row at y").toggle(x);
        self.cols.get_mut(x).expect("no col at x").toggle(y);
        self
    }
}

fn sum_mirrors(i: impl IntoIterator<Item = Mirror>) -> usize {
    i.into_iter().map(|m| m.val()).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Mirror {
    row: usize,
    col: usize,
}

impl Mirror {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    fn from_row(idx: usize) -> Self {
        Self::new(idx, 0)
    }
    fn from_col(idx: usize) -> Self {
        Self::new(0, idx)
    }
    fn val(&self) -> usize {
        self.row * 100 + self.col
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
        let in1 = include_str!("in1.txt");
        let res = summarize_patterns(in1, true);
        assert_eq!(res, 32069);
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
        assert_eq!(sum_mirrors(mrs), 5); // vert match at idx=5
        let mrs = pats[1].mirrors();
        assert_eq!(sum_mirrors(mrs), 400); // horiz match at idx=4
    }

    #[test]
    #[traced_test]
    fn test_permute() {
        let ex1 = include_str!("ex1.txt");
        let pat = &parse(ex1)[0];
        let prms = pat.permute().collect_vec();
        assert_eq!(prms.len(), 63); // 7x9
        assert!(prms.iter().all_unique());
        assert!(!prms.iter().any(|p| p == pat));
        assert!(prms.iter().all(|p| p.rows.len() == 7 && p.cols.len() == 9));
        assert!(prms
            .iter()
            .flat_map(|r| r.cols.iter().chain(r.rows.iter()))
            .flat_map(|s| s.chs.iter())
            .all(|ch| ch == &'.' || ch == &'#'))
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
