/// Board operations based on CC.
pub mod cc_ops {
    use crate::boards::BoardOp;
    use crate::placements::CcPlacement;

    /// See `CCPlacement::set_all()`
    pub fn set_all(board: &mut impl BoardOp, placement: CcPlacement) {
        placement.set_all(board);
    }

    /// See `CCPlacement::unset_all()`
    pub fn unset_all(board: &mut impl BoardOp, placement: CcPlacement) {
        placement.unset_all(board);
    }

    /// See `CCPlacement::place_on()`
    pub fn place(board: &mut impl BoardOp, placement: CcPlacement) {
        placement.place_on(board);
    }

    /// See `CCPlacement::place_on_and_clear_lines()`
    pub fn place_and_clear_lines(board: &mut impl BoardOp, placement: CcPlacement) {
        placement.place_on_and_clear_lines(board);
    }
}

/// Board operations based on BL.
pub mod bl_ops {
    use crate::boards::BoardOp;
    use crate::placements::BlPlacement;

    /// See `BLPlacement::set_all()`
    pub fn set_all(board: &mut impl BoardOp, placement: BlPlacement) {
        placement.set_all(board);
    }

    /// See `BLPlacement::unset_all()`
    pub fn unset_all(board: &mut impl BoardOp, placement: BlPlacement) {
        placement.unset_all(board);
    }

    /// See `BLPlacement::place_on()`
    pub fn place(board: &mut impl BoardOp, placement: BlPlacement) {
        placement.place_on(board);
    }

    /// See `BLPlacement::place_on_and_clear_lines()`
    pub fn place_and_clear_lines(board: &mut impl BoardOp, placement: BlPlacement) {
        placement.place_on_and_clear_lines(board);
    }
}

/// Board operations based on TR.
pub mod tr_ops {
    use crate::boards::BoardOp;
    use crate::placements::TrPlacement;

    /// See `TRPlacement::set_all()`
    pub fn set_all(board: &mut impl BoardOp, placement: TrPlacement) {
        placement.set_all(board);
    }

    /// See `TRPlacement::unset_all()`
    pub fn unset_all(board: &mut impl BoardOp, placement: TrPlacement) {
        placement.unset_all(board);
    }

    /// See `TRPlacement::place_on()`
    pub fn place(board: &mut impl BoardOp, placement: TrPlacement) {
        placement.place_on(board);
    }

    /// See `TRPlacement::place_on_and_clear_lines()`
    pub fn place_and_clear_lines(board: &mut impl BoardOp, placement: TrPlacement) {
        placement.place_on_and_clear_lines(board);
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use rstest_reuse::*;

    use crate::piece;
    use crate::prelude::*;

    #[fixture]
    pub fn board8() -> Board8 {
        Board8::blank()
    }

    #[fixture]
    pub fn board16() -> Board16 {
        Board16::blank()
    }

    #[fixture]
    pub fn board32() -> Board32 {
        Board32::blank()
    }

    #[fixture]
    pub fn board64() -> Board64 {
        Board64::blank()
    }

    #[template]
    #[rstest]
    #[case::board8(board8())]
    #[case::board16(board16())]
    #[case::board32(board32())]
    #[case::board64(board64())]
    fn all_boards(#[case] mut board: impl BoardOp) {}

    #[apply(all_boards)]
    fn cc_board_operators(mut board: impl BoardOp) {
        let on = piece!(ON);

        cc_ops::set_all(&mut board, on.with(cc(0, 0)));
        cc_ops::set_all(&mut board, on.with(cc(2, 0)));
        cc_ops::set_all(&mut board, on.with(cc(4, 0)));
        cc_ops::set_all(&mut board, on.with(cc(6, 0)));
        cc_ops::set_all(&mut board, on.with(cc(8, 0)));
        assert_eq!(board.count_blocks(), 20);

        cc_ops::unset_all(&mut board, on.with(cc(8, 0)));
        assert_eq!(board.count_blocks(), 16);

        cc_ops::place(&mut board, on.with(cc(8, 0)));
        assert_eq!(board.count_blocks(), 20);

        cc_ops::unset_all(&mut board, on.with(cc(8, 0)));
        assert_eq!(board.count_blocks(), 16);

        cc_ops::place_and_clear_lines(&mut board, on.with(cc(8, 0)));
        assert_eq!(board.count_blocks(), 0);
    }

    #[apply(all_boards)]
    fn bl_board_operators(mut board: impl BoardOp) {
        let on = piece!(OS);

        bl_ops::set_all(&mut board, on.with(bl(0, 0)));
        bl_ops::set_all(&mut board, on.with(bl(2, 0)));
        bl_ops::set_all(&mut board, on.with(bl(4, 0)));
        bl_ops::set_all(&mut board, on.with(bl(6, 0)));
        bl_ops::set_all(&mut board, on.with(bl(8, 0)));
        assert_eq!(board.count_blocks(), 20);

        bl_ops::unset_all(&mut board, on.with(bl(8, 0)));
        assert_eq!(board.count_blocks(), 16);

        bl_ops::place(&mut board, on.with(bl(8, 0)));
        assert_eq!(board.count_blocks(), 20);

        bl_ops::unset_all(&mut board, on.with(bl(8, 0)));
        assert_eq!(board.count_blocks(), 16);

        bl_ops::place_and_clear_lines(&mut board, on.with(bl(8, 0)));
        assert_eq!(board.count_blocks(), 0);
    }

    #[apply(all_boards)]
    fn tr_board_operators(mut board: impl BoardOp) {
        let on = piece!(OS);

        tr_ops::set_all(&mut board, on.with(tr(1, 1)));
        tr_ops::set_all(&mut board, on.with(tr(3, 1)));
        tr_ops::set_all(&mut board, on.with(tr(5, 1)));
        tr_ops::set_all(&mut board, on.with(tr(7, 1)));
        tr_ops::set_all(&mut board, on.with(tr(9, 1)));
        assert_eq!(board.count_blocks(), 20);

        tr_ops::unset_all(&mut board, on.with(tr(9, 1)));
        assert_eq!(board.count_blocks(), 16);

        tr_ops::place(&mut board, on.with(tr(9, 1)));
        assert_eq!(board.count_blocks(), 20);

        tr_ops::unset_all(&mut board, on.with(tr(9, 1)));
        assert_eq!(board.count_blocks(), 16);

        tr_ops::place_and_clear_lines(&mut board, on.with(tr(9, 1)));
        assert_eq!(board.count_blocks(), 0);
    }
}
