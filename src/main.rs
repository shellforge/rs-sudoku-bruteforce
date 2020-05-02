mod puzzle_state;
mod brute_solver;

use crate::puzzle_state::PuzzleState;
use crate::brute_solver::BruteSolver;

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

    match PuzzleState::new(input) {
        Ok(mut result) => {
            let grid_results = BruteSolver::solve(&mut result);
            BruteSolver::display_output(grid_results);       
        },
        Err(why) => panic!(why),
    }
}
