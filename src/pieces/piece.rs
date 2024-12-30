use std::fmt;

use crate::coordinates::{BlPosition, CcPosition, TrPosition};
use crate::internal_macros::forward_ref_from;
use crate::pieces::{Orientation, PieceBlocks, PieceBlocksFactory, Shape};
use crate::placements::{BlPlacement, CcPlacement, TrPlacement};
use crate::{Rotate, Rotation, With};

/// It has shape and orientation as a piece.
/// ```
/// use bitris::prelude::*;
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
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(TS).with(cc(5, 6)), CcPlacement { piece: piece!(TS), position: cc(5, 6) });
    /// ```
    fn with(self, position: CcPosition) -> Self::Output {
        CcPlacement::new(self, position)
    }
}

impl With<BlPosition> for Piece {
    type Output = BlPlacement;

    /// ```
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(piece!(TS).with(bl(5, 6)), BlPlacement { piece: piece!(TS), position: bl(5, 6) });
    /// ```
    fn with(self, position: BlPosition) -> Self::Output {
        BlPlacement::new(self, position)
    }
}

impl With<TrPosition> for Piece {
    type Output = TrPlacement;

    /// ```
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
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

    /// Fixes the orientation with no change in form.
    /// Always return the post-canonicalization piece.
    /// see also `Self::canonicalize()`.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Piece::new(Shape::I, Orientation::North).canonical_or_self(), Piece::new(Shape::I, Orientation::North));
    /// assert_eq!(Piece::new(Shape::I, Orientation::South).canonical_or_self(), Piece::new(Shape::I, Orientation::North));
    /// ```
    #[inline]
    pub fn canonical_or_self(&self) -> Piece {
        self.canonical().unwrap_or(*self)
    }

