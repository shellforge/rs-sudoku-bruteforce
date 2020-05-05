#[derive(Debug, Clone)]
pub struct Cell {
    pub established: bool,
    pub value: u64,
}

pub type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
pub struct PuzzleState {
    pub grid: Grid,
    pub row: usize,
    pub col: usize,
}

impl PuzzleState {
    pub fn new(input: serde_json::Value) -> Result<PuzzleState, String> {
        let rows: Vec<Vec<u64>> = match serde_json::from_value(input) {
            Err(_e) => {
                println!("error parsing");
                return Err("error parsing".to_string());
            }
            Ok(v) => v,
        };
        let grid: Grid = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|n| Cell {
                        established: (*n != 0),
                        value: n.clone(),
                    })
                    .collect()
            })
            .collect();

        // find starting position assuming that sudo starts
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