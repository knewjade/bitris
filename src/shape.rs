use std::array::IntoIter;
use std::slice::Iter;

use crate::{Orientation, Piece};
use crate::internal_macros::enum_display;

/// Converting to different data with additional data.
pub trait With<T> {
    type Output;

    fn with(self, arg: T) -> Self::Output;
}

/// A collection of piece shapes based on Tetrominoes.
/// ```
/// use bitris::Shape;
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
    #[default] T = 0,
    I = 1,
    O = 2,
    L = 3,
    J = 4,
    S = 5,
    Z = 6,
}

impl Shape {
    const VALUES: [Shape; 7] = [Shape::T, Shape::I, Shape::O, Shape::L, Shape::J, Shape::S, Shape::Z];

    /// ```
    /// use bitris::Shape;
    /// let mut iter = Shape::all_iter();
    /// assert_eq!(Some(&Shape::T), iter.next());
    /// assert_eq!(Some(&Shape::I), iter.next());
    /// assert_eq!(Some(&Shape::O), iter.next());
    /// assert_eq!(Some(&Shape::L), iter.next());
    /// assert_eq!(Some(&Shape::J), iter.next());
    /// assert_eq!(Some(&Shape::S), iter.next());
    /// assert_eq!(Some(&Shape::Z), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    #[inline]
    pub fn all_iter() -> Iter<'static, Shape> {
        Self::VALUES.iter()
    }

    /// ```
    /// use bitris::Shape;
    /// let mut iter = Shape::all_into_iter();
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
    pub fn all_into_iter() -> IntoIter<Shape, 7> {
        Self::VALUES.into_iter()
    }

    /// Returns the orientations with no change in shape.
    /// ```
    /// use bitris::{Orientation, Piece, Shape};
    /// use Orientation::*;
    /// assert_eq!(Shape::T.canonical_orientations(), &[North, East, South, West]);
    /// assert_eq!(Shape::L.canonical_orientations(), &[North, East, South, West]);
    /// assert_eq!(Shape::J.canonical_orientations(), &[North, East, South, West]);
    /// assert_eq!(Shape::I.canonical_orientations(), &[North, East]);
    /// assert_eq!(Shape::S.canonical_orientations(), &[North, East]);
    /// assert_eq!(Shape::Z.canonical_orientations(), &[North, East]);
    /// assert_eq!(Shape::O.canonical_orientations(), &[North]);
    /// ```
    #[inline]
    pub const fn canonical_orientations(self) -> &'static [Orientation] {
        use Orientation::*;
        match self {
            Shape::T | Shape::L | Shape::J => &[North, East, South, West],
            Shape::I | Shape::S | Shape::Z => &[North, East],
            Shape::O => &[North],
        }
    }

    /// Returns the orientations with change in shape.
    /// ```
    /// use bitris::{Orientation, Piece, Shape};
    /// use Orientation::*;
    /// assert_eq!(Shape::T.no_canonical_orientations(), &[]);
    /// assert_eq!(Shape::L.no_canonical_orientations(), &[]);
    /// assert_eq!(Shape::J.no_canonical_orientations(), &[]);
    /// assert_eq!(Shape::I.no_canonical_orientations(), &[South, West]);
    /// assert_eq!(Shape::S.no_canonical_orientations(), &[South, West]);
    /// assert_eq!(Shape::Z.no_canonical_orientations(), &[South, West]);
    /// assert_eq!(Shape::O.no_canonical_orientations(), &[East, South, West]);
    /// ```
    #[inline]
    pub const fn no_canonical_orientations(self) -> &'static [Orientation] {
        use Orientation::*;
        match self {
            Shape::T | Shape::L | Shape::J => &[],
            Shape::I | Shape::S | Shape::Z => &[South, West],
            Shape::O => &[East, South, West],
        }
    }
}

impl With<Orientation> for Shape {
    type Output = Piece;

    /// ```
    /// use bitris::{Orientation, piece, Shape, Piece, With};
    /// assert_eq!(Shape::T.with(Orientation::North), piece!(TN));
    /// ```
    fn with(self, orientation: Orientation) -> Self::Output {
        Piece::new(self, orientation)
    }
}

// A collection of errors that occur during converting to the shape.
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

    use crate::Shape;

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
