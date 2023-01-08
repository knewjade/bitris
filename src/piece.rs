use std::fmt;

use crate::{BlPlacement, BlPosition, CcPlacement, CcPosition, Orientation, PieceBlocks, PieceBlocksFactory, Rotate, Rotation, Shape, TrPlacement, TrPosition, With};
use crate::internal_macros::forward_ref_from;

/// It has shape and orientation as a piece.
/// ```
/// use bitris::{Orientation, Piece, Shape};
/// assert_eq!(Piece::default(), Piece { shape: Shape::T, orientation: Orientation::North });
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Piece {
    pub shape: Shape,
    pub orientation: Orientation,
}

impl With<CcPosition> for Piece {
    type Output = CcPlacement;

    /// ```
    /// use bitris::{cc, CcPlacement, Orientation, Piece, piece, Shape, With};
    /// assert_eq!(piece!(TS).with(cc(5, 6)), CcPlacement { piece: piece!(TS), position: cc(5, 6) });
    /// ```
    fn with(self, position: CcPosition) -> Self::Output {
        CcPlacement::new(self, position)
    }
}

impl With<BlPosition> for Piece {
    type Output = BlPlacement;

    /// ```
    /// use bitris::{bl, BlPlacement, Orientation, Piece, piece, Shape, With};
    /// assert_eq!(piece!(TS).with(bl(5, 6)), BlPlacement { piece: piece!(TS), position: bl(5, 6) });
    /// ```
    fn with(self, position: BlPosition) -> Self::Output {
        BlPlacement::new(self, position)
    }
}

impl With<TrPosition> for Piece {
    type Output = TrPlacement;

    /// ```
    /// use bitris::{tr, TrPlacement, Orientation, Piece, piece, Shape, With};
    /// assert_eq!(piece!(TS).with(tr(5, 6)), TrPlacement { piece: piece!(TS), position: tr(5, 6) });
    /// ```
    #[inline]
    fn with(self, position: TrPosition) -> Self::Output {
        TrPlacement::new(self, position)
    }
}

impl Piece {
    #[inline]
    pub const fn new(shape: Shape, orientation: Orientation) -> Self {
        Self { shape, orientation }
    }

    #[inline]
    pub fn to_piece_blocks(self) -> &'static PieceBlocks {
        PieceBlocksFactory.get(self)
    }

    /// Fixes the orientation with no change in shape.
    /// Always return the post-canonicalization piece.
    /// see also `Self::canonicalize()`
    /// ```
    /// use bitris::{Orientation, Piece, Shape};
    /// assert_eq!(Piece::new(Shape::I, Orientation::North).canonical_or_self(), Piece::new(Shape::I, Orientation::North));
    /// assert_eq!(Piece::new(Shape::I, Orientation::South).canonical_or_self(), Piece::new(Shape::I, Orientation::North));
    /// ```
    #[inline]
    pub fn canonical_or_self(&self) -> Piece {
        self.canonical().unwrap_or(*self)
    }

    /// Fixes the orientation with no change in shape.
    /// If modification is not necessary, return None.
    /// * T or L or J => Not fixed
    /// * I or S or Z => Fixed to North or East
    /// * O => Fixed to North
    /// ```
    /// use bitris::{Orientation, Piece, Shape};
    /// assert_eq!(Piece::new(Shape::I, Orientation::North).canonical(), None);
    /// assert_eq!(Piece::new(Shape::I, Orientation::South).canonical(), Some(Piece::new(Shape::I, Orientation::North)));
    /// ```
    #[inline]
    pub const fn canonical(&self) -> Option<Piece> {
        use Orientation::*;
        match self.shape {
            Shape::T | Shape::L | Shape::J => None,
            Shape::I | Shape::S | Shape::Z => match self.orientation {
                North => None,
                South => Some(Piece { shape: self.shape, orientation: North }),
                East => None,
                West => Some(Piece { shape: self.shape, orientation: East }),
            },
            Shape::O => match self.orientation {
                North => None,
                South => Some(Piece { shape: self.shape, orientation: North }),
                East => Some(Piece { shape: self.shape, orientation: North }),
                West => Some(Piece { shape: self.shape, orientation: North }),
            },
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.shape, self.orientation)
    }
}

impl Rotate for Piece {
    type Item = Piece;

