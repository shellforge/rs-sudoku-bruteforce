use crate::puzzle_board::PuzzleBoard;
use crate::puzzle_patterns::PuzzlePatterns;

pub struct PuzzleMaker;

impl PuzzleMaker {

    pub fn apply_patterns(puzzle_board: PuzzleBoard, threshold: usize) -> PuzzleBoard {
        let mut temp = puzzle_board.clone();
        let mut board_restores = 0;
        loop {
            temp = Self::apply_pattern(temp, PuzzlePatterns::get_rand_pattern());

            if temp.clues() < 39 {
                temp = puzzle_board.clone();
                board_restores = board_restores + 1;
                println!("Board restored {} times\nBoard clues {}", board_restores, temp.clues());
            }
            if temp.clues() > 39 && temp.clues() <= threshold {
                return temp;
            }
        }
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
    use crate::board_builder::BoardBuilder;

    #[test]
    pub fn test_apply_patterns() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board = PuzzleMaker::apply_patterns(puzzle_board, 45);
        puzzle_board.print_board();
        println!("Clues: {}", puzzle_board.clues());
    }
}