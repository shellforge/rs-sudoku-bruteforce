#[derive(PartialEq, Debug, Clone)]
pub struct Piece {
    pub established: bool,
    pub value: u64,
}

pub type Grid = Vec<Vec<Piece>>;

#[derive(Debug, Clone, Default)]
pub struct PuzzleState {
    pub grid: Grid,
    pub row: usize,
    pub col: usize,
}

impl PuzzleState {
    pub fn with_values(input: serde_json::Value) -> Result<PuzzleState, String> {
        
        let rows: Vec<Vec<u64>> = match serde_json::from_value(input) {
            Err(why) => panic!(why.to_string()),
            Ok(v) => v,
        };
        
        let grid: Grid = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|n| Piece {
                        established: (*n != 0),
                        value: n.clone(),
                    })
                    .collect()
            })
            .collect();

        // find starting position assuming that sudoku starts
        // with an empty cell in the first row
        let mut start_col: usize = 0;
        for i in 0..9 {
            if !grid[0][i].established {
                start_col = i;
                break;
            }
        }

        Ok(PuzzleState {
            grid: grid,
            col: start_col,
            row: 0,
        })
    }

    pub fn backtrack(&mut self) {
        self.grid[self.row][self.col].value = 0;

        loop {
            if self.col == 0 {
                self.col = 9;
                self.row = self.row - 1;
            }
            self.col = self.col - 1;

            if !self.grid[self.row][self.col].established {
                break;
            }
        }
    }

    pub fn next_position(&mut self) {
        loop {
            self.col = self.col + 1;
            if self.col == 9 {
                self.row = self.row + 1;
                self.col = 0;
            }

            if !self.grid[self.row][self.col].established {
                break;
            }
        }
    }
}

#[cfg(test)]

mod test {

    use super::*;
    use crate::file_reader::FileReader;

    use serde_json::*;
    
    fn input0() -> serde_json::Value {
        json!([
            [0, 0, 8, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 1, 0, 2, 0, 4, 0],
            [7, 0, 2, 0, 8, 0, 0, 0, 5],
            [0, 5, 0, 9, 2, 0, 0, 8, 0],
            [0, 0, 1, 6, 0, 7, 9, 0, 0],
            [0, 4, 0, 0, 5, 3, 0, 1, 0],
            [1, 0, 0, 0, 6, 0, 8, 0, 2],
            [0, 8, 0, 7, 0, 4, 0, 0, 0],
            [0, 0, 3, 0, 0, 0, 1, 0, 0],
        ])
    }

    #[test]
    fn test_new_state() {
        let state0 = PuzzleState::with_values(input0()).unwrap();
        assert_eq!(state0.grid[0][2].value, 8);
        assert_eq!(state0.col, 0);
        assert_eq!(state0.row, 0);
    }

    #[test]
    fn test_read_sudoku_test_file() {
        let file = "test_files/sudoku_test.json";
        let file = FileReader::read(file);
        let file = serde_json::from_str(&file).unwrap();
        let state0 = PuzzleState::with_values(file).unwrap();

        assert_eq!(state0.grid[0][2].value, 8);
        assert_eq!(state0.col, 0);
        assert_eq!(state0.row, 0);
    }
}