use std::{fmt, ops};

use crate::{Rotate, Rotation};
use crate::boards::{BoardOp, Lines};
use crate::coordinates::{CcPosition, dd, Location, Offset};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::pieces::{Orientation, Piece, PieceBlocks, PieceBlocksFactory, Shape};
use crate::placements::{BlPlacement, PlacedPiece, TrPlacement};

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

    #[inline]
    pub fn to_piece_blocks(self) -> &'static PieceBlocks {
        PieceBlocksFactory.get(self.piece)
    }

    #[inline]
    pub const fn shape(&self) -> Shape {
        self.piece.shape
    }

    #[inline]
    pub const fn orientation(&self) -> Orientation {
        self.piece.orientation
    }

    /// Returns locations for each block.
    #[inline]
    pub fn locations(&self) -> [Location; 4] {
        let cc = self.position.to_location();
        self.to_piece_blocks().offsets.map(|offset| cc + offset)
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        board.set_all(self.locations().as_slice());
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        board.unset_all(self.locations().as_slice());
    }

    /// Returns rows on which the block exists.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IE).with(cc(1, 4)).using_rows(), Lines::new(0b111100));
    /// ```
    #[inline]
    pub fn using_rows(&self) -> Lines {
        self.locations().iter()
            .fold(Lines::blank(), |lines, location| {
                lines | Lines::new_at(location.y as u8)
            })
    }

    /// Returns a placed piece at the position where the interception was inserted for the placement.
    /// See `PlacedPiece::new_with_interception()`
    #[inline]
    pub fn with_interception(self, interception: Lines) -> PlacedPiece {
        self.to_bl_placement().with_interception(interception)
    }

    /// Fixes the orientation with no change in form.
    /// Always return the post-canonicalization piece.
    /// see also `Self::canonicalize()`.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IN).with(cc(3, 3)).canonical_or_self(), piece!(IN).with(cc(3, 3)).canonical_or_self());
    /// assert_eq!(piece!(IS).with(cc(4, 3)).canonical_or_self(), piece!(IN).with(cc(3, 3)).canonical_or_self());
    /// ```
    #[inline]
    pub fn canonical_or_self(self) -> Self {
        let bl: BlPlacement = self.into();
        bl.canonical_or_self().into()
    }

    /// Fixes the orientation with no change in form.
    /// If modification is not necessary, return None.
    /// see also `Piece::canonicalize()`.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IN).with(cc(3, 3)).canonical(), None);
    /// assert_eq!(piece!(IS).with(cc(4, 3)).canonical(), Some(piece!(IN).with(cc(3, 3))));
    /// ```
    #[inline]
    pub fn canonical(self) -> Option<Self> {
        let bl: BlPlacement = self.into();
        bl.canonical().map(Into::into)
    }

    /// Returns true if the two have of the same block locations.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert!(piece!(SN).with(cc(2, 3)).has_same_blocks_as(piece!(SS).with(cc(2, 4))));
    /// assert!(!piece!(SN).with(cc(2, 3)).has_same_blocks_as(piece!(SS).with(cc(2, 3))));
    /// ```
    #[inline]
    pub fn has_same_blocks_as(&self, placement: Self) -> bool {
        let bl: BlPlacement = self.into();
        bl.has_same_blocks_as(placement.into())
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

        let locations = self.to_piece_blocks().to_locations(self.position);
        board.set_all(&locations);
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
        self.to_piece_blocks().to_locations(below)
            .into_iter()
            .any(|location| !board.test_access(location) || board.is_occupied_at(location))
    }

    /// Returns true if no blocks are on the board in the placement positions.
    /// Whether the placement is landing or not is independent of the result.
    #[inline]
    pub fn is_in_free_space(&self, board: &impl BoardOp) -> bool {
        self.to_piece_blocks().to_locations(self.position)
            .into_iter()
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
    fn rotate(&self, rotation: Rotation) -> Self {
        Self::new(self.piece.rotate(rotation), self.position)
    }
}

impl ops::Add<Offset> for CcPlacement {
    type Output = CcPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
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
forward_ref_from!(CcPlacement, from CcPlacement);
forward_ref_from!(CcPlacement, from TrPlacement);

add_member_for_from!(BlPlacement, to_bl_placement, to CcPlacement);
add_member_for_from!(TrPlacement, to_tr_placement, to CcPlacement);


#[cfg(test)]
mod tests {
    use rstest::*;
    use rstest_reuse::*;
    use tinyvec::array_vec;

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn to_cc_placement_works() {
        let placement = CcPlacement {
            piece: piece!(LW),
            position: cc(2, 8),
        };
        assert_eq!(placement.piece, piece!(LW));
        assert_eq!(placement.position, cc(2, 8));
    }

    #[test]
    fn cc_from() {
        let piece = Piece::new(Shape::T, Orientation::North);

        let placement = piece.with(cc(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(4, 3)));

        let placement = piece.with(bl(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(5, 3)));
        assert_eq!(CcPlacement::from(&placement), piece.with(cc(5, 3)));
        assert_eq!(placement.to_cc_placement(), piece.with(cc(5, 3)));
        assert_eq!((&placement).to_cc_placement(), piece.with(cc(5, 3)));

        let placement = piece.with(tr(4, 3));
        assert_eq!(CcPlacement::from(placement), piece.with(cc(3, 2)));
        assert_eq!(CcPlacement::from(&placement), piece.with(cc(3, 2)));
        assert_eq!(placement.to_cc_placement(), piece.with(cc(3, 2)));
        assert_eq!((&placement).to_cc_placement(), piece.with(cc(3, 2)));
    }

    #[test]
    fn with_cc() {
        let piece = Piece::new(Shape::J, Orientation::North);
        let placement = piece.with(cc(4, 3));
        assert_eq!(placement.position, cc(4, 3));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(3, 3)));
        assert_eq!(placement.to_tr_placement(), piece.with(tr(5, 4)));

        let placement = &placement;
        assert_eq!(placement.position, cc(4, 3));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(3, 3)));
        assert_eq!(placement.to_tr_placement(), piece.with(tr(5, 4)));
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
    fn locations() {
        let locations = piece!(TN).with(cc(2, 3)).locations();
        assert_eq!(locations, [xy(1, 3), xy(2, 3), xy(3, 3), xy(2, 4)]);
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

    #[test]
    fn to_placed_piece() {
        assert_eq!(
            piece!(IE).with(cc(5, 4)).with_interception(Lines::new(0b01010000)),
            PlacedPiece::new(piece!(IE), 5, array_vec![2, 3, 5, 7]),
        );
    }
}
