extern crate rusty_2048;

use rusty_2048::{Board, Cell, BOARD_ROWS, BOARD_COLS};

#[test]
fn make_board() {
    let board = Board::new();
    for i in 0..BOARD_ROWS {
        for j in 0..BOARD_COLS {
            assert_eq!(Cell::Empty, board.get_cell((i, j)));
        }
    }
}

#[test]
fn fill_cell() {
    let mut board = Board::new();
    match board.fill_cell() {
        Some(idx) => {
            match board.get_cell(idx) {
                Cell::Full(_) => {}
                Cell::Empty => assert!(false),
            }
        }
        None => assert!(false),
    }
}

#[test]
fn shift_left() {
    let mut board = Board::new();
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, 0), Cell::Full(2));

    assert!(board.shift_left());

    assert_eq!(Cell::Full(1), board.get_cell((0, 0)));
    assert_eq!(Cell::Full(2), board.get_cell((1, 0)));
    assert_eq!(Cell::Empty, board.get_cell((0, 1)));

    assert!(!board.shift_left());
}

#[test]
fn shift_right() {
    let mut board = Board::new();
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, BOARD_COLS - 1), Cell::Full(2));

    assert!(board.shift_right());

    assert_eq!(Cell::Full(1), board.get_cell((0, BOARD_COLS - 1)));
    assert_eq!(Cell::Full(2), board.get_cell((1, BOARD_COLS - 1)));
    assert_eq!(Cell::Empty, board.get_cell((0, 1)));

    assert!(!board.shift_right());
}

#[test]
fn shift_up() {
    let mut board = Board::new();
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, 0), Cell::Full(2));

    assert!(board.shift_up());

    assert_eq!(Cell::Full(1), board.get_cell((0, 1)));
    assert_eq!(Cell::Full(2), board.get_cell((0, 0)));
    assert_eq!(Cell::Empty, board.get_cell((1, 0)));

    assert!(!board.shift_up());
}

#[test]
fn shift_down() {
    let mut board = Board::new();
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((BOARD_ROWS - 1, 0), Cell::Full(2));

    assert!(board.shift_down());

    assert_eq!(Cell::Full(1), board.get_cell((BOARD_ROWS - 1, 1)));
    assert_eq!(Cell::Full(2), board.get_cell((BOARD_ROWS - 1, 0)));
    assert_eq!(Cell::Empty, board.get_cell((0, 1)));

    assert!(!board.shift_down());
}