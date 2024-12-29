use crate::internal_macros::enum_display;
use crate::pieces::{Orientation, Piece};
use crate::With;

/// A collection of piece shapes based on Tetrominoes.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(Shape::default(), Shape::T);
/// assert_eq!(Shape::T as i32, 0);
/// assert_eq!(Shape::I as i32, 1);
/// assert_eq!(Shape::O as i32, 2);
/// assert_eq!(Shape::L as i32, 3);
/// assert_eq!(Shape::J as i32, 4);
/// assert_eq!(Shape::S as i32, 5);
/// assert_eq!(Shape::Z as i32, 6);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Shape {
    #[default]
    T = 0,
    I = 1,
    O = 2,
    L = 3,
    J = 4,
    S = 5,
    Z = 6,
}

impl Shape {
    const VALUES: [Shape; 7] = [
        Shape::T,
        Shape::I,
        Shape::O,
        Shape::L,
        Shape::J,
        Shape::S,
        Shape::Z,
    ];

    /// ```
    /// use bitris::prelude::*;
    /// let mut iter = Shape::all_iter();
    /// assert_eq!(Some(Shape::T), iter.next());
    /// assert_eq!(Some(Shape::I), iter.next());
    /// assert_eq!(Some(Shape::O), iter.next());
    /// assert_eq!(Some(Shape::L), iter.next());
    /// assert_eq!(Some(Shape::J), iter.next());
    /// assert_eq!(Some(Shape::S), iter.next());
    /// assert_eq!(Some(Shape::Z), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    #[inline]
    pub fn all_iter() -> impl Iterator<Item = Shape> {
        Self::VALUES.into_iter()
    }

    /// Returns an iterator to make all pieces with no change in form.
    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(Shape::T.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::T.with(North), Shape::T.with(East), Shape::T.with(South), Shape::T.with(West)]);
    /// assert_eq!(Shape::L.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::L.with(North), Shape::L.with(East), Shape::L.with(South), Shape::L.with(West)]);
    /// assert_eq!(Shape::J.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::J.with(North), Shape::J.with(East), Shape::J.with(South), Shape::J.with(West)]);
    /// assert_eq!(Shape::I.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::I.with(North), Shape::I.with(East)]);
    /// assert_eq!(Shape::S.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::S.with(North), Shape::S.with(East)]);
    /// assert_eq!(Shape::Z.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::Z.with(North), Shape::Z.with(East)]);
    /// assert_eq!(Shape::O.canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::O.with(North)]);
    /// ```
    #[inline]
    pub fn canonical_pieces_iter(self) -> impl Iterator<Item = Piece> {
        use Orientation::*;
        let slice: &[Orientation] = match self {
            Shape::T | Shape::L | Shape::J => &[North, East, South, West],
            Shape::I | Shape::S | Shape::Z => &[North, East],
            Shape::O => &[North],
        };
        slice.iter().map(move |&orientation| self.with(orientation))
    }

    /// Returns an iterator to make all pieces with change in form.
    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(Shape::T.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![]);
    /// assert_eq!(Shape::L.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![]);
    /// assert_eq!(Shape::J.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![]);
    /// assert_eq!(Shape::I.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::I.with(South), Shape::I.with(West)]);
    /// assert_eq!(Shape::S.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::S.with(South), Shape::S.with(West)]);
    /// assert_eq!(Shape::Z.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::Z.with(South), Shape::Z.with(West)]);
    /// assert_eq!(Shape::O.no_canonical_pieces_iter().collect::<Vec<Piece>>(), vec![Shape::O.with(East), Shape::O.with(South), Shape::O.with(West)]);
    /// ```
    #[inline]
    pub fn no_canonical_pieces_iter(self) -> impl Iterator<Item = Piece> {
        use Orientation::*;
        let slice: &[Orientation] = match self {
            Shape::T | Shape::L | Shape::J => &[],
            Shape::I | Shape::S | Shape::Z => &[South, West],
            Shape::O => &[East, South, West],
        };
        slice.iter().map(move |&orientation| self.with(orientation))
    }

    /// Returns an iterator to make all pieces with change in form.
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// use Shape::*;
    /// use Orientation::*;
    /// assert_eq!(
    ///     Vec::from_iter(T.all_pieces_iter()),
    ///     vec![T.with(North), T.with(East), T.with(South), T.with(West)],
    /// );
    /// ```
    #[inline]
    pub fn all_pieces_iter(self) -> impl Iterator<Item = Piece> {
        Orientation::all_iter().map(move |orientation| self.with(orientation))
    }
}

impl With<Orientation> for Shape {
    type Output = Piece;

    /// ```
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(Shape::T.with(Orientation::North), piece!(TN));
    /// ```
    fn with(self, orientation: Orientation) -> Self::Output {
        Piece::new(self, orientation)
    }
}

/// A collection of errors that occur during converting to the shape.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ShapeTryFromError {
    InvalidValue(usize),
}

impl TryFrom<usize> for Shape {
    type Error = ShapeTryFromError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Shape::T),
            1 => Ok(Shape::I),
            2 => Ok(Shape::O),
            3 => Ok(Shape::L),
            4 => Ok(Shape::J),
            5 => Ok(Shape::S),
            6 => Ok(Shape::Z),
            _ => Err(ShapeTryFromError::InvalidValue(value)),
        }
    }
}

enum_display! { Shape, has T,I,O,L,J,S,Z }

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use crate::prelude::*;

    #[test]
    fn string() {
        use Shape::*;
        assert_eq!(String::from("T"), T.to_string());
        assert_eq!(String::from("L"), format!("{:?}", L));
        assert_eq!(String::from("Z"), format!("{}", Z));
    }

    #[test]
    fn eq() {
        use Shape::*;
        assert_eq!(T, T);
        assert_ne!(T, S);
    }

    #[test]
    fn ord() {
        use Shape::*;
        assert!(T < S);
        assert!((S < T).not());
        assert!((T < T).not());
    }
}
