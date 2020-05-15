use rand::prelude::*;

pub struct PuzzlePatterns;

impl PuzzlePatterns {
    const STAR: [(u32, u32); 17] =
    [
        (0,0), (1,1), (7,7), (8,8), 
        (8,0), (7,1), (0,8), (1,7),
        (4,0), (4,1), (4,7), (4,8),
        (0,4), (1,4), (7,4), (8,4),
        (4,4),
    ];


    const EAST_TO_WEST: [(u32, u32); 24] = 
    [
        (0,0), (0,1), (0,2), (2,6), (2,7), (2,8),
        (8,0), (8,1), (8,3), (6,6), (6,7), (6,8),
        (2,0), (2,1), (2,2), (0,6), (0,7), (0,8),
        (6,0), (6,1), (6,2), (8,6), (8,7), (8,8),

    ];

    const XS: [(u32, u32); 17] = 
    [
        (0,0), (1,1), (2,2), (6,2), (7,1), (8,0),
        (3,3), (3,5), (4,4), (5,3), (5,5), 
        (0,8), (1,7), (2,6), (6,6), (7,7), (8,8),
    ];

    const OS: [(u32, u32); 36] =
    [
        (1,0), (4,0), (7,0), 
        (0,1), (2,1), (3,1), (5,1), (6,1), (8,1),
        (1,2), (4,2), (7,2),

        (1,3), (4,3), (7,3),
        (0,4), (2,4), (3,4), (5,4), (6,4), (8,4),
        (1,5), (4,5), (7,5),

        (1,6), (4,6), (7,6),
        (0,7), (2,7), (3,7), (5,7), (6,7), (8,7),
        (1,8), (4,8), (7,8),
    ];

    const TWOS_X_FOURS_X_TWOS: [(u32, u32); 24] =
    [
        (1,0), (3,0), (5,0), (7,0),
        (1,2), (3,2), (5,2), (7,2),
        (1,3), (3,3), (5,3), (7,3),
        (1,5), (3,5), (5,5), (7,5),
        (1,6), (3,6), (5,6), (7,6),
        (1,8), (3,8), (5,8), (7,8),
    ];

    const FOUR_CORNERS: [(u32, u32); 20] =
    [
        (0,0), (0,1), (1,0),
        (7,0), (8,0), (8,1),
        (3,3), (3,4), (3,5), (4,3), (4,5), (3,5), (4,5), (5,5),
        (0,7), (0,8), (1,8),
        (8,7), (7,8), (8,8),
    ];

    const SPIRALS: [(u32, u32); 25] =
    [
        (0,0), (2,0), (2,1), (1,2), (4,1), (5,1), (5,2), (7,2),
        (0,4), (2,4), (3,3), (5,3), (4,4), (3,5), (5,5), (6,4), (8,4),
        (1,6), (3,6), (3,7), (4,7), (7,6), (6,7), (6,8), (8,8),
    ];

    pub fn get_rand_pattern() -> Vec<(u32, u32)> {
        let mut rng = rand::thread_rng();
        const MIN: u32 = 0;
        const MAX: u32 = 7;
        let board = rng.gen_range(MIN, MAX);
    
        match board {
            0 => Self::STAR.to_vec(),
            1 => Self::EAST_TO_WEST.to_vec(),
            2 => Self::XS.to_vec(),
            3 => Self::OS.to_vec(),
            4 => Self::TWOS_X_FOURS_X_TWOS.to_vec(),
            5 => Self::FOUR_CORNERS.to_vec(),
            6 => Self::SPIRALS.to_vec(),
            _ => panic!("Out of range"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::board_builder::BoardBuilder;
    use crate::puzzle_patterns::PuzzlePatterns;
    use crate::puzzle_maker::PuzzleMaker;
    use crate::puzzle_board::PuzzleBoard;

    #[test]
    pub fn test_star_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzlePatterns::STAR.to_vec());
        for (x, y) in PuzzlePatterns::STAR.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
        
        puzzle_board.print_stats();
    }


    #[test]
    pub fn test_east_to_west_pattern() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzlePatterns::EAST_TO_WEST.to_vec());
        for (x, y) in PuzzlePatterns::EAST_TO_WEST.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }

        puzzle_board.print_stats();
    }

    
    #[test]
    pub fn test_all_patterns() {
        let mut puzzle_board = BoardBuilder::build_fail_board();
        puzzle_board.print_board();

        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzlePatterns::STAR.to_vec());
        puzzle_board = PuzzleMaker::apply_pattern(puzzle_board, PuzzlePatterns::EAST_TO_WEST.to_vec());

        for (x, y) in PuzzlePatterns::STAR.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
        for (x, y) in PuzzlePatterns::EAST_TO_WEST.iter() {
            assert_eq!(puzzle_board.board[PuzzleBoard::xy_idx(*x, *y)].get_val(), 0);
        }
    }
}