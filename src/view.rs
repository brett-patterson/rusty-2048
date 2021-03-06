use std::char;

use rustbox::{RustBox, Color, Event, Key, RB_NORMAL, RB_BOLD};
use super::board::{Board, Cell};

const HELP: &'static str = "Use the arrow keys to shift cells. Press 'q' to quit.";

pub enum Action {
    Shift,
    NoShift,
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
        let nrows = board.nrows();
        let ncols = board.ncols();

        for i in 1..nrows + 1 {
            self.term.print_char(0, i, RB_BOLD, Color::White, Color::Black, '|');
            self.term.print_char(ncols + 1, i, RB_BOLD, Color::White, Color::Black, '|');
        }

        for i in 1..board.ncols() + 1 {
            self.term.print_char(i, 0, RB_BOLD, Color::White, Color::Black, '-');
            self.term.print_char(i, nrows + 1, RB_BOLD, Color::White, Color::Black, '-');
        }

        for i in 0..nrows {
            for j in 0..ncols {
                let c = match board.get_cell((i, j)) {
                    Some(Cell::Full(n)) => char::from_digit(n, 10).unwrap(),
                    Some(Cell::Empty) => ' ',
                    None => panic!("Display of invalid cell"),
                };
                self.term.print_char(j + 1, i + 1, RB_NORMAL, Color::White, Color::Black, c);
            }
        }

        self.term.print(0, nrows + 2, RB_NORMAL, Color::White, Color::Black, HELP);

        self.term.present();
    }

    pub fn handle_event(&self, board: &mut Board) -> Action {
        match self.term.poll_event(false) {
            Ok(Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => Action::End,
                    Key::Left => {
                        if board.shift_left() {
                            Action::Shift
                        } else {
                            Action::NoShift
                        }
                    }
                    Key::Right => {
                        if board.shift_right() {
                            Action::Shift
                        } else {
                            Action::NoShift
                        }
                    }
                    Key::Up => {
                        if board.shift_up() {
                            Action::Shift
                        } else {
                            Action::NoShift
                        }
                    }
                    Key::Down => {
                        if board.shift_down() {
                            Action::Shift
                        } else {
                            Action::NoShift
                        }
                    }
                    _ => Action::None,
                }
            }
            Err(e) => panic!("{}", e),
            _ => Action::None,
        }
    }
}
