use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

const SIZE: usize = 9;

#[derive(Serialize, Deserialize)]
struct Sudoku {
    board: [[u8; SIZE]; SIZE],
    #[serde(skip_deserializing)]
    tries: i32,
}

impl Sudoku {
    fn solve(&mut self) -> bool {
        self.tries += 1;
        let (row, col) = match self.find_empty() {
            Some(rc) => rc,
            None => {
                return true;
            }
        };
        for num in 1..=SIZE {
            if self.is_safe(row, col, num as u8) {
                self.board[row][col] = num as u8;
                if self.solve() {
                    return true;
                }
                self.board[row][col] = 0;
            }
        }
        false
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..SIZE {
            if self.board[row][i] == num {
                return false;
            }
        }
        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        true
    }

    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!("Tries: {}\n", self.tries));
        for i in 0..SIZE {
            let mut row = String::new();
            for j in 0..SIZE {
                let ch = format!("{}", self.board[i][j]);
                row.push_str(&ch);
                if j < SIZE - 1 {
                    row.push_str(" ")
                }
            }
            res.push_str(&row);
            res.push_str(&"\n");
        }
        res
    }
}

async fn solve(Json(mut sudoku): Json<Sudoku>) -> Result<String, StatusCode> {
    if sudoku.solve() {
        Ok(sudoku.to_string())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/solve", post(solve));

    Ok(router.into())
}
