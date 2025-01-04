use crate::internal_macros::enum_display;
use crate::{Rotate, Rotation};

/// A collection of the direction that piece is facing.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(Orientation::default(), Orientation::North);
/// assert_eq!(Orientation::North as i32, 0);
/// assert_eq!(Orientation::East as i32, 1);
/// assert_eq!(Orientation::South as i32, 2);
/// assert_eq!(Orientation::West as i32, 3);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Orientation {
    #[default]
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Orientation {
    const VALUES: [Orientation; 4] = [
        Orientation::North,
        Orientation::East,
        Orientation::South,
        Orientation::West,
    ];

    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// let mut iter = Orientation::all_iter();
    /// assert_eq!(Some(North), iter.next());
    /// assert_eq!(Some(East), iter.next());
    /// assert_eq!(Some(South), iter.next());
    /// assert_eq!(Some(West), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    #[inline]
    pub fn all_iter() -> impl Iterator<Item = Orientation> {
        Self::VALUES.into_iter()
    }
}

impl Rotate for Orientation {
    type Item = Orientation;

    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(North.cw(), East);
    /// assert_eq!(East.cw(), South);
    /// assert_eq!(South.cw(), West);
    /// assert_eq!(West.cw(), North);
    /// ```
    #[inline]
    fn cw(&self) -> Orientation {
        use Orientation::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(North.ccw(), West);
    /// assert_eq!(East.ccw(), North);
    /// assert_eq!(South.ccw(), East);
    /// assert_eq!(West.ccw(), South);
    /// ```
    #[inline]
    fn ccw(&self) -> Orientation {
        use Orientation::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(North.r180(), South);
    /// assert_eq!(East.r180(), West);
    /// assert_eq!(South.r180(), North);
    /// assert_eq!(West.r180(), East);
    /// ```
    #[inline]
    fn r180(&self) -> Orientation {
        use Orientation::*;
        match self {
            North => South,
            West => East,
            South => North,
            East => West,
        }
    }

    /// ```
    /// use bitris::prelude::*;
    /// use Orientation::*;
    /// assert_eq!(North.rotate(Rotation::Cw), East);
    /// assert_eq!(North.rotate(Rotation::Ccw), West);
    /// assert_eq!(North.rotate(Rotation::R180), South);
    /// ```
    #[inline]
    fn rotate(&self, rotation: Rotation) -> Orientation {
        match rotation {
            Rotation::Cw => self.cw(),
            Rotation::Ccw => self.ccw(),
            Rotation::R180 => self.r180(),
        }
    }
}

enum_display! { Orientation, has North,East,South,West }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn string() {
        use Orientation::*;
        assert_eq!(String::from("North"), North.to_string());
        assert_eq!(String::from("South"), format!("{}", South));
        assert_eq!(String::from("West"), format!("{:?}", West));
    }
}
