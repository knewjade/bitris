use std::{fmt, ops};

use crate::{BlPosition, BoardOp, CcPosition, dd, Lines, Location, Offset, Piece, Rotate, Rotation, ToPieceBlocks, TrPosition};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};

/// The position to be placed, based on the center of the piece.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CcPlacement {
    pub piece: Piece,
    pub position: CcPosition,
}

impl CcPlacement {
    #[inline]
    pub const fn new(piece: Piece, position: CcPosition) -> Self {
        Self { piece, position }
    }

    /// Returns locations for each block.
    #[inline]
    pub fn locations(&self) -> Vec<Location> {
        let cc = self.position.to_location();
        self.to_piece_blocks().offsets.iter()
            .map(|offset| cc + offset)
            .collect()
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        board.set_all(self.to_piece_blocks(), self.position);
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        board.unset_all(self.to_piece_blocks(), self.position);
    }

    /// Place blocks on the board according to the placement. No apply line clear.
    ///
    /// Returns true if the placement is successfully placed.
    /// Return false if the placement is "not landing" or "no free space in the board."
    #[inline]
    pub fn place_on(&self, board: &mut impl BoardOp) -> bool {
        if !self.can_place_on(board) {
            return false;
        }

        let cc = self.position.to_location();
        for offset in self.to_piece_blocks().offsets {
            board.set_at(cc + offset);
        }
        true
    }

    /// Execute `place_on()` and then clear lines.
    ///
    /// Returns the lines cleared if succeed.
    /// Returns None if failed to place.
    #[inline]
    pub fn place_on_and_clear_lines(&self, board: &mut impl BoardOp) -> Option<Lines> {
        let placed = self.place_on(board);
        if !placed {
            return None;
        }

        let lines = board.clear_lines();
        Some(lines)
    }

    /// Returns true if the placement is in free space and is landing.
    #[inline]
    pub fn can_place_on(&self, board: &impl BoardOp) -> bool {
        self.is_in_free_space(board) && self.is_landing(board)
    }

    /// Returns true if one or more blocks are on the board below the placement positions.
    /// Whether the placement is in free space or not is independent of the result.
    #[inline]
    pub fn is_landing(&self, board: &impl BoardOp) -> bool {
        let below = self.position + dd(0, -1);
        let below = below.to_location();
        self.to_piece_blocks().offsets.iter()
            .map(|offset| below + offset)
            .any(|location| !board.test_access(location) || board.is_occupied_at(location))
    }

    /// Returns true if no blocks are on the board in the placement positions.
    /// Whether the placement is landing or not is independent of the result.
    #[inline]
    pub fn is_in_free_space(&self, board: &impl BoardOp) -> bool {
        let cc = self.position.to_location();
        self.to_piece_blocks().offsets.iter()
            .map(|offset| cc + offset)
            .all(|location| board.test_access(location) && board.is_free_at(location))
    }
}

impl fmt::Display for CcPlacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Placement: {}, {}", self.piece, self.position)
    }
}

impl Rotate for CcPlacement {
    type Item = CcPlacement;

    #[inline]
    fn rotate(&self, rotation: Rotation) -> CcPlacement {
        CcPlacement::new(self.piece.rotate(rotation), self.position)
    }
}

impl ops::Add<Offset> for CcPlacement {
    type Output = CcPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        CcPlacement {
            piece: self.piece,
            position: self.position + rhs,
        }
    }
}

impl ops::AddAssign<Offset> for CcPlacement {
    fn add_assign(&mut self, rhs: Offset) {
        self.position += rhs;
    }
}

forward_ref_op! { CcPlacement, + Offset, = CcPlacement }
forward_ref_op! { CcPlacement, += Offset }

