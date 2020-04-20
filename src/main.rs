
#[derive(Debug, Clone)]
struct Cell {
    pub established: bool,
    pub value: u64,
}

type Grid = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
struct State {
    pub grid: Grid,
    pub row: usize,
    pub col: usize,
}

impl State {
    fn new(input: serde_json::Value) -> Result<State, String> {
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

        Ok(State {
            grid: grid,
            col: start_col,
            row: 0,
        })
    }

    fn backtrack(&mut self) {
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

    fn next_position(&mut self) {
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

fn not_in_row(grid: &Grid, row: usize, value: u64) -> bool {
    !grid[row].iter().any(|c| c.value == value)
}

fn not_in_column(grid: &Grid, col: usize, value: u64) -> bool {
    let col_vector: Vec<u64> = grid
        .iter()
        .map(|row| row[col].value)
        .collect();
    !col_vector.contains(&value)
}

fn not_in_square(grid: &Grid, row: usize, col: usize, value: u64) -> bool {
    let row_start = (row / 3) * 3;
    let col_start = (col / 3) * 3;

    for i in row_start..(row_start+3) {
        for j in col_start..(col_start+3) {
            if grid[i][j].value == value { return false; }
        }
    }

    true
}

fn next_lowest_value(grid: &Grid, row: usize, col: usize, last_value: u64) -> Option<u64> {
    for i in (last_value+1)..10 {
        if not_in_column(grid,col,i) && not_in_row(grid,row,i) && not_in_square(grid,row,col,i) {
            return Some(i);
        }
    }

    None
}

/*
fn read_input(filename: &str) -> serde_json::Value {

}
*/

fn display_output(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{} ", cell.value);
        }
        println!("");
    }   
}

fn solve(state: &mut State) -> Grid {
    loop {
        let last_value = state.grid[state.row][state.col].value;
        let v = next_lowest_value(&state.grid, state.row, state.col, last_value);
        if v.is_none() {
            state.backtrack();
        }
        else {
            state.grid[state.row][state.col].value = v.unwrap();
            if state.row == 8 && state.col == 8 {
                break;
            }
            state.next_position();
        }
    }
    state.grid.clone()
}

fn main() {
    println!("sudoku solver!");
    
    let input = serde_json::json!([
        [0, 0, 8, 0, 0, 0, 6, 0, 0],
        [0, 0, 0, 1, 0, 2, 0, 4, 0],
        [7, 0, 2, 0, 8, 0, 0, 0, 5],
        [0, 5, 0, 9, 2, 0, 0, 8, 0],
        [0, 0, 1, 6, 0, 7, 9, 0, 0],
        [0, 4, 0, 0, 5, 3, 0, 1, 0],
        [1, 0, 0, 0, 6, 0, 8, 0, 2],
        [0, 8, 0, 7, 0, 4, 0, 0, 0],
        [0, 0, 3, 0, 0, 0, 1, 0, 0],
    ]);
    let mut state0 = State::new(input).unwrap(); // bad
    solve(&mut state0);
   
    display_output(&state0.grid);
}

#[cfg(test)]
mod test {
    use super::{not_in_column, not_in_row, not_in_square, next_lowest_value, solve, State};
    use serde_json::json;

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
        let state0 = State::new(input0()).unwrap();
        assert_eq!(state0.grid[0][2].value, 8);
        assert_eq!(state0.col, 0);
        assert_eq!(state0.row, 0);
    }

    #[test]
    fn test_not_in_row() {
        let state0 = State::new(input0()).unwrap();
        assert!(not_in_row(&state0.grid, 0, 1));
        assert!(!not_in_row(&state0.grid, 0, 8));
        assert!(not_in_row(&state0.grid, 4, 5));
        assert!(!not_in_row(&state0.grid, 4, 9));
    }

    #[test]
    fn test_not_in_column() {
        let state0 = State::new(input0()).unwrap();
        assert!(not_in_column(&state0.grid, 0, 2));
        assert!(!not_in_column(&state0.grid, 0, 7));
        assert!(not_in_column(&state0.grid, 6, 5));
        assert!(!not_in_column(&state0.grid, 6, 9));
    }

    #[test]
    fn test_not_in_square() {
        let state0 = State::new(input0()).unwrap();
        assert!(not_in_square(&state0.grid, 0, 0, 1));
        assert!(!not_in_square(&state0.grid, 0, 0, 8));
        assert!(not_in_square(&state0.grid, 5, 6, 5));
        assert!(!not_in_square(&state0.grid, 5, 6, 9));
    }

    #[test]
    fn test_next_lowest_value() {
        let state0 = State::new(input0()).unwrap();
        assert_eq!(next_lowest_value(&state0.grid, 0, 3, 0).unwrap(), 3);
        assert_eq!(next_lowest_value(&state0.grid, 4, 4, 0).unwrap(), 4);
        assert_eq!(next_lowest_value(&state0.grid, 4, 4, 5), None);
    }

    #[test]
    fn test_next_position() {
        let mut state0 = State::new(input0()).unwrap();
        state0.next_position();
        assert_eq!(state0.row, 0);
        assert_eq!(state0.col, 1);
        state0.row = 2;
        state0.col = 7;
        state0.next_position();
        assert_eq!(state0.row, 3);
        assert_eq!(state0.col, 0);
    }

    #[test]
    fn test_backtrack() {
        let mut state0 = State::new(input0()).unwrap();
        state0.row = 2;
        state0.col = 1;
        state0.backtrack();
        assert_eq!(state0.row, 1);
        assert_eq!(state0.col, 8);
        state0.row = 4;
        state0.col = 7;
        state0.backtrack();
        assert_eq!(state0.row, 4);
        assert_eq!(state0.col, 4);
    }

    #[test]
    fn test_solve() {
        let mut state0 = State::new(input0()).unwrap();
        solve(&mut state0);
    }
}
