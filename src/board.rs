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
                if let Cell::Full(_) = self.get_cell((i, j)) {
                    for k in 0..j {
                        match self.get_cell((i, k)) {
                            Cell::Empty => {
                                self.move_cell((i, j), (i, k));
                                break;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn shift_right(&mut self) {
        for i in 0..BOARD_ROWS {
            for j in (0..BOARD_COLS - 1).rev() {
                if let Cell::Full(_) = self.get_cell((i, j)) {
                    for k in (j..BOARD_COLS).rev() {
                        match self.get_cell((i, k)) {
                            Cell::Empty => {
                                self.move_cell((i, j), (i, k));
                                break;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn shift_up(&mut self) {
        for j in 0..BOARD_COLS {
            for i in 1..BOARD_ROWS {
                if let Cell::Full(_) = self.get_cell((i, j)) {
                    for k in 0..i {
                        match self.get_cell((k, j)) {
                            Cell::Empty => {
                                self.move_cell((i, j), (k, j));
                                break;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn shift_down(&mut self) {
        for j in 0..BOARD_COLS {
            for i in (0..BOARD_ROWS - 1).rev() {
                if let Cell::Full(_) = self.get_cell((i, j)) {
                    for k in (i..BOARD_ROWS).rev() {
                        match self.get_cell((k, j)) {
                            Cell::Empty => {
                                self.move_cell((i, j), (k, j));
                                break;
                            },
                            _ => {}
                        }
                    }
                }
            }
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

    fn move_cell(&mut self, from: CellIdx, to: CellIdx) {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        self.board[to_row][to_col] = self.board[from_row][from_col];
        self.board[from_row][from_col] = Cell::Empty;
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