impl From<BlPlacement> for CcPlacement {
    fn from(bl: BlPlacement) -> Self {
        let piece_blocks = bl.to_piece_blocks();
        Self {
            piece: bl.piece,
            position: CcPosition {
                cx: bl.position.lx - piece_blocks.bottom_left.dx,
                cy: bl.position.by - piece_blocks.bottom_left.dy,
            },
        }
    }
}

impl From<TrPlacement> for CcPlacement {
    fn from(tr: TrPlacement) -> Self {
        let piece_blocks = tr.to_piece_blocks();
        Self {
            piece: tr.piece,
            position: CcPosition {
                cx: tr.position.rx - piece_blocks.top_right.dx,
                cy: tr.position.ty - piece_blocks.top_right.dy,
            },
        }
    }
}

forward_ref_from!(CcPlacement, from BlPlacement);
forward_ref_from!(CcPlacement, from TrPlacement);

add_member_for_from!(BlPlacement, bl_placement, to CcPlacement);
add_member_for_from!(TrPlacement, tr_placement, to CcPlacement);


/// The position to be placed, based on the bottom-left of the piece.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BlPlacement {
    pub piece: Piece,
    pub position: BlPosition,
}

impl BlPlacement {
    #[inline]
    pub const fn new(piece: Piece, position: BlPosition) -> Self {
        Self { piece, position }
    }

    /// Returns locations for each block.
    #[inline]
    pub fn locations(&self) -> Vec<Location> {
        self.cc_placement().locations()
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        self.cc_placement().set_all(board)
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        self.cc_placement().unset_all(board)
    }

    /// Place blocks on the board according to the placement. No apply line clear.
    ///
    /// Returns true if the placement is successfully placed.
    /// Return false if the placement is "not landing" or "no free space in the board."
    #[inline]
    pub fn place_on(&self, board: &mut impl BoardOp) -> bool {
        self.cc_placement().place_on(board)
    }

    /// Execute `place_on()` and then clear lines.
    ///
    /// Returns the lines cleared if succeed.
    /// Returns None if failed to place.
    #[inline]
    pub fn place_on_and_clear_lines(&self, board: &mut impl BoardOp) -> Option<Lines> {
        self.cc_placement().place_on_and_clear_lines(board)
    }

    /// Returns true if the placement is in free space and is landing.
    #[inline]
    pub fn can_place_on(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().can_place_on(board)
    }

    /// Returns true if one or more blocks are on the board below the placement positions.
    /// Whether the placement is in free space or not is independent of the result.
    #[inline]
    pub fn is_landing(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().is_landing(board)
    }

    /// Returns true if no blocks are on the board in the placement positions.
    /// Whether the placement is landing or not is independent of the result.
    #[inline]
    pub fn is_in_free_space(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().is_in_free_space(board)
    }
}

impl fmt::Display for BlPlacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Placement: {}, {}", self.piece, self.position)
    }
}

impl Rotate for BlPlacement {
    type Item = BlPlacement;

    /// It rotates around the center of the piece Therefore, the position may change.
    ///
    /// Note that no kick occurs.
    /// ```
    /// use bitris::{piece, bl, Shape, Rotation, Piece, Orientation, Rotate, With};
    /// let placement = piece!(IN).with(bl(3, 3));
    /// assert_eq!(placement.rotate(Rotation::Cw), piece!(IE).with(bl(4, 1)));
    /// ```
    #[inline]
    fn rotate(&self, rotation: Rotation) -> BlPlacement {
        self.cc_placement().rotate(rotation).bl_placement()
    }
}

impl ops::Add<Offset> for BlPlacement {
    type Output = BlPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        BlPlacement {
            piece: self.piece,
            position: self.position + rhs,
        }
    }
}

impl ops::AddAssign<Offset> for BlPlacement {
    fn add_assign(&mut self, rhs: Offset) {
        self.position += rhs;
    }
}

forward_ref_op! { BlPlacement, + Offset, = BlPlacement }
forward_ref_op! { BlPlacement, += Offset }

