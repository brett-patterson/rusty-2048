use rand::{self, Rng};

pub const BOARD_ROWS: usize = 4;
pub const BOARD_COLS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum BoardCell {
    Empty,
    Full(u32),
}

pub struct Board {
    board: [[BoardCell; BOARD_ROWS]; BOARD_COLS],
}

impl Board {
    pub fn new() -> Self {
        Board { board: [[BoardCell::Empty; BOARD_COLS]; BOARD_ROWS] }
    }

    pub fn get_cell(&self, idx: (usize, usize)) -> BoardCell {
        let (row, col) = idx;
        self.board[row][col]
    }

    pub fn fill_cell(&mut self) -> Option<(usize, usize)> {
        let next = self.next_empty_cell();

        if let Some(idx) = next {
            self.set_cell(idx, BoardCell::Full(Board::next_num()));
        }

        next
    }

    fn next_num() -> u32 {
        if rand::thread_rng().gen_weighted_bool(10) {
            4
        } else {
            2
        }
    }

    fn set_cell(&mut self, idx: (usize, usize), value: BoardCell) {
        let (row, col) = idx;
        self.board[row][col] = value;
    }

    fn next_empty_cell(&self) -> Option<(usize, usize)> {
        rand::thread_rng().choose(self.empty_cells().as_slice()).map(|t| t.clone())
    }

    fn empty_cells(&self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();

        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                match self.get_cell((i, j)) {
                    BoardCell::Empty => result.push((i, j)),
                    _ => {}
                }
            }
        }

        result
    }
}