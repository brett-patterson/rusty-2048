use rand::{self, Rng};
use ndarray::{Array, Array2};

pub type CellIdx = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Full(u32),
}

enum ShiftResult {
    Shift,
    Combine,
    None,
}

pub struct Board {
    board: Array2<Cell>,
    nrows: usize,
    ncols: usize,
}

impl Board {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Board {
            board: Array::from_elem((nrows, ncols), Cell::Empty),
            nrows: nrows,
            ncols: ncols,
        }
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn get_cell(&self, idx: CellIdx) -> Option<Cell> {
        self.board.get(idx).map(|c| c.clone())
    }

    pub fn set_cell(&mut self, idx: CellIdx, value: Cell) {
        self.board[idx] = value;
    }

    pub fn fill_cell(&mut self) -> Option<CellIdx> {
        let next = self.next_empty_cell();

        if let Some(idx) = next {
            self.set_cell(idx, Cell::Full(Board::next_num()));
        }

        next
    }

    pub fn shift_left(&mut self) -> bool {
        let mut shifted = false;
        let mut boundary = 0;

        for i in 0..self.nrows {
            for j in 1..self.ncols {
                for k in boundary..j {
                    match self.shift_cell((i, j), (i, k)) {
                        ShiftResult::Combine => {
                            shifted = true;
                            boundary = k + 1;
                            break;
                        }
                        ShiftResult::Shift => {
                            shifted = true;
                            break;
                        }
                        ShiftResult::None => {}
                    }
                }
            }
        }
        shifted
    }

    pub fn shift_right(&mut self) -> bool {
        let mut shifted = false;
        let mut boundary = self.ncols;

        for i in 0..self.nrows {
            for j in (0..self.ncols - 1).rev() {
                for k in (j..boundary).rev() {
                    match self.shift_cell((i, j), (i, k)) {
                        ShiftResult::Combine => {
                            shifted = true;
                            boundary = k;
                            break;
                        }
                        ShiftResult::Shift => {
                            shifted = true;
                            break;
                        }
                        ShiftResult::None => {}
                    }
                }
            }
        }
        shifted
    }

    pub fn shift_up(&mut self) -> bool {
        let mut shifted = false;
        let mut boundary = 0;

        for j in 0..self.ncols {
            for i in 1..self.nrows {
                for k in boundary..i {
                    match self.shift_cell((i, j), (k, j)) {
                        ShiftResult::Combine => {
                            shifted = true;
                            boundary = k + 1;
                            break;
                        }
                        ShiftResult::Shift => {
                            shifted = true;
                            break;
                        }
                        ShiftResult::None => {}
                    }
                }
            }
        }
        shifted
    }

    pub fn shift_down(&mut self) -> bool {
        let mut shifted = false;
        let mut boundary = self.nrows;

        for j in 0..self.ncols {
            for i in (0..self.nrows - 1).rev() {
                for k in (i..boundary).rev() {
                    match self.shift_cell((i, j), (k, j)) {
                        ShiftResult::Combine => {
                            shifted = true;
                            boundary = k;
                            break;
                        }
                        ShiftResult::Shift => {
                            shifted = true;
                            break;
                        }
                        ShiftResult::None => {}
                    }
                }
            }
        }
        shifted
    }

    fn shift_cell(&mut self, from: CellIdx, to: CellIdx) -> ShiftResult {
        if let Some(Cell::Full(n)) = self.get_cell(from) {
            match self.get_cell(to) {
                Some(Cell::Empty) => {
                    self.set_cell(to, Cell::Full(n));
                    self.set_cell(from, Cell::Empty);
                    ShiftResult::Shift
                }
                Some(Cell::Full(other)) => {
                    if other == n {
                        self.set_cell(to, Cell::Full(n * 2));
                        self.set_cell(from, Cell::Empty);
                        ShiftResult::Combine
                    } else {
                        ShiftResult::None
                    }
                }
                None => ShiftResult::None,
            }
        } else {
            ShiftResult::None
        }
    }

    fn next_num() -> u32 {
        if rand::thread_rng().gen_weighted_bool(10) {
            4
        } else {
            2
        }
    }

    fn next_empty_cell(&self) -> Option<CellIdx> {
        rand::thread_rng().choose(self.empty_cells().as_slice()).map(|t| t.clone())
    }

    fn empty_cells(&self) -> Vec<CellIdx> {
        let mut result: Vec<CellIdx> = Vec::new();

        for i in 0..self.nrows {
            for j in 0..self.ncols {
                match self.get_cell((i, j)) {
                    Some(Cell::Empty) => result.push((i, j)),
                    _ => {}
                }
            }
        }

        result
    }
}