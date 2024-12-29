use std::{fmt, ops};

use crate::coordinates::{BlPosition, CcPosition, Offset, TrPosition};
use crate::internal_macros::{forward_ref_from, forward_ref_op};

/// An absolute point on the board.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(Location::default(), Location { x: 0, y: 0 });
/// assert_eq!(xy(1, -2), Location { x: 1, y: -2 });
/// assert_eq!(format!("{}", Location { x: 1, y: -2 }), "xy (1, -2)");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xy ({}, {})", self.x, self.y)
    }
}

impl ops::Add<Offset> for Location {
    type Output = Location;

    fn add(self, rhs: Offset) -> Self::Output {
        Location {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl ops::AddAssign<Offset> for Location {
    fn add_assign(&mut self, rhs: Offset) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

forward_ref_op! { Location, + Offset, = Location }
forward_ref_op! { Location, += Offset }

impl From<CcPosition> for Location {
    fn from(cc: CcPosition) -> Self {
        Location { x: cc.cx, y: cc.cy }
    }
}

forward_ref_from!(Location, from CcPosition);

impl From<BlPosition> for Location {
    fn from(bl: BlPosition) -> Self {
        Location { x: bl.lx, y: bl.by }
    }
}

forward_ref_from!(Location, from BlPosition);

impl From<TrPosition> for Location {
    fn from(tr: TrPosition) -> Self {
        Location { x: tr.rx, y: tr.ty }
    }
}

forward_ref_from!(Location, from TrPosition);

/// An shortcut for `Location { x, y }`.
#[inline(always)]
pub const fn xy(x: i32, y: i32) -> Location {
    Location { x, y }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn operators() {
        assert_eq!(xy(1, 2) + dd(-2, 3), xy(-1, 5));

        let mut position = xy(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, xy(-1, 2));
    }

    #[test]
    fn from() {
        let position = cc(5, 5);
        assert_eq!(Location::from(position), xy(5, 5));
        assert_eq!(Location::from(&position), xy(5, 5));
        assert_eq!(position.to_location(), xy(5, 5));
        assert_eq!((&position).to_location(), xy(5, 5));

        let position = bl(5, 5);
        assert_eq!(Location::from(position), xy(5, 5));
        assert_eq!(Location::from(&position), xy(5, 5));
        assert_eq!(position.to_location(), xy(5, 5));
        assert_eq!((&position).to_location(), xy(5, 5));

        let position = tr(5, 5);
        assert_eq!(Location::from(position), xy(5, 5));
        assert_eq!(Location::from(&position), xy(5, 5));
        assert_eq!(position.to_location(), xy(5, 5));
        assert_eq!((&position).to_location(), xy(5, 5));
    }
}
