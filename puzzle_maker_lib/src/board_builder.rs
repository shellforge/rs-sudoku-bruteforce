use crate::puzzle_board::PuzzleBoard;
use crate::duplicate_checker::DuplicateChecker;

pub struct BoardBuilder;

impl BoardBuilder {

    pub fn build_brute() {
        println!("Build brute force");
        let mut resets = 0;

        'build: loop {
            
            'inner: loop {

                let mut pos = 0;
                let mut rows_inserted = 0;
                let mut cont_runs = 0;

                let mut puzzle_board = PuzzleBoard::new();
                &puzzle_board.insert_list(pos, PuzzleBoard::make_row());
                rows_inserted = rows_inserted + 1;

                while pos < 9 {
                    cont_runs = cont_runs + 1;
    
                    &puzzle_board.insert_new_chunk(pos);
                    rows_inserted = rows_inserted + 1;
    
                    if !DuplicateChecker::duplicate_found(&puzzle_board, pos) {
                        pos = pos + 1;
                        cont_runs = 0;
                    }
    
                    if cont_runs != 0 && cont_runs % 500000 == 0 {
                        println!("\n\n{} iterations without a solution. Dead end with board fail.\nReset num: {}\n", &cont_runs, resets);
                        puzzle_board.print_board();
                        resets = resets + 1;
                        break 'inner;
                    }
                }
                println!("Board Success\n\n");
                puzzle_board.print_board();
                break 'build;
            }
        }
    }
}