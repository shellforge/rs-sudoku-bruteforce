use rand::prelude::*;
use crate::puzzle_piece::{Cell, PuzzlePiece};

#[derive(Debug, Clone)]
pub struct PuzzleBoard {
    pub board: Vec<PuzzlePiece>,
}

pub type NumList = Vec<u32>;
pub type PuzzlePieceList = Vec<PuzzlePiece>;

impl PuzzleBoard {

    const SHUFFLE_COUNT: usize = 100;

    /// # About
    /// Constructs a blank board of size 9*9 by calling
    /// PuzzlePiece::build_blank_board()
    pub fn new() -> PuzzleBoard {

        PuzzleBoard {
            board: Self::build_blank_board(),
        }
    }

    /// # About
    /// Helper function for getting grid position in PuzzleBoard<Vec<PuzzlePiece<>>
    /// 
    /// # Example
    /// xy_idx(0, 0) returns board[0]
    /// xy_idx(3, 7) returns board[66]
    pub fn xy_idx(x: u32, y: u32) -> usize {
        (y as usize * 9) + x as usize
    }

    fn build_blank_board() -> Vec<PuzzlePiece> {    
        vec![PuzzlePiece::new(); 9*9]
    }

    pub fn fill_board(mut self) -> PuzzleBoard {
        let mut num = 1;
        for x in 0..self.board.len() {
            if num % 10 == 0 {
                num = 1;
            } 
            &self.board[x].set_val(num);
            num = num + 1;
        }

        self
    }

    pub fn make_row() -> NumList {
        Self::shuffle(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Self::SHUFFLE_COUNT)
    }

    pub fn extract_row(&self, row: u32) -> PuzzlePieceList {
        let mut temp: PuzzlePieceList = Vec::new();
        for x in 0..9 {
            temp.push(self.board[PuzzleBoard::xy_idx(x, row)].clone());
        }

        temp
    }

    pub fn extract_col(&self, col: u32) -> PuzzlePieceList {
        let mut temp: PuzzlePieceList = Vec::new();
        for y in 0..9 {
            temp.push(self.board[PuzzleBoard::xy_idx(col, y)].clone());
        }

        temp

    }

    pub fn insert_list(&mut self, row: u32, list: Vec<u32>) {
        for x in 0..9 {
            self.board[PuzzleBoard::xy_idx(x, row)].cell = Cell::Value(list[x as usize]);
        }
    }

    pub fn insert_new_chunk(&mut self, pos: u32) {
        let mut temp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut temp_pos = 0;
        temp = Self::shuffle(temp, Self::SHUFFLE_COUNT);

        match pos {
            0 => {
                for x in 0..3 {
                    for y in 0..3 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            1 => {
                for x in 3..6 {
                    for y in 0..3 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            2 => {
                for x in 6..9 {
                    for y in 0..3 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            3 => {
                for x in 0..3 {
                    for y in 3..6 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            4 => {
                for x in 3..6 {
                    for y in 3..6 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            5 => {
                for x in 6..9 {
                    for y in 3..6 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            6 => {
                for x in 0..3 {
                    for y in 6..9 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            7 => {
                for x in 3..6 {
                    for y in 6..9 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            8 => {
                for x in 6..9 {
                    for y in 6..9 {
                        self.board[PuzzleBoard::xy_idx(x, y)].set_val(temp[temp_pos] as u32);
                        temp_pos = temp_pos + 1;
                    }
                }
            },
            _ => (),
        }
    }

    pub fn print_board(&self) {
        let mut line_break = 0;
        for p in &self.board {
            if line_break % 10 == 0 {
                println!();
                line_break = 1;
            }
            line_break = line_break + 1;

            match p.cell {
                Cell::Value(v) => print!("{} ", v),
                Cell::None => print!("_ "),
            }
        }
        println!();
    }

    pub fn shuffle(mut list: Vec<u32>, mut count: usize) -> Vec<u32> {

        while count > 0 {
            let p_pos = Self::get_rand_pos();
            let n_pos = Self::get_rand_pos();
            let t_val = list[n_pos];
            list[n_pos] = list[p_pos];
            list[p_pos] = t_val;

            count = count - 1;
        }

        list
    }

    fn get_rand_pos() -> usize {
        let mut rng = rand::thread_rng();

        const MIN: u32 = 0;
        const MAX: u32 = 9;

        rng.gen_range(MIN, MAX) as usize
    }
}


#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test_blank_board() {
        let puzzle_board = PuzzleBoard::new();
        dbg!("{:?}", puzzle_board);
    }

    #[test]
    pub fn test_insert_chunk() {
        let mut puzzle_board = PuzzleBoard::new();
        dbg!("{:?}", &puzzle_board);
        for x in 0..9 {
            puzzle_board.insert_new_chunk(x);
            puzzle_board.print_board();    
        }
        
    }

    #[test]
    pub fn test_new_board_set_value() {
        let mut puzzle_board = PuzzleBoard::new();
        assert_eq!(puzzle_board.clone().board[PuzzleBoard::xy_idx(0, 0)].cell, Cell::None);
        puzzle_board.board[PuzzleBoard::xy_idx(0, 0)].set_val(9);
        assert_eq!(puzzle_board.clone().board[PuzzleBoard::xy_idx(0, 0)].get_val(), 9);
        dbg!(puzzle_board);
    }

    #[test]
    pub fn test_fill_in_board() {
        let puzzle_board = PuzzleBoard::new().fill_board();
        assert_eq!(puzzle_board.board.len(), 81);
        dbg!("{:?}", &puzzle_board);
        puzzle_board.print_board();

    }

    #[test]
    pub fn test_extract_row() {
        let puzzle_board = PuzzleBoard::new().fill_board();
        dbg!(puzzle_board.extract_row(0));
    }

    #[test]
    pub fn test_get_pos() {
        let puzzle_board = PuzzleBoard::new().fill_board();
        match &puzzle_board.board[PuzzleBoard::xy_idx(0, 0)].cell {
            Cell::None => (),
            Cell::Value(v) => assert_eq!(*v, 1),
        }
        dbg!(&puzzle_board.board[PuzzleBoard::xy_idx(0, 0)].cell);
    }
}