use crate::puzzle_board::PuzzleBoard;

pub struct PuzzleMaker;

impl PuzzleMaker {

    pub const PATTERN_STAR: [(u32, u32); 17] =
    [
        (0,0), (1,1), (7,7), (8,8), 
        (8,0), (7,1), (0,8), (1,7),
        (4,0), (4,1), (4,7), (4,8),
        (0,4), (1,4), (7,4), (8,4),
        (4,4),
    ];


    pub const PATTERN_EAST_TO_WEST: [(u32, u32); 16] = 
    [
        (0,0), (0,1), (2,7), (2,8),
        (8,0), (8,1), (6,7), (6,8),
        (2,0), (2,1), (0,7), (0,8),
        (6,0), (6,1), (8,7), (8,8),

    ];

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
    pub fn test_star_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_STAR.to_vec());
        for (x, y) in PuzzleMaker::PATTERN_STAR.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
 
 
    }


    #[test]
    pub fn test_east_to_west_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_EAST_TO_WEST.to_vec());
        for (x, y) in PuzzleMaker::PATTERN_EAST_TO_WEST.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
    }

    
    #[test]
    pub fn test_all_patterns() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_STAR.to_vec());
        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzleMaker::PATTERN_EAST_TO_WEST.to_vec());
        for (x, y) in PuzzleMaker::PATTERN_STAR.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
        for (x, y) in PuzzleMaker::PATTERN_EAST_TO_WEST.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
    }
}