use std::io::prelude::*;
use std::fs::File;

use crate::puzzle_board::PuzzleBoard;

pub struct FileWriter;

impl FileWriter {

    pub fn write_results(puzzle_board: &PuzzleBoard, ext: &str) {
        let name = [&puzzle_board.uuid.to_string(), ext].concat();
        let path = std::path::Path::new(&name);

        let serialized = &puzzle_board.pack_board_to_export();

        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(why) => panic!(why.to_string()),
        };

        match file.write_all(&serialized.as_bytes()) {
            Ok(r) => r,
            Err(why) => panic!(why.to_string()),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board_builder::BoardBuilder;

    #[test]
    pub fn test_write_results() {
        let puzzle_board: PuzzleBoard = BoardBuilder::build_fail_board();
        FileWriter::write_results(&puzzle_board, ".answer");
    }
}