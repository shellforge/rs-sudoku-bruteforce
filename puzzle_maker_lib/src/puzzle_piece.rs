#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cell {
    Value(u32),
    None,
}

#[derive(Debug, Clone)]
pub struct PuzzlePiece {
    pub cell: Cell,
}

impl PuzzlePiece {

    pub fn new() -> PuzzlePiece {
        PuzzlePiece {
            cell: Cell::None,
        }
    }

    pub fn get_val(&self) -> u32 {
        match self.cell {
            Cell::Value(v) => v,
            Cell::None => panic!("This cell has no value"),
        }
    }

    pub fn set_val(&mut self, val: u32) {
        // dbg!(&val);
        self.cell = Cell::Value(val);
    }
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn test_new_puzzle_piece() {
        let piece = PuzzlePiece::new();
        assert_eq!(piece.cell, Cell::None);
    }

    #[test]
    pub fn test_set_puzzle_piece_value() {
        let mut piece = PuzzlePiece::new();
        piece.set_val(9);
        assert_eq!(piece.clone().get_val(), 9);
        dbg!(piece);
    }
}