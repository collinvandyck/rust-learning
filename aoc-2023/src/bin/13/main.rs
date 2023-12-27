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

// the problem right now is that we don't know from the output of mirrors if the original line was
// horiz or vertical.
fn summarize_patterns(input: &str, smudges: bool) -> usize {
    if smudges {
        let pats = parse(input);
        pats.iter()
            .take(1)
            .map(|p| {
                let mrs = p.mirrors();
                assert_eq!(mrs.len(), 1);
                (p, mrs[0])
            })
            .enumerate()
            .par_bridge()
            .into_par_iter()
            .map(|(idx, (p, orig))| {
                p.permute()
                    .find_map(|p| p.mirrors().into_iter().filter(|m| m != &orig).next())
                    .unwrap_or_else(|| panic!("no permuted diff found for idx={idx}"))
            })
            .map(|m| m.val())
            .sum()
    } else {
        parse(input)
            .iter()
            .map(|p| p.mirrors())
            .map(sum_mirrors)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn permute(&self) -> impl Iterator<Item = Pattern> + '_ {
        (0..self.rows.len())
            .flat_map(|y| (0..self.cols.len()).map(move |x| (x, y)))
            .map(|(x, y)| {
                let mut pat = self.clone();
                let row = pat.rows.get_mut(y).expect("no row at y");
                let ch = row.chs.get_mut(x).expect("no ch at x");
                if *ch == '.' {
                    *ch = '#';
                } else {
                    *ch = '.';
                }
                pat
            })
    }
    fn mirrors(&self) -> Vec<Mirror> {
        fn stripe_reflects(stripes: &[Stripe]) -> impl Iterator<Item = usize> + '_ {
            (1..stripes.len()).filter_map(|idx| {
                let prev = stripes[0..idx].iter().rev();
                let next = stripes[idx..].iter();
                if prev.zip(next).all(|(a, b)| a == b) {
                    Some(idx)
                } else {
                    None
                }
            })
        }
        stripe_reflects(&self.rows)
            .map(|idx| Mirror {
                row_idx: idx,
                col_idx: 0,
            })
            .chain(stripe_reflects(&self.cols).map(|idx| Mirror {
                row_idx: 0,
                col_idx: idx,
            }))
            .collect_vec()
    }
}

fn sum_mirrors(i: impl IntoIterator<Item = Mirror>) -> usize {
    i.into_iter().map(|m| m.val()).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mirror {
    row_idx: usize,
    col_idx: usize,
}

impl Mirror {
    fn val(&self) -> usize {
        self.row_idx * 100 + self.col_idx
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
        assert_eq!(sum_mirrors(mrs), 5); // vert match at idx=5
        let mrs = pats[1].mirrors();
        assert_eq!(sum_mirrors(mrs), 400); // horiz match at idx=4
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
