use crate::internal_macros::enum_display;

/// Converting to the post-rotation item.
pub trait Rotate: Sized {
    type Item;

    #[inline]
    fn cw(&self) -> Self::Item {
        self.rotate(Rotation::Cw)
    }

    #[inline]
    fn ccw(&self) -> Self::Item {
        self.rotate(Rotation::Ccw)
    }

    #[inline]
    fn r180(&self) -> Self::Item {
        self.rotate(Rotation::R180)
    }

    /// Return the post-rotation item.
    fn rotate(&self, rotation: Rotation) -> Self::Item;
}

enum_display! { Rotation, has Cw,Ccw,R180 }

/// A collection of piece shapes based on Tetrominoes.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(Rotation::default(), Rotation::Cw);
/// assert_eq!(Rotation::Cw as i32, 0);
/// assert_eq!(Rotation::Ccw as i32, 1);
/// assert_eq!(Rotation::R180 as i32, 2);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Rotation {
    #[default]
    Cw = 0,
    Ccw = 1,
    R180 = 2,
}

impl Rotation {
    const VALUES: [Rotation; 3] = [Rotation::Cw, Rotation::Ccw, Rotation::R180];

    /// ```
    /// use bitris::prelude::*;
    /// let mut iter = Rotation::all_iter();
    /// assert_eq!(Some(Rotation::Cw), iter.next());
    /// assert_eq!(Some(Rotation::Ccw), iter.next());
    /// assert_eq!(Some(Rotation::R180), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    #[inline]
    pub fn all_iter() -> impl Iterator<Item = Rotation> {
        Self::VALUES.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn string() {
        use Rotation::*;
        assert_eq!(String::from("Cw"), Cw.to_string());
        assert_eq!(String::from("Ccw"), format!("{}", Ccw));
        assert_eq!(String::from("R180"), format!("{:?}", R180));
    }
}
