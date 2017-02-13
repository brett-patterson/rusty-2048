extern crate rand;
extern crate rustbox;

mod board;
mod display;

use board::Board;
use display::Display;
use rustbox::{Event, Key};

fn main() {
    let mut board = Board::new();
    let mut display = Display::new();

    // Initialize board with two filled cells
    board.fill_cell();
    board.fill_cell();

    loop {
        display.draw(&board);

        match display.poll_event() {
            Ok(Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
