use aoc_2020::prelude::*;

fn main() -> Result<()> {
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
        todo!()
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
            println!("ch:{ch} min:{min} max:{max}");
        }
        unreachable!()
    }

    fn col(&self) -> usize {
        let mut min = 0;
        let mut max = NUM_COLS - 1;
        for ch in self.0.chars().skip(7).take(3) {
            let mid = (max - min + 1) / 2;
            println!("COL ch:{ch} min:{min} max:{max} mid:{mid}");
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
            println!("  COL ch:{ch} min:{min} max:{max}");
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
}
