// extern crate puzzle_maker_lib;

// use crate::puzzle_maker_lib::puzzle_board::PuzzleBoard;
// use crate::puzzle_maker_lib::duplicate_checker::DuplicateChecker;

// #[test]
// fn test_build_board() {
//     println!(
//     "Running board build primary. 
//     Process builds the cubes for a board by starting
//     in the top and going left to right until the end.
    
//     We print the amount of iterations we go through to
//     insert new chunks.
    
//     We print the continous iterations for long running
//     chunks to watch for dead ends.
    
//     This is NOT a working process so far.\n\n");

//     let mut puzzle_board = PuzzleBoard::new();
//     let mut pos = 0;
//     let mut rows_inserted = 0;
//     let mut cont_runs = 0;

//     &puzzle_board.insert_list(pos, PuzzleBoard::make_row());
//     rows_inserted = rows_inserted + 1;

//     while pos < 9 {
//         cont_runs = cont_runs + 1;

//         &puzzle_board.insert_new_chunk(pos);
//         rows_inserted = rows_inserted + 1;

//         if !DuplicateChecker::duplicate_found(&puzzle_board, pos) {
//             pos = pos + 1;
//             cont_runs = 0;
//         }

//         if rows_inserted % 1000 == 0 {  

//             println!("iterations {}", rows_inserted);
//         }

//         // if cont_runs != 0 && cont_runs % 50000 == 0 {
//         //     println!("{} continuous iterations. Dead end?", &cont_runs);
//         //     &puzzle_board.print_board();
//         // }

//         if cont_runs != 0 && cont_runs % 1000000 == 0 {
//             println!("\n\n{} iterations without a solution. Dead end. #FailedBoard\n", &cont_runs);
//             puzzle_board.print_board();
//             std::process::exit(0);
//         }
//     }

//     println!("Board Success\n\n");
//     puzzle_board.print_board();
// }