use std::env;

pub mod puzzle_state;
pub mod brute_solver;
pub mod file_reader;

use crate::puzzle_state::PuzzleState;
use crate::brute_solver::BruteSolver;
use crate::file_reader::FileReader;

fn main() {
    // Read in command line args
    // options: Create Puzzle, Solve Puzzle
    println!("sudoku solver!");

    // let args: Vec<String> = env::args().collect();

    // if args.len() == 1 {
    //     println!("--file [file name] -- input file name");
    //     std::process::exit(0);
    // }

    // match args[1].parse::<String>() {
    //     "--file" => 
    // }

    // let file = "brute_solver_prgm/test_files/sudoku_test.json";
    let file = "brute_solver_lib/test_files/7eafa229-8de4-451b-9ebc-bf08dea46607.result";

    let file = FileReader::read(file);

    let input = serde_json::from_str(&file).unwrap();

    match PuzzleState::with_values(input) {
        Ok(mut result) => {
            let grid_results = BruteSolver::solve(&mut result);
            BruteSolver::display_output(grid_results);       
        },
        Err(why) => panic!(why.to_string()),
    }
}
