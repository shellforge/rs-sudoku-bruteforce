pub mod board_builder;
pub mod puzzle_maker;

mod puzzle_board;
mod duplicate_checker;
mod puzzle_piece;
mod file_writer;
mod puzzle_patterns;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
