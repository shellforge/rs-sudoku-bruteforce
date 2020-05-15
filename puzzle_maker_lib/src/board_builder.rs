use crate::puzzle_board::PuzzleBoard;
use crate::duplicate_checker::DuplicateChecker;
use crate::file_writer::FileWriter;

pub struct BoardBuilder;

impl BoardBuilder {

    pub fn build_brute() -> PuzzleBoard {
        println!("Build brute force");
        let mut resets = 0;

        loop {
            
            'inner: loop {

                let mut pos = 0;
                let mut rows_inserted = 0;
                let mut cont_runs = 0;

                let mut puzzle_board = PuzzleBoard::new();
                &puzzle_board.insert_new_chunk(pos);
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
                        println!("\n\n{} iterations without a solution. Dead end with board fail.\nReset num: {}", &cont_runs, resets);
                        puzzle_board.print_board();
                        resets = resets + 1;
                        break 'inner;
                    }
                }
                println!("\n\nBoard Success");
                puzzle_board.print_board();

                return puzzle_board;
            }
        }
    }

    pub fn build_fail_board() -> PuzzleBoard {
        println!("Building fail board for tests only");

        let mut pos = 0;
        let mut rows_inserted = 0;
        let mut cont_runs = 0;
        let mut puzzle_board = PuzzleBoard::new();
        &puzzle_board.insert_new_chunk(pos);
        rows_inserted = rows_inserted + 1;
        while pos < 9 {
            cont_runs = cont_runs + 1;
            &puzzle_board.insert_new_chunk(pos);
            rows_inserted = rows_inserted + 1;
            pos = pos + 1;
            puzzle_board.print_board();
        }
        println!("\n\nBoard Success");
        puzzle_board.print_board();
        return puzzle_board;
    }

    pub fn write_to_file(puzzle_board: &PuzzleBoard, extension: &str) {
        FileWriter::write_results(&puzzle_board, extension);
    }
}