extern crate puzzle_maker_lib;

mod menu;

use crate::menu::Menu;
use puzzle_maker_lib::board_builder::BoardBuilder;

fn main() {

    match Menu::display() {
        1 => BoardBuilder::build_brute(),
        _ => (),
    }
}