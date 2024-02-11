use std::{fmt, ops};

use crate::coordinates::{Location, Offset};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::placements::BlPlacement;

/// The position of the bottom-left of the piece on the board; BL means that x points to the left of the piece and y points to the bottom of the piece.
///
/// Note that bottom-left is not robust to the rotation since the rotation takes place in the center of the piece.
/// If you use rotation, consider using `CCPlacement`.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(BlPosition::default(), BlPosition { lx: 0, by: 0 });
/// assert_eq!(bl(1, 2), BlPosition { lx: 1, by: 2 });
/// assert_eq!(format!("{}", BlPosition { lx: 1, by: 2 }), "bl (1, 2)");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BlPosition {
    pub lx: i32,
    pub by: i32,
}

impl BlPosition {
    #[inline]
    pub const fn new(lx: i32, by: i32) -> Self {
        Self { lx, by }
    }
}

impl fmt::Display for BlPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bl ({}, {})", self.lx, self.by)
    }
}

impl ops::Add<Offset> for BlPosition {
    type Output = BlPosition;

    fn add(self, rhs: Offset) -> Self::Output {
        BlPosition { lx: self.lx + rhs.dx, by: self.by + rhs.dy }
    }
}

impl ops::AddAssign<Offset> for BlPosition {
    fn add_assign(&mut self, rhs: Offset) {
        self.lx += rhs.dx;
        self.by += rhs.dy;
    }
}

forward_ref_op! { BlPosition, + Offset, = BlPosition }
forward_ref_op! { BlPosition, += Offset }

impl From<BlPlacement> for BlPosition {
    fn from(placement: BlPlacement) -> Self {
        placement.position
    }
}

forward_ref_from!(BlPosition, from BlPlacement);

add_member_for_from!(Location, to_location, to BlPosition);

/// A shortcut for `BLPosition { lx, by }`.
#[inline(always)]
pub const fn bl(lx: i32, by: i32) -> BlPosition {
    BlPosition { lx, by }
}


#[cfg(test)]
mod tests {
    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn bl_placement_to_position() {
        let placement = BlPlacement::new(piece!(TN), BlPosition::new(2, 3));
        assert_eq!(BlPosition::from(placement), bl(2, 3));
    }

    #[test]
    fn bl_position_operators() {
        assert_eq!(bl(1, 2) + dd(-2, 3), bl(-1, 5));

        let mut position = bl(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, bl(-1, 2));
    }
}