    /// Fixes the orientation with no change in form.
    /// If modification is not necessary, return None.
    /// * T or L or J => Not fixed
    /// * I or S or Z => Fixed to North or East
    /// * O => Fixed to North
    /// ```
    /// use bitris::prelude::*;
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
                South => Some(Piece {
                    shape: self.shape,
                    orientation: North,
                }),
                East => None,
                West => Some(Piece {
                    shape: self.shape,
                    orientation: East,
                }),
            },
            Shape::O => match self.orientation {
                North => None,
                South => Some(Piece {
                    shape: self.shape,
                    orientation: North,
                }),
                East => Some(Piece {
                    shape: self.shape,
                    orientation: North,
                }),
                West => Some(Piece {
                    shape: self.shape,
                    orientation: North,
                }),
            },
        }
    }

    /// Returns orientations having the same form.
    /// ```
    /// use bitris::prelude::*;
    ///
    /// use Shape::*;
    /// use Orientation::*;
    /// assert_eq!(Piece::new(T, North).orientations_having_same_form(), &[North]);
    /// assert_eq!(Piece::new(I, North).orientations_having_same_form(), &[North, South]);
    /// assert_eq!(Piece::new(O, North).orientations_having_same_form(), &[North, East, South, West]);
    /// ```
    #[inline]
    pub const fn orientations_having_same_form(&self) -> &'static [Orientation] {
        use Orientation::*;
        match self.shape {
            Shape::T | Shape::L | Shape::J => match self.orientation {
                North => &[North],
                East => &[East],
                South => &[South],
                West => &[West],
            },
            Shape::I | Shape::S | Shape::Z => match self.orientation {
                North | South => &[North, South],
                East | West => &[East, West],
            },
            Shape::O => &[North, East, South, West],
        }
    }

    /// Returns the height of piece.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Piece::new(Shape::I, Orientation::North).height(), 1);
    /// assert_eq!(Piece::new(Shape::O, Orientation::South).height(), 2);
    /// assert_eq!(Piece::new(Shape::S, Orientation::East).height(), 3);
    /// assert_eq!(Piece::new(Shape::I, Orientation::West).height(), 4);
    /// ```
    #[inline]
    pub const fn height(&self) -> u32 {
        use Shape::*;
        match self.orientation {
            Orientation::North | Orientation::South => match self.shape {
                I => 1,
                O | T | L | J | S | Z => 2,
            },
            Orientation::East | Orientation::West => match self.shape {
                I => 4,
                O => 2,
                T | L | J | S | Z => 3,
            },
        }
    }

    /// Returns the width of piece.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Piece::new(Shape::I, Orientation::West).width(), 1);
    /// assert_eq!(Piece::new(Shape::O, Orientation::East).width(), 2);
    /// assert_eq!(Piece::new(Shape::T, Orientation::South).width(), 3);
    /// assert_eq!(Piece::new(Shape::I, Orientation::North).width(), 4);
    /// ```
    #[inline]
    pub const fn width(&self) -> u32 {
        use Shape::*;
        match self.orientation {
            Orientation::North | Orientation::South => match self.shape {
                I => 4,
                O => 2,
                T | L | J | S | Z => 3,
            },
            Orientation::East | Orientation::West => match self.shape {
                I => 1,
                O | T | L | J | S | Z => 2,
            },
        }
    }

    /// Returns true if the two have of the same form.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert!(piece!(SN).has_same_form_as(piece!(SS)));
    /// assert!(piece!(OW).has_same_form_as(piece!(OE)));
    ///
    /// assert!(!piece!(TW).has_same_form_as(piece!(TE)));
    /// ```
    #[inline]
    pub fn has_same_form_as(&self, piece: Piece) -> bool {
        self.canonical_or_self() == piece.canonical_or_self()
    }

    /// Returns an iterator to make all pieces.
    /// ```
    /// use bitris::prelude::*;
    /// let vec: Vec<Piece> = Piece::all_iter().collect();
    /// assert_eq!(vec.len(), 4 * 7);
    /// ```
    #[inline]
    pub fn all_iter() -> impl Iterator<Item = Piece> {
        Shape::all_iter().flat_map(|shape| {
            Orientation::all_iter().map(move |orientation| shape.with(orientation))
        })
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
        self.shape.with(self.orientation.rotate(rotation))
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

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn string() {
        assert_eq!(String::from("T-North"), piece!(TN).to_string());
        assert_eq!(String::from("O-East"), format!("{}", piece!(OE)));
        assert_eq!(
            String::from("Piece { shape: Z, orientation: West }"),
            format!("{:?}", piece!(ZW))
        );
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
        assert_eq!(
            Piece::new(shape, South).canonical(),
            Some(Piece::new(shape, North))
        );
        assert_eq!(
            Piece::new(shape, West).canonical(),
            Some(Piece::new(shape, East))
        );
    }

    #[rstest]
    #[case(Shape::O)]
    fn piece_canonical_360(#[case] shape: Shape) {
        use Orientation::*;
        assert_eq!(Piece::new(shape, North).canonical(), None);
        assert_eq!(
            Piece::new(shape, East).canonical(),
            Some(Piece::new(shape, North))
        );
        assert_eq!(
            Piece::new(shape, South).canonical(),
            Some(Piece::new(shape, North))
        );
        assert_eq!(
            Piece::new(shape, West).canonical(),
            Some(Piece::new(shape, North))
        );
    }

    #[test]
    fn shape_canonical_orientations_and_piece_canonical_in_sync() {
        for piece in Piece::all_iter() {
            assert_eq!(
                piece.shape.canonical_pieces_iter().any(|it| it == piece),
                piece.canonical().is_none(),
            );
            assert_eq!(
                piece.shape.no_canonical_pieces_iter().any(|it| it == piece),
                piece.canonical().is_some(),
            );
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

    #[test]
    fn height_and_width() {
        for piece in Piece::all_iter() {
            let piece_blocks = piece.to_piece_blocks();
            assert_eq!(piece.height(), piece_blocks.height);
            assert_eq!(piece.width(), piece_blocks.width);
        }
    }
}
