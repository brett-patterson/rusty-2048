use std::char;

use rustbox::{RustBox, Color, EventResult, RB_BOLD};
use super::board::{Board, BoardCell, BOARD_ROWS, BOARD_COLS};

pub struct Display {
    term: RustBox,
}

impl Display {
    pub fn new() -> Self {
        Display { term: RustBox::init(Default::default()).unwrap() }
    }

    pub fn draw(&self, board: &Board) {
        for i in 1..BOARD_ROWS + 1 {
            self.term.print_char(0, i, RB_BOLD, Color::White, Color::Black, '|');
            self.term.print_char(BOARD_COLS + 1, i, RB_BOLD, Color::White, Color::Black, '|');
        }

        for i in 1..BOARD_COLS + 1 {
            self.term.print_char(i, 0, RB_BOLD, Color::White, Color::Black, '-');
            self.term.print_char(i, BOARD_ROWS + 1, RB_BOLD, Color::White, Color::Black, '-');
        }

        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                let c = match board.get_cell((i, j)) {
                    BoardCell::Full(n) => char::from_digit(n, 10).unwrap(),
                    BoardCell::Empty => ' ',
                };
                self.term.print_char(i + 1, j + 1, RB_BOLD, Color::White, Color::Black, c);
            }
        }

        self.term.present();
    }

    pub fn poll_event(&self) -> EventResult {
        self.term.poll_event(false)
    }
}