impl From<CcPlacement> for BlPlacement {
    fn from(cc: CcPlacement) -> Self {
        let piece_blocks = cc.to_piece_blocks();
        Self {
            piece: cc.piece,
            position: BlPosition {
                lx: cc.position.cx + piece_blocks.bottom_left.dx,
                by: cc.position.cy + piece_blocks.bottom_left.dy,
            },
        }
    }
}

impl From<TrPlacement> for BlPlacement {
    fn from(tr: TrPlacement) -> Self {
        let piece_blocks = tr.to_piece_blocks();
        Self {
            piece: tr.piece,
            position: BlPosition {
                lx: tr.position.rx - piece_blocks.width + 1,
                by: tr.position.ty - piece_blocks.height + 1,
            },
        }
    }
}

forward_ref_from!(BlPlacement, from CcPlacement);
forward_ref_from!(BlPlacement, from TrPlacement);

add_member_for_from!(CcPlacement, cc_placement, to BlPlacement);
add_member_for_from!(TrPlacement, tr_placement, to BlPlacement);


/// The position to be placed, based on the top-right of the piece.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TrPlacement {
    pub piece: Piece,
    pub position: TrPosition,
}

impl TrPlacement {
    #[inline]
    pub const fn new(piece: Piece, position: TrPosition) -> Self {
        Self { piece, position }
    }

    /// Returns locations for each block.
    #[inline]
    pub fn locations(&self) -> Vec<Location> {
        self.cc_placement().locations()
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        self.cc_placement().set_all(board)
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        self.cc_placement().unset_all(board)
    }

    /// Place blocks on the board according to the placement. No apply line clear.
    ///
    /// Returns true if the placement is successfully placed.
    /// Return false if the placement is "not landing" or "no free space in the board."
    #[inline]
    pub fn place_on(&self, board: &mut impl BoardOp) -> bool {
        self.cc_placement().place_on(board)
    }

    /// Execute `place_on()` and then clear lines.
    ///
    /// Returns the lines cleared if succeed.
    /// Returns None if failed to place.
    #[inline]
    pub fn place_on_and_clear_lines(&self, board: &mut impl BoardOp) -> Option<Lines> {
        self.cc_placement().place_on_and_clear_lines(board)
    }

    /// Returns true if the placement is in free space and is landing.
    #[inline]
    pub fn can_place_on(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().can_place_on(board)
    }

    /// Returns true if one or more blocks are on the board below the placement positions.
    /// Whether the placement is in free space or not is independent of the result.
    #[inline]
    pub fn is_landing(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().is_landing(board)
    }

    /// Returns true if no blocks are on the board in the placement positions.
    /// Whether the placement is landing or not is independent of the result.
    #[inline]
    pub fn is_in_free_space(&self, board: &impl BoardOp) -> bool {
        self.cc_placement().is_in_free_space(board)
    }
}

impl fmt::Display for TrPlacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Placement: {}, {}", self.piece, self.position)
    }
}

impl Rotate for TrPlacement {
    type Item = TrPlacement;

    /// It rotates around the center of the piece Therefore, the position may change.
    ///
    /// Note that no kick occurs.
    /// ```
    /// use bitris::{piece, tr, Shape, Rotation, Piece, Orientation, Rotate, With};
    /// let placement = piece!(IN).with(tr(5, 5));
    /// assert_eq!(placement.rotate(Rotation::Cw), piece!(IE).with(tr(3, 6)));
    /// ```
    #[inline]
    fn rotate(&self, rotation: Rotation) -> TrPlacement {
        self.cc_placement().rotate(rotation).tr_placement()
    }
}

impl ops::Add<Offset> for TrPlacement {
    type Output = TrPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        TrPlacement {
            piece: self.piece,
            position: self.position + rhs,
        }
    }
}

impl ops::AddAssign<Offset> for TrPlacement {
    fn add_assign(&mut self, rhs: Offset) {
        self.position += rhs;
    }
}

