use crate::puzzle_board::PuzzleBoard;
use crate::puzzle_piece::{PuzzlePiece, Cell};

pub struct DuplicateChecker;

impl DuplicateChecker {

    /// # About
    /// Checks for duplicates horizontal and then vertical across the board.
    /// Returning when we find a None in the horiz.
    pub fn duplicate_found(puzzle_board: &PuzzleBoard, pos: u32) -> bool {
        match pos {
            0 => return Self::board_iter(puzzle_board, vec![0, 1, 2], vec![0, 1, 2]),
            1 => return Self::board_iter(puzzle_board, vec![0, 1, 2], vec![3, 4, 5]),
            2 => return Self::board_iter(puzzle_board, vec![0, 1, 2], vec![6, 7, 8]),
            3 => return Self::board_iter(puzzle_board, vec![3, 4, 5], vec![0, 1, 2]),
            4 => return Self::board_iter(puzzle_board, vec![3, 4, 5], vec![3, 4, 5]),
            5 => return Self::board_iter(puzzle_board, vec![3, 4, 5], vec![6, 7, 8]),
            6 => return Self::board_iter(puzzle_board, vec![6, 7, 8], vec![0, 1, 2]),
            7 => return Self::board_iter(puzzle_board, vec![6, 7, 8], vec![3, 4, 5]),
            8 => return Self::board_iter(puzzle_board, vec![6, 7, 8], vec![6, 7, 8]),
            _ => false,
        }
    }

    fn board_iter(puzzle_board: &PuzzleBoard, row: Vec<u32>, col: Vec<u32>) -> bool {
        for pos in row {
            if Self::list_dup_found(&puzzle_board.extract_row(pos)) {
                return true;
            }
        }

        for pos in col {
            if Self::list_dup_found(&puzzle_board.extract_col(pos)) {
                return true;
            }
        }
        
        false
    }

    pub fn list_dup_found(list: &Vec<PuzzlePiece>) -> bool {
        for x in 0..list.len() {
            for y in (x + 1)..list.len() {
                match (list[x].cell.clone(), list[y].cell.clone()) {
                    (Cell::Value(f), Cell::Value(s)) => {
                        if f == s {
                            return true;
                        }
                    },
                    (Cell::Value(_), Cell::None) => (),
                    (Cell::None, Cell::None) => (),
                    (Cell::None, _) => (),
                }
            }
        }

        false
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test_column_dup_find() {
        let mut pb = PuzzleBoard::new();

        &pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        &pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        &pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        &pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        &pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        &pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        &pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        &pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        &pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);

        // dbg!(&pb);

        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), false);
    }

    #[test]
    pub fn test_list_dup_found() {
        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);
        pb.print_board();

        assert_eq!(DuplicateChecker::list_dup_found(&pb.extract_row(0)), false);

        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);
        pb.print_board();

        assert_eq!(DuplicateChecker::list_dup_found(&pb.extract_row(0)), false);
    }

    #[test]
    pub fn test_column_dup_find_dup() {
        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);
        pb.print_board();

        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), true);


        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(1);
        pb.print_board();
        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), true);

        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);
        pb.print_board();
        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), true);

        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        // pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(1);
        pb.print_board();
        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), true);

        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        // pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        // pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        // pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        // pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        // pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(9);
        pb.print_board();
        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), false);

        let mut pb = PuzzleBoard::new();

        pb.board[PuzzleBoard::xy_idx(0, 0)].set_val(1);
        // pb.board[PuzzleBoard::xy_idx(0, 1)].set_val(2);
        // pb.board[PuzzleBoard::xy_idx(0, 2)].set_val(3);
        // pb.board[PuzzleBoard::xy_idx(0, 3)].set_val(4);
        // pb.board[PuzzleBoard::xy_idx(0, 4)].set_val(5);
        // pb.board[PuzzleBoard::xy_idx(0, 5)].set_val(6);
        // pb.board[PuzzleBoard::xy_idx(0, 6)].set_val(7);
        // pb.board[PuzzleBoard::xy_idx(0, 7)].set_val(8);
        // pb.board[PuzzleBoard::xy_idx(0, 8)].set_val(1);
        pb.print_board();
        assert_eq!(DuplicateChecker::list_dup_found(&pb.board), false);

    }
}