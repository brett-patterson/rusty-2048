use std::char;

use rustbox::{RustBox, Color, Event, Key, RB_NORMAL, RB_BOLD};
use super::board::{Board, Cell, BOARD_ROWS, BOARD_COLS};

const HELP_TEXT: &'static str = "Use the arrow keys to shift cells. Press 'q' to quit.";

pub enum Action {
    Shift,
    End,
    None,
}

pub struct View {
    term: RustBox,
}

impl View {
    pub fn new() -> Self {
        View { term: RustBox::init(Default::default()).unwrap() }
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
                    Cell::Full(n) => char::from_digit(n, 10).unwrap(),
                    Cell::Empty => ' ',
                };
                self.term.print_char(j + 1, i + 1, RB_NORMAL, Color::White, Color::Black, c);
            }
        }

        self.term.print(0, BOARD_ROWS + 2, RB_NORMAL, Color::White, Color::Black, HELP_TEXT);

        self.term.present();
    }

    pub fn handle_event(&self, board: &mut Board) -> Action {
         match self.term.poll_event(false) {
            Ok(Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => Action::End,
                    Key::Left => {
                        board.shift_left();
                        Action::Shift
                    },
                    Key::Right => {
                        board.shift_right();
                        Action::Shift
                    },
                    Key::Up => {
                        board.shift_up();
                        Action::Shift
                    },
                    Key::Down => {
                        board.shift_down();
                        Action::Shift
                    },
                    _ => Action::None
                }
            }
            Err(e) => panic!("{}", e),
            _ => Action::None
        }
    }
}