forward_ref_op! { TrPlacement, + Offset, = TrPlacement }
forward_ref_op! { TrPlacement, += Offset }

impl From<CcPlacement> for TrPlacement {
    fn from(cc: CcPlacement) -> Self {
        let piece_blocks = cc.to_piece_blocks();
        Self {
            piece: cc.piece,
            position: TrPosition {
                rx: cc.position.cx + piece_blocks.top_right.dx,
                ty: cc.position.cy + piece_blocks.top_right.dy,
            },
        }
    }
}

impl From<BlPlacement> for TrPlacement {
    fn from(br: BlPlacement) -> Self {
        let piece_blocks = br.to_piece_blocks();
        Self {
            piece: br.piece,
            position: TrPosition::new(
                br.position.lx + piece_blocks.width - 1,
                br.position.by + piece_blocks.height - 1,
            ),
        }
    }
}

forward_ref_from!(TrPlacement, from CcPlacement);
forward_ref_from!(TrPlacement, from BlPlacement);

add_member_for_from!(BlPlacement, bl_placement, to TrPlacement);
add_member_for_from!(CcPlacement, cc_placement, to TrPlacement);


#[cfg(test)]
mod tests {
    use rstest::*;
    use rstest_reuse::*;

    use crate::{Board16, Board32, Board64, Board8, Orientation, piece, Shape, With, xy};
    use crate::placements::*;
    use crate::positions::*;

    #[test]
    fn cc_placement_works() {
        let placement = CcPlacement {
            piece: piece!(LW),
            position: cc(2, 8),
        };
        assert_eq!(placement.piece, piece!(LW));
        assert_eq!(placement.position, cc(2, 8));
    }

    #[test]
    fn bl_placement_works() {
        let placement = BlPlacement {
            piece: piece!(IE),
            position: bl(5, 3),
        };
        assert_eq!(placement.piece, piece!(IE));
        assert_eq!(placement.position, bl(5, 3));
    }

    #[test]
    fn tr_placement_works() {
        let placement = TrPlacement {
            piece: piece!(LW),
            position: tr(2, 8),
        };
        assert_eq!(placement.piece, piece!(LW));
        assert_eq!(placement.position, tr(2, 8));
    }

    #[test]
    fn cc_from() {
        let piece = Piece::new(Shape::T, Orientation::North);

        let placement = piece.with(cc(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(4, 3)));

        let placement = piece.with(bl(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(5, 3)));
        assert_eq!(CcPlacement::from(&placement), piece.with(cc(5, 3)));
        assert_eq!(placement.cc_placement(), piece.with(cc(5, 3)));
        assert_eq!((&placement).cc_placement(), piece.with(cc(5, 3)));

        let placement = piece.with(tr(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(3, 2)));
        assert_eq!(CcPlacement::from(&placement), piece.with(cc(3, 2)));
        assert_eq!(placement.cc_placement(), piece.with(cc(3, 2)));
        assert_eq!((&placement).cc_placement(), piece.with(cc(3, 2)));
    }

    #[test]
    fn bl_from() {
        let piece = Piece::new(Shape::T, Orientation::North);

        let placement = piece.with(bl(4, 3));
        assert_eq!(BlPlacement::from(placement), piece.with(bl(4, 3)));

        let placement = piece.with(cc(4, 3));
        assert_eq!(BlPlacement::from(placement), piece.with(bl(3, 3)));
        assert_eq!(BlPlacement::from(&placement), piece.with(bl(3, 3)));
        assert_eq!(placement.bl_placement(), piece.with(bl(3, 3)));
        assert_eq!((&placement).bl_placement(), piece.with(bl(3, 3)));

        let placement = piece.with(tr(4, 3));
        assert_eq!(BlPlacement::from(placement), piece.with(bl(2, 2)));
        assert_eq!(BlPlacement::from(&placement), piece.with(bl(2, 2)));
        assert_eq!(placement.bl_placement(), piece.with(bl(2, 2)));
        assert_eq!((&placement).bl_placement(), piece.with(bl(2, 2)));
    }

