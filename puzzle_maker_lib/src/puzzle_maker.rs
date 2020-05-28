use crate::puzzle_board::PuzzleBoard;
use crate::puzzle_patterns::PuzzlePatterns;

use rayon::prelude::*;

pub struct PuzzleMaker;

/// # About
/// Applies patterns to the generated board until the MAX_CLUES and supplied threshold
/// are met.
/// 
/// # ToDo
/// * Parallelization - In progress
impl PuzzleMaker {

    pub fn apply_patterns(puzzle_board: PuzzleBoard, threshold: usize) -> PuzzleBoard {
        const MAX_CLUES: usize = 39;

        let mut temp = puzzle_board.clone();
        let mut board_restores = 0;

        loop {
            temp = Self::apply_pattern(temp, PuzzlePatterns::get_rand_pattern());
            let clues = temp.clues_par();
            
            if clues < 39 {
                temp = puzzle_board.clone();
                board_restores = board_restores + 1;
                println!("Board restored {} times\nBoard clues {}", board_restores, clues);
            }
            else if clues > MAX_CLUES && clues <= threshold {
                return temp;
            }
        }
    }

    pub fn apply_pattern_par(puzzle_board: PuzzleBoard, pattern: Vec<(u32, u32)>) -> PuzzleBoard {
        let mut temp = Vec::new();
        pattern.into_par_iter().map_with(puzzle_board.clone(), |p, (x, y)| {
            dbg!(&p);
            p.board[PuzzleBoard::xy_idx(x, y)].set_val(0);
        }).collect_into_vec(&mut temp);
        dbg!(temp);

        puzzle_board

        // self.board.clone().into_par_iter().filter(|x| x.get_val() > 0).collect::<Vec<PuzzlePiece>>().len()

    }

    pub fn apply_pattern(mut puzzle_board: PuzzleBoard, pattern: Vec<(u32, u32)>) -> PuzzleBoard {
        for (x, y) in pattern.iter() {
            puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].set_val(0);
        }

        puzzle_board
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::puzzle_patterns::PuzzlePatterns;
    use crate::board_builder::BoardBuilder;


    #[test]
    pub fn test_apply_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzlePatterns::get_rand_pattern());
        puzzle_board.print_board();
        println!("Clues: {}", puzzle_board.clues());
    }

    #[test]
    pub fn test_apply_par_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board = PuzzleMaker::apply_pattern_par(puzzle_board, PuzzlePatterns::get_rand_pattern());
        puzzle_board.print_board();
        println!("Clues: {}", puzzle_board.clues());
    }
}