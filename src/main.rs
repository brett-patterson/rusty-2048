extern crate rand;
extern crate rustbox;

mod board;
mod view;

use board::Board;
use view::{View, Action};

fn main() {
    let mut board = Board::new();
    let view = View::new();

    // Initialize board with two filled cells
    board.fill_cell();
    board.fill_cell();

    loop {
        view.draw(&board);
        match view.handle_event(&mut board) {
            Action::Shift => {
                board.fill_cell();
            }
            Action::End => {
                break;
            }
            _ => {}
        }
    }
}