    #[test]
    fn tr_from() {
        let piece = Piece::new(Shape::T, Orientation::North);

        let placement = piece.with(tr(4, 3));
        assert_eq!(TrPlacement::from(placement), piece.with(tr(4, 3)));

        let placement = piece.with(cc(4, 3));
        assert_eq!(TrPlacement::from(placement), piece.with(tr(5, 4)));
        assert_eq!(TrPlacement::from(&placement), piece.with(tr(5, 4)));
        assert_eq!(placement.tr_placement(), piece.with(tr(5, 4)));
        assert_eq!((&placement).tr_placement(), piece.with(tr(5, 4)));

        let placement = piece.with(bl(4, 3));
        assert_eq!(TrPlacement::from(placement), piece.with(tr(6, 4)));
        assert_eq!(TrPlacement::from(&placement), piece.with(tr(6, 4)));
        assert_eq!(placement.tr_placement(), piece.with(tr(6, 4)));
        assert_eq!((&placement).tr_placement(), piece.with(tr(6, 4)));
    }

    #[test]
    fn with_cc() {
        let piece = Piece::new(Shape::J, Orientation::North);
        let placement = piece.with(cc(4, 3));
        assert_eq!(placement.position, cc(4, 3));
        assert_eq!(placement.bl_placement(), piece.with(bl(3, 3)));
        assert_eq!(placement.tr_placement(), piece.with(tr(5, 4)));

        let placement = &placement;
        assert_eq!(placement.position, cc(4, 3));
        assert_eq!(placement.bl_placement(), piece.with(bl(3, 3)));
        assert_eq!(placement.tr_placement(), piece.with(tr(5, 4)));
    }

    #[test]
    fn with_bl() {
        let piece = Piece::new(Shape::L, Orientation::North);
        let placement = piece.with(bl(3, 3));
        assert_eq!(placement.position, bl(3, 3));
        assert_eq!(placement.cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.tr_placement(), piece.with(tr(5, 4)));

        let placement = &placement;
        assert_eq!(placement.position, bl(3, 3));
        assert_eq!(placement.cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.tr_placement(), piece.with(tr(5, 4)));
    }

    #[test]
    fn with_tr() {
        let piece = Piece::new(Shape::S, Orientation::North);
        let placement = piece.with(tr(5, 4));
        assert_eq!(placement.position, tr(5, 4));
        assert_eq!(placement.cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.bl_placement(), piece.with(bl(3, 3)));

        let placement = &placement;
        assert_eq!(placement.position, tr(5, 4));
        assert_eq!(placement.cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.bl_placement(), piece.with(bl(3, 3)));
    }

    #[test]
    fn cc_rotate() {
        let from = piece!(TN).with(cc(3, 4));
        let cw = piece!(TE).with(cc(3, 4));
        let ccw = piece!(TW).with(cc(3, 4));
        assert_eq!(from.rotate(Rotation::Cw), cw);
        assert_eq!(from.rotate(Rotation::Ccw), ccw);
    }

    #[test]
    fn bl_rotate() {
        let from = piece!(TN).with(bl(3, 4));
        let cw = piece!(TE).with(bl(4, 3));
        let ccw = piece!(TW).with(bl(3, 3));
        assert_eq!(from.rotate(Rotation::Cw), cw);
        assert_eq!(from.rotate(Rotation::Ccw), ccw);
    }

    #[test]
    fn tr_rotate() {
        let from = piece!(TN).with(tr(3, 4));
        let cw = piece!(TE).with(tr(3, 4));
        let ccw = piece!(TW).with(tr(2, 4));
        assert_eq!(from.rotate(Rotation::Cw), cw);
        assert_eq!(from.rotate(Rotation::Ccw), ccw);
    }

