extern crate rusty_2048;

use rusty_2048::{Board, Cell};

#[test]
fn make_board() {
    let board = Board::new(4, 4);
    for i in 0..board.nrows() {
        for j in 0..board.ncols() {
            assert_eq!(Some(Cell::Empty), board.get_cell((i, j)));
        }
    }
}

#[test]
fn fill_cell() {
    let mut board = Board::new(4, 4);
    match board.fill_cell() {
        Some(idx) => {
            match board.get_cell(idx) {
                Some(Cell::Full(_)) => {}
                Some(Cell::Empty) |
                None => assert!(false),
            }
        }
        None => assert!(false),
    }
}

#[test]
fn shift_left() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, 0), Cell::Full(2));

    assert!(board.shift_left());

    assert_eq!(Some(Cell::Full(1)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((1, 0)));
    assert_eq!(Some(Cell::Empty), board.get_cell((0, 1)));

    assert!(!board.shift_left());
}

#[test]
fn shift_right() {
    let mut board = Board::new(4, 4);
    let ncols = board.ncols();

    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, ncols - 1), Cell::Full(2));

    assert!(board.shift_right());

    assert_eq!(Some(Cell::Full(1)), board.get_cell((0, ncols - 1)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((1, ncols - 1)));
    assert_eq!(Some(Cell::Empty), board.get_cell((0, 1)));

    assert!(!board.shift_right());
}

#[test]
fn shift_up() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((1, 0), Cell::Full(2));

    assert!(board.shift_up());

    assert_eq!(Some(Cell::Full(1)), board.get_cell((0, 1)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Empty), board.get_cell((1, 0)));

    assert!(!board.shift_up());
}

#[test]
fn shift_down() {
    let mut board = Board::new(4, 4);
    let nrows = board.nrows();

    board.set_cell((0, 1), Cell::Full(1));
    board.set_cell((nrows - 1, 0), Cell::Full(2));

    assert!(board.shift_down());

    assert_eq!(Some(Cell::Full(1)), board.get_cell((nrows - 1, 1)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((nrows - 1, 0)));
    assert_eq!(Some(Cell::Empty), board.get_cell((0, 1)));

    assert!(!board.shift_down());
}

#[test]
fn combine() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 0), Cell::Full(2));
    board.set_cell((0, 1), Cell::Full(2));
    board.set_cell((1, 0), Cell::Full(2));
    board.set_cell((1, 1), Cell::Full(4));

    assert!(board.shift_left());

    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((1, 0)));
    assert_eq!(Some(Cell::Full(4)), board.get_cell((1, 1)));

    board = Board::new(4, 4);
    board.set_cell((0, 0), Cell::Full(2));
    board.set_cell((0, 1), Cell::Full(2));
    board.set_cell((0, 2), Cell::Full(2));
    board.set_cell((0, 3), Cell::Full(2));

    assert!(board.shift_left());
    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 1)));
}

#[test]
fn no_shift() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 0), Cell::Full(2));
    board.set_cell((1, 0), Cell::Full(2));
    board.set_cell((2, 0), Cell::Full(2));

    assert!(!board.shift_left());
    assert_eq!(Some(Cell::Full(2)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((1, 0)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((2, 0)));
}

#[test]
fn no_combine() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 0), Cell::Full(4));
    board.set_cell((0, 1), Cell::Full(2));

    assert!(!board.shift_left());
    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(2)), board.get_cell((0, 1)));
}

#[test]
fn no_double_combine() {
    let mut board = Board::new(4, 4);
    board.set_cell((0, 0), Cell::Full(2));
    board.set_cell((0, 1), Cell::Full(2));
    board.set_cell((0, 2), Cell::Full(4));

    assert!(board.shift_left());
    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 0)));
    assert_eq!(Some(Cell::Full(4)), board.get_cell((0, 1)));
}