# About
Scratch project for working with Rust to build various aspects of Sudoku.

# Building
Running "cargo run" will call the Puzzle Maker lib to build a puzzle. Building as "cargo build --release" will provide a more optimized and speedier executable. This puzzle is not in a form that can be solved. Values still need to be removed from the board. 

There are two options, --build a puzzle or --solve a puzzle. The puzzle format needs to be in the format of the included in the examples directory. The --build function automatically formats them so.

# Todo
* [DONE] - Add menu options for solving a puzzle vs building a puzzle
* [DONE] - Take built puzzles and configure them to be solvable puzzles
* Provide a scoring mechanism to determine the difficulty of the puzzle to obscure
* review brute force solver to determine counts of potential answers to determine how many solutions exist for a puzzle