    #[test]
    fn locations() {
        let locations = piece!(TN).with(cc(2, 3)).locations();
        assert_eq!(locations, vec![xy(1, 3), xy(2, 3), xy(3, 3), xy(2, 4)]);

        let locations = piece!(TN).with(bl(1, 3)).locations();
        assert_eq!(locations, piece!(TN).with(cc(2, 3)).locations());

        let locations = piece!(TN).with(tr(3, 4)).locations();
        assert_eq!(locations, piece!(TN).with(cc(2, 3)).locations());
    }

    #[fixture]
    pub fn board8() -> Board8 { Board8::blank() }

    #[fixture]
    pub fn board16() -> Board16 { Board16::blank() }

    #[fixture]
    pub fn board32() -> Board32 { Board32::blank() }

    #[fixture]
    pub fn board64() -> Board64 { Board64::blank() }

    #[template]
    #[rstest]
    #[case::board8(board8())]
    #[case::board16(board16())]
    #[case::board32(board32())]
    #[case::board64(board64())]
    fn all_boards(#[case] mut board: impl BoardOp) {}

    #[apply(all_boards)]
    fn cc_set_all_and_unset_all(mut board: impl BoardOp) {
        piece!(TN).with(cc(5, 5)).set_all(&mut board);
        assert_eq!(board.count_blocks(), 4);

        piece!(TN).with(cc(5, 5)).unset_all(&mut board);
        assert_eq!(board.count_blocks(), 0);
    }

    #[apply(all_boards)]
    fn cc_is_landing(board: impl BoardOp) {
        assert!(!piece!(TN).with(cc(5, 1)).is_landing(&board));
        assert!(piece!(TN).with(cc(5, 0)).is_landing(&board));

        assert!(piece!(TN).with(cc(-1, 4)).is_landing(&board));
        assert!(piece!(TN).with(cc(10, 4)).is_landing(&board));
        assert!(piece!(TN).with(cc(4, -1)).is_landing(&board));
        assert!(piece!(TN).with(cc(4, 99)).is_landing(&board));
    }

    #[apply(all_boards)]
    fn cc_is_in_free_space(mut board: impl BoardOp) {
        assert!(piece!(TN).with(cc(5, 5)).is_in_free_space(&board));
        board.set_at(xy(5, 5));
        assert!(!piece!(TN).with(cc(5, 5)).is_in_free_space(&board));

        assert!(!piece!(TN).with(cc(-1, 4)).is_in_free_space(&board));
        assert!(!piece!(TN).with(cc(10, 4)).is_in_free_space(&board));
        assert!(!piece!(TN).with(cc(4, -1)).is_in_free_space(&board));
        assert!(!piece!(TN).with(cc(4, 99)).is_in_free_space(&board));
    }

    #[apply(all_boards)]
    fn cc_can_place_on(mut board: impl BoardOp) {
        assert!(!piece!(TN).with(cc(5, 5)).can_place_on(&board));
        board.set_at(xy(5, 4));
        assert!(piece!(TN).with(cc(5, 5)).can_place_on(&board));
        board.set_at(xy(5, 5));
        assert!(!piece!(TN).with(cc(5, 5)).can_place_on(&board));

        assert!(!piece!(TN).with(cc(-1, 4)).can_place_on(&board));
        assert!(!piece!(TN).with(cc(10, 4)).can_place_on(&board));
        assert!(!piece!(TN).with(cc(4, -1)).can_place_on(&board));
        assert!(!piece!(TN).with(cc(4, 99)).can_place_on(&board));
    }

    #[apply(all_boards)]
    fn cc_place_on(mut board: impl BoardOp) {
        assert!(!piece!(TN).with(cc(1, 1)).place_on(&mut board));
        assert_eq!(board.count_blocks(), 0);

        assert!(!piece!(TN).with(cc(-1, 4)).place_on(&mut board));
        assert!(!piece!(TN).with(cc(10, 4)).place_on(&mut board));
        assert!(!piece!(TN).with(cc(4, -1)).place_on(&mut board));
        assert!(!piece!(TN).with(cc(4, 99)).place_on(&mut board));
        assert_eq!(board.count_blocks(), 0);

        assert!(piece!(ON).with(cc(0, 0)).place_on(&mut board));
        assert!(piece!(ON).with(cc(2, 0)).place_on(&mut board));
        assert!(piece!(ON).with(cc(4, 0)).place_on(&mut board));
        assert!(piece!(ON).with(cc(6, 0)).place_on(&mut board));
        assert!(piece!(ON).with(cc(8, 0)).place_on(&mut board));
        assert_eq!(board.count_blocks(), 20);

        assert!(!piece!(TN).with(cc(1, 0)).place_on(&mut board));
        assert_eq!(board.count_blocks(), 20);
    }

    #[apply(all_boards)]
    fn cc_place_on_and_clear_lines(mut board: impl BoardOp) {
        assert_eq!(piece!(ON).with(cc(0, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0)));
        assert_eq!(piece!(ON).with(cc(2, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0)));
        assert_eq!(piece!(ON).with(cc(4, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0)));
        assert_eq!(piece!(ON).with(cc(6, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0)));
        assert_eq!(piece!(ON).with(cc(8, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0b11)));
        assert_eq!(board.count_blocks(), 0);
    }

    #[apply(all_boards)]
    fn bl_board_operators(mut board: impl BoardOp) {
        let os = piece!(OS);

        os.with(bl(0, 0)).set_all(&mut board);
        os.with(bl(0, 0)).unset_all(&mut board);
        assert_eq!(board.count_blocks(), 0);

        assert!(os.with(bl(0, 0)).place_on(&mut board));
        assert!(os.with(bl(2, 0)).place_on(&mut board));
        assert!(os.with(bl(4, 0)).place_on(&mut board));
        assert!(os.with(bl(6, 0)).place_on(&mut board));

        assert!(!os.with(bl(7, 0)).can_place_on(&mut board));
        assert!(os.with(bl(8, 0)).can_place_on(&mut board));

        assert!(!os.with(bl(7, 0)).is_in_free_space(&mut board));
        assert!(os.with(bl(8, 0)).is_in_free_space(&mut board));

        assert!(!os.with(bl(8, 1)).is_landing(&mut board));
        assert!(os.with(bl(8, 0)).is_landing(&mut board));

        assert_eq!(os.with(bl(8, 0)).place_on_and_clear_lines(&mut board), Some(Lines::new(0b11)));
        assert_eq!(board.count_blocks(), 0);
    }

    #[apply(all_boards)]
    fn tr_board_operators(mut board: impl BoardOp) {
        let os = piece!(OS);

        os.with(tr(1, 1)).set_all(&mut board);
        os.with(tr(1, 1)).unset_all(&mut board);
        assert_eq!(board.count_blocks(), 0);

        assert!(os.with(tr(1, 1)).place_on(&mut board));
        assert!(os.with(tr(3, 1)).place_on(&mut board));
        assert!(os.with(tr(5, 1)).place_on(&mut board));
        assert!(os.with(tr(7, 1)).place_on(&mut board));

        assert!(!os.with(tr(8, 1)).can_place_on(&mut board));
        assert!(os.with(tr(9, 1)).can_place_on(&mut board));

        assert!(!os.with(tr(8, 1)).is_in_free_space(&mut board));
        assert!(os.with(tr(9, 1)).is_in_free_space(&mut board));

        assert!(!os.with(tr(9, 2)).is_landing(&mut board));
        assert!(os.with(tr(9, 1)).is_landing(&mut board));

        assert_eq!(os.with(tr(9, 1)).place_on_and_clear_lines(&mut board), Some(Lines::new(0b11)));
        assert_eq!(board.count_blocks(), 0);
    }
}
