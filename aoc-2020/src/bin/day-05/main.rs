use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = get_assignments("input.txt")?
        .into_iter()
        .map(|a| a.seat_id())
        .max();
    println!("p1={p1:?}");
    Ok(())
}

fn get_assignments(p: impl AsRef<Path>) -> Result<Vec<Assignment>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    Ok(file_to_lines(p)?.into_iter().map(Assignment).collect())
}

static NUM_ROWS: usize = 128;
static NUM_COLS: usize = 8;

struct Assignment(String);

impl Assignment {
    fn seat_id(&self) -> usize {
        self.row() * 8 + self.col()
    }

    fn row(&self) -> usize {
        let mut min = 0;
        let mut max = NUM_ROWS - 1;
        for ch in self.0.chars().take(7) {
            let mid = (max - min + 1) / 2;
            match ch {
                'F' => {
                    if mid == 1 {
                        return min;
                    }
                    max -= mid
                }
                'B' => {
                    if mid == 1 {
                        return max;
                    }
                    min += mid
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }

    fn col(&self) -> usize {
        let mut min = 0;
        let mut max = NUM_COLS - 1;
        for ch in self.0.chars().skip(7).take(3) {
            let mid = (max - min + 1) / 2;
            match ch {
                'L' => {
                    if mid == 1 {
                        return min;
                    }
                    max -= mid
                }
                'R' => {
                    if mid == 1 {
                        return max;
                    }
                    min += mid
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}

#[test]
fn test_assignment() {
    let a = Assignment(String::from("FBFBBFFRLR"));
    assert_eq!(a.row(), 44);
    assert_eq!(a.col(), 5);
    let a = Assignment(String::from("BFFFBBFRRR"));
    assert_eq!(a.row(), 70);
    assert_eq!(a.col(), 7);
}
