use rand::{self, Rng};

pub const BOARD_ROWS: usize = 4;
pub const BOARD_COLS: usize = 4;

pub type CellIdx = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Full(u32),
}

pub struct Board {
    board: [[Cell; BOARD_ROWS]; BOARD_COLS],
}

impl Board {
    pub fn new() -> Self {
        Board { board: [[Cell::Empty; BOARD_COLS]; BOARD_ROWS] }
    }

    pub fn get_cell(&self, idx: CellIdx) -> Cell {
        let (row, col) = idx;
        self.board[row][col]
    }

    pub fn fill_cell(&mut self) -> Option<CellIdx> {
        let next = self.next_empty_cell();

        if let Some(idx) = next {
            self.set_cell(idx, Cell::Full(Board::next_num()));
        }

        next
    }

    pub fn shift_left(&mut self) {
        for i in 0..BOARD_ROWS {
            for j in 1..BOARD_COLS {
                for k in 0..j {
                    if self.handle_shift((i, j), (i, k)) {
                        break;
                    }
                }
            }
        }
    }

    pub fn shift_right(&mut self) {
        for i in 0..BOARD_ROWS {
            for j in (0..BOARD_COLS - 1).rev() {
                for k in (j..BOARD_COLS).rev() {
                    if self.handle_shift((i, j), (i, k)) {
                        break;
                    }
                }
            }
        }
    }

    pub fn shift_up(&mut self) {
        for j in 0..BOARD_COLS {
            for i in 1..BOARD_ROWS {
                for k in 0..i {
                    if self.handle_shift((i, j), (k, j)) {
                        break;
                    }
                }
            }
        }
    }

    pub fn shift_down(&mut self) {
        for j in 0..BOARD_COLS {
            for i in (0..BOARD_ROWS - 1).rev() {
                for k in (i..BOARD_ROWS).rev() {
                    if self.handle_shift((i, j), (k, j)) {
                        break;
                    }
                }
            }
        }
    }

    fn handle_shift(&mut self, from: CellIdx, to: CellIdx) -> bool {
        if let Cell::Full(n) = self.get_cell(from) {
            match self.get_cell(to) {
                Cell::Empty => {
                    let from_cell = self.get_cell(from);
                    self.set_cell(to, from_cell);
                    self.set_cell(from, Cell::Empty);
                    true
                }
                Cell::Full(other) => {
                    if other == n {
                        self.set_cell(to, Cell::Full(n * 2));
                        self.set_cell(from, Cell::Empty);
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    fn next_num() -> u32 {
        if rand::thread_rng().gen_weighted_bool(10) {
            4
        } else {
            2
        }
    }

    fn set_cell(&mut self, idx: CellIdx, value: Cell) {
        let (row, col) = idx;
        self.board[row][col] = value;
    }

    fn next_empty_cell(&self) -> Option<CellIdx> {
        rand::thread_rng().choose(self.empty_cells().as_slice()).map(|t| t.clone())
    }

    fn empty_cells(&self) -> Vec<CellIdx> {
        let mut result: Vec<CellIdx> = Vec::new();

        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                match self.get_cell((i, j)) {
                    Cell::Empty => result.push((i, j)),
                    _ => {}
                }
            }
        }

        result
    }
}