    #[inline]
    fn rotate(&self, rotation: Rotation) -> Piece {
        Piece::new(self.shape, self.orientation.rotate(rotation))
    }
}

impl From<CcPlacement> for Piece {
    fn from(placement: CcPlacement) -> Self {
        placement.piece
    }
}

forward_ref_from!(Piece, from CcPlacement);

impl From<BlPlacement> for Piece {
    fn from(placement: BlPlacement) -> Self {
        placement.piece
    }
}

forward_ref_from!(Piece, from BlPlacement);

impl From<TrPlacement> for Piece {
    fn from(placement: TrPlacement) -> Self {
        placement.piece
    }
}

forward_ref_from!(Piece, from TrPlacement);

impl From<&PieceBlocks> for Piece {
    #[inline]
    fn from(value: &PieceBlocks) -> Self {
        value.piece
    }
}


#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::*;

    #[test]
    fn string() {
        assert_eq!(String::from("T-North"), piece!(TN).to_string());
        assert_eq!(String::from("O-East"), format!("{}", piece!(OE)));
        assert_eq!(String::from("Piece { shape: Z, orientation: West }"), format!("{:?}", piece!(ZW)));
    }

    #[rstest]
    #[case(Shape::T)]
    #[case(Shape::L)]
    #[case(Shape::J)]
    fn piece_canonical_90(#[case] shape: Shape) {
        use Orientation::*;
        assert_eq!(Piece::new(shape, North).canonical(), None);
        assert_eq!(Piece::new(shape, East).canonical(), None);
        assert_eq!(Piece::new(shape, South).canonical(), None);
        assert_eq!(Piece::new(shape, West).canonical(), None);
    }

    #[rstest]
    #[case(Shape::I)]
    #[case(Shape::S)]
    #[case(Shape::Z)]
    fn piece_canonical_180(#[case] shape: Shape) {
        use Orientation::*;
        assert_eq!(Piece::new(shape, North).canonical(), None);
        assert_eq!(Piece::new(shape, East).canonical(), None);
        assert_eq!(Piece::new(shape, South).canonical(), Some(Piece::new(shape, North)));
        assert_eq!(Piece::new(shape, West).canonical(), Some(Piece::new(shape, East)));
    }

    #[rstest]
    #[case(Shape::O)]
    fn piece_canonical_360(#[case] shape: Shape) {
        use Orientation::*;
        assert_eq!(Piece::new(shape, North).canonical(), None);
        assert_eq!(Piece::new(shape, East).canonical(), Some(Piece::new(shape, North)));
        assert_eq!(Piece::new(shape, South).canonical(), Some(Piece::new(shape, North)));
        assert_eq!(Piece::new(shape, West).canonical(), Some(Piece::new(shape, North)));
    }

    #[test]
    fn shape_canonical_orientations_and_piece_canonical_in_sync() {
        for shape in Shape::all_into_iter() {
            for orientation in Orientation::all_into_iter() {
                assert_eq!(
                    shape.canonical_orientations().contains(&orientation),
                    Piece::new(shape, orientation).canonical() == None,
                );
                assert_eq!(
                    shape.no_canonical_orientations().contains(&orientation),
                    Piece::new(shape, orientation).canonical() != None,
                );
            }
        }
    }

    #[test]
    fn from() {
        let piece = Piece::new(Shape::T, Orientation::North);
        assert_eq!(Piece::from(piece.with(cc(5, 5))), piece);
        assert_eq!(Piece::from(piece.with(bl(5, 5))), piece);
        assert_eq!(Piece::from(piece.with(tr(5, 5))), piece);

        assert_eq!(piece.with(cc(5, 5)).piece, piece);
        assert_eq!(piece.with(bl(5, 5)).piece, piece);
        assert_eq!(piece.with(tr(5, 5)).piece, piece);

        let piece = &Piece::new(Shape::O, Orientation::East);
        assert_eq!(Piece::from(piece.with(cc(5, 5))), *piece);
        assert_eq!(Piece::from(piece.with(bl(5, 5))), *piece);
        assert_eq!(Piece::from(piece.with(tr(5, 5))), *piece);

        assert_eq!(piece.with(cc(5, 5)).piece, *piece);
        assert_eq!(piece.with(bl(5, 5)).piece, *piece);
        assert_eq!(piece.with(tr(5, 5)).piece, *piece);
    }
}
