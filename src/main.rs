extern crate rand;
extern crate rustbox;

mod board;
mod display;

use board::Board;
use display::{Display, Action};

fn main() {
    let mut board = Board::new();
    let display = Display::new();

    // Initialize board with two filled cells
    board.fill_cell();
    board.fill_cell();

    loop {
        display.draw(&board);
        match display.handle_event(&mut board) {
            Action::Shift => {
                board.fill_cell();
            },
            Action::End => {
                break;
            },
            _ => {}
        }
    }
}
