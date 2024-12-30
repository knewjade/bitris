use std::{fmt, ops};

use tinyvec::ArrayVec;

use crate::boards::{BoardOp, Lines};
use crate::coordinates::{BlPosition, Location, Offset};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::pieces::{Orientation, Piece, PieceBlocks, PieceBlocksFactory, Shape};
use crate::placements::{CcPlacement, PlacedPiece, TrPlacement};
use crate::{Rotate, Rotation};

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
        self.to_cc_placement().locations()
    }

    /// Returns block locations of possible touch with the ground.
    /// Finds the y-coordinate of the lowest block in each x-coordinate.
    /// ```
    /// use tinyvec::ArrayVec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(
    ///     piece!(JS).with(bl(2, 3)).touching_locations().as_slice(),
    ///     ArrayVec::from([Location::new(2, 4), Location::new(3, 4), Location::new(4, 3)]).as_slice(),
    /// );
    /// assert_eq!(
    ///     piece!(SN).with(bl(4, 0)).touching_locations().as_slice(),
    ///     ArrayVec::from([Location::new(4, 0), Location::new(5, 0), Location::new(6, 1)]).as_slice(),
    /// );
    /// ```
    #[inline]
    pub fn touching_locations(&self) -> ArrayVec<[Location; 4]> {
        self.to_cc_placement().touching_locations()
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        self.to_cc_placement().set_all(board)
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        self.to_cc_placement().unset_all(board)
    }

    /// Returns rows on which the block exists.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IE).with(bl(1, 2)).using_rows(), Lines::new(0b111100));
    /// ```
    #[inline]
    pub fn using_rows(&self) -> Lines {
        self.to_cc_placement().using_rows()
    }

    /// Returns a placed piece at the position where the interception was inserted for the placement.
    /// See also `PlacedPiece::new_with_interception()`.
    #[inline]
    pub fn with_interception(self, interception: Lines) -> PlacedPiece {
        PlacedPiece::new_with_interception(self, interception)
    }

    /// Fixes the orientation with no change in form.
    /// Always return the post-canonicalization piece.
    /// see also `Self::canonicalize()`.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IN).with(bl(2, 3)).canonical_or_self(), piece!(IN).with(bl(2, 3)).canonical_or_self());
    /// assert_eq!(piece!(IS).with(bl(2, 3)).canonical_or_self(), piece!(IN).with(bl(2, 3)).canonical_or_self());
    /// ```
    #[inline]
    pub fn canonical_or_self(self) -> Self {
        Self {
            piece: self.piece.canonical_or_self(),
            position: self.position,
        }
    }

    /// Fixes the orientation with no change in form.
    /// If modification is not necessary, return None.
    /// see also `Piece::canonicalize()`.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(IN).with(bl(2, 3)).canonical(), None);
    /// assert_eq!(piece!(IS).with(bl(2, 3)).canonical(), Some(piece!(IN).with(bl(2, 3))));
    /// ```
    #[inline]
    pub fn canonical(self) -> Option<Self> {
        self.piece.canonical().map(|piece| Self {
            piece,
            position: self.position,
        })
    }

    /// Returns true if the two have of the same block locations.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert!(piece!(SN).with(bl(2, 3)).has_same_blocks_as(piece!(SS).with(bl(2, 3))));
    /// assert!(!piece!(SN).with(bl(2, 3)).has_same_blocks_as(piece!(SS).with(bl(2, 2))));
    /// ```
    #[inline]
    pub fn has_same_blocks_as(&self, placement: Self) -> bool {
        self.canonical_or_self() == placement.canonical_or_self()
    }

    /// Place blocks on the board according to the placement. No apply line clear.
    ///
    /// Returns true if the placement is successfully placed.
    /// Return false if the placement is "not landing" or "no free space in the board."
    #[inline]
    pub fn place_on(&self, board: &mut impl BoardOp) -> bool {
        self.to_cc_placement().place_on(board)
    }

    /// Execute `place_on()` and then clear lines.
    ///
    /// Returns the lines cleared if succeed.
    /// Returns None if failed to place.
    #[inline]
    pub fn place_on_and_clear_lines(&self, board: &mut impl BoardOp) -> Option<Lines> {
        self.to_cc_placement().place_on_and_clear_lines(board)
    }

    /// Returns true if the placement is in free space and is landing.
    #[inline]
    pub fn can_place_on(&self, board: &impl BoardOp) -> bool {
        self.to_cc_placement().can_place_on(board)
    }

    /// Returns true if one or more blocks are on the board below the placement positions.
    /// Whether the placement is in free space or not is independent of the result.
    #[inline]
    pub fn is_landing(&self, board: &impl BoardOp) -> bool {
        self.to_cc_placement().is_landing(board)
    }

    /// Returns true if no blocks are on the board in the placement positions.
    /// Whether the placement is landing or not is independent of the result.
    #[inline]
    pub fn is_in_free_space(&self, board: &impl BoardOp) -> bool {
        self.to_cc_placement().is_in_free_space(board)
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
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// let placement = piece!(IN).with(bl(3, 3));
    /// assert_eq!(placement.rotate(Rotation::Cw), piece!(IE).with(bl(4, 1)));
    /// ```
    #[inline]
    fn rotate(&self, rotation: Rotation) -> Self {
        self.to_cc_placement().rotate(rotation).to_bl_placement()
    }
}

