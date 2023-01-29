use std::{fmt, ops};

use crate::{Rotate, Rotation};
use crate::boards::{BoardOp, Lines};
use crate::coordinates::{Location, Offset, TrPosition};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::pieces::{Orientation, Piece, PieceBlocks, PieceBlocksFactory, Shape};
use crate::placements::{BlPlacement, CcPlacement, PlacedPiece};

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
    /// assert_eq!(piece!(IE).with(tr(1, 5)).using_rows(), Lines::new(0b111100));
    /// ```
    #[inline]
    pub fn using_rows(&self) -> Lines {
        self.to_cc_placement().using_rows()
    }

    /// Returns a placed piece at the position where the interception was inserted for the placement.
    /// See also `PlacedPiece::new_with_interception()`.
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
    /// assert_eq!(piece!(IN).with(tr(5, 3)).canonical_or_self(), piece!(IN).with(tr(5, 3)).canonical_or_self());
    /// assert_eq!(piece!(IS).with(tr(5, 3)).canonical_or_self(), piece!(IN).with(tr(5, 3)).canonical_or_self());
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
    /// assert_eq!(piece!(IN).with(tr(6, 3)).canonical(), None);
    /// assert_eq!(piece!(IS).with(tr(6, 3)).canonical(), Some(piece!(IN).with(tr(6, 3))));
    /// ```
    #[inline]
    pub fn canonical(self) -> Option<Self> {
        self.piece.canonical().map(|piece| {
            Self {
                piece,
                position: self.position,
            }
        })
    }

    /// Returns true if the two have of the same block locations.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert!(piece!(SN).with(tr(2, 3)).has_same_blocks_as(piece!(SS).with(tr(2, 3))));
    /// assert!(!piece!(SN).with(tr(2, 3)).has_same_blocks_as(piece!(SS).with(tr(2, 2))));
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
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// let placement = piece!(IN).with(tr(5, 5));
    /// assert_eq!(placement.rotate(Rotation::Cw), piece!(IE).with(tr(3, 6)));
    /// ```
    #[inline]
    fn rotate(&self, rotation: Rotation) -> Self {
        self.to_cc_placement().rotate(rotation).to_tr_placement()
    }
}

impl ops::Add<Offset> for TrPlacement {
    type Output = TrPlacement;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
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
                br.position.lx + piece_blocks.width as i32 - 1,
                br.position.by + piece_blocks.height as i32 - 1,
            ),
        }
    }
}

forward_ref_from!(TrPlacement, from CcPlacement);
forward_ref_from!(TrPlacement, from BlPlacement);
forward_ref_from!(TrPlacement, from TrPlacement);

add_member_for_from!(BlPlacement, to_bl_placement, to TrPlacement);
add_member_for_from!(CcPlacement, to_cc_placement, to TrPlacement);


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

    #[test]
    fn to_placed_piece() {
        assert_eq!(
            piece!(TW).with(tr(5, 5)).with_interception(Lines::new(0b01010000)),
            PlacedPiece::new(piece!(TW), 4, array_vec![3, 5, 7]),
        );
    }
}
