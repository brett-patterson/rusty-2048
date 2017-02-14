extern crate rusty_2048;

use rusty_2048::{Board, View, Action};

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
            Action::NoShift => {
                println!("You lost!");
                break;
            }
            Action::End => break,
            _ => {}
        }
    }
}