impl ops::Add<Offset> for BlPlacement {
    type Output = BlPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
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
                lx: tr.position.rx - piece_blocks.width as i32 + 1,
                by: tr.position.ty - piece_blocks.height as i32 + 1,
            },
        }
    }
}

forward_ref_from!(BlPlacement, from BlPlacement);
forward_ref_from!(BlPlacement, from CcPlacement);
forward_ref_from!(BlPlacement, from TrPlacement);

add_member_for_from!(CcPlacement, to_cc_placement, to BlPlacement);
add_member_for_from!(TrPlacement, to_tr_placement, to BlPlacement);

#[cfg(test)]
mod tests {
    use rstest::*;
    use rstest_reuse::*;
    use tinyvec::array_vec;

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn to_bl_placement_works() {
        let placement = BlPlacement {
            piece: piece!(IE),
            position: bl(5, 3),
        };
        assert_eq!(placement.piece, piece!(IE));
        assert_eq!(placement.position, bl(5, 3));
    }

    #[test]
    fn bl_from() {
        let piece = Piece::new(Shape::T, Orientation::North);

        let placement = piece.with(bl(4, 3));
        assert_eq!(placement, piece.with(bl(4, 3)));

        let placement = piece.with(cc(4, 3));
        assert_eq!(BlPlacement::from(placement), piece.with(bl(3, 3)));
        assert_eq!(BlPlacement::from(&placement), piece.with(bl(3, 3)));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(3, 3)));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(3, 3)));

        let placement = piece.with(tr(4, 3));
        assert_eq!(BlPlacement::from(placement), piece.with(bl(2, 2)));
        assert_eq!(BlPlacement::from(&placement), piece.with(bl(2, 2)));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(2, 2)));
        assert_eq!(placement.to_bl_placement(), piece.with(bl(2, 2)));
    }

    #[test]
    fn with_bl() {
        let piece = Piece::new(Shape::L, Orientation::North);
        let placement = piece.with(bl(3, 3));
        assert_eq!(placement.position, bl(3, 3));
        assert_eq!(placement.to_cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.to_tr_placement(), piece.with(tr(5, 4)));

        let placement = &placement;
        assert_eq!(placement.position, bl(3, 3));
        assert_eq!(placement.to_cc_placement(), piece.with(cc(4, 3)));
        assert_eq!(placement.to_tr_placement(), piece.with(tr(5, 4)));
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
    fn locations() {
        let locations = piece!(TN).with(bl(1, 3)).locations();
        assert_eq!(locations, piece!(TN).with(cc(2, 3)).locations());
    }

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

        assert_eq!(
            os.with(bl(8, 0)).place_on_and_clear_lines(&mut board),
            Some(Lines::new(0b11))
        );
        assert_eq!(board.count_blocks(), 0);
    }

    #[test]
    fn to_placed_piece() {
        assert_eq!(
            piece!(SE)
                .with(bl(7, 1))
                .with_interception(Lines::new(0b010101111)),
            PlacedPiece::new(piece!(SE), 7, array_vec![6, 8, 9]),
        );
    }
}
