use std::str::FromStr;

use bitris::prelude::*;

fn main() {
    // Make boards. Select the bit width.
    let board8: Board8 = Board8::blank();
    let board16: Board16 = Board16::blank();
    let board32: Board32 = Board32::blank();
    let board64: Board64 = Board64::blank();

    // Depending on the bit width, the ceiling height will vary.
    assert_eq!(board8.ceiling(), 8);
    assert_eq!(board16.ceiling(), 16);
    assert_eq!(board32.ceiling(), 32);
    assert_eq!(board64.ceiling(), 64);

    // Boards can be created from strings.
    let board64: Board64 = Board64::from_str("
            ..........
            ####....##
            ####...###
            ####..####
            ####...###
        ").expect("Failed to create a board");
    assert_eq!(board64.count_blocks(), 28);
    assert_eq!(board64.well_top(), 4);

    // If you want to extend the bit width, you can use Into to convert.
    let board32: Board32 = Board8::blank().into();
    assert_eq!(board32.ceiling(), 32);

    // If you want to shrink the bit width, you must explicitly convert them.
    let mut board64 = Board64::blank();
    board64.set_at(xy(0, 8));
    assert!(board64.ceiling() == 64 && board64.count_blocks() == 1);
    let board8: Board8 = Board8::shrink_from(board64);
    assert!(board8.ceiling() == 8 && board8.count_blocks() == 0);

    // Set and unset a block. Manipulation of the board changes itself (it means mutable).
    // ..........
    // ..........
    // ..........
    // ..#.......
    // ..........
    let mut board = Board64::blank();
    board.set_at(xy(2, 1));
    assert!(board.is_occupied_at(xy(2, 1)));
    assert!(!board.is_free_at(xy(2, 1)));

    board.unset_at(xy(2, 1));
    assert!(!board.is_occupied_at(xy(2, 1)));
    assert!(board.is_free_at(xy(2, 1)));
}
