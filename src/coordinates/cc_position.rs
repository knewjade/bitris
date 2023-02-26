use std::{fmt, ops};

use crate::coordinates::{Location, Offset};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::placements::CcPlacement;

/// The position of the rotation center of the piece on the board; CC means that xy both point to the center of the piece.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(CcPosition::default(), CcPosition { cx: 0, cy: 0 });
/// assert_eq!(cc(1, -2), CcPosition { cx: 1, cy: -2 });
/// assert_eq!(format!("{}", CcPosition { cx: 1, cy: -2 }), "cc (1, -2)");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CcPosition {
    pub cx: i32,
    pub cy: i32,
}

impl CcPosition {
    #[inline]
    pub const fn new(cx: i32, cy: i32) -> Self {
        Self { cx, cy }
    }
}

impl fmt::Display for CcPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cc ({}, {})", self.cx, self.cy)
    }
}

impl ops::Add<Offset> for CcPosition {
    type Output = CcPosition;

    fn add(self, rhs: Offset) -> Self::Output {
        CcPosition { cx: self.cx + rhs.dx, cy: self.cy + rhs.dy }
    }
}

impl ops::AddAssign<Offset> for CcPosition {
    fn add_assign(&mut self, rhs: Offset) {
        self.cx += rhs.dx;
        self.cy += rhs.dy;
    }
}

forward_ref_op! { CcPosition, + Offset, = CcPosition }
forward_ref_op! { CcPosition, += Offset }

impl From<CcPlacement> for CcPosition {
    fn from(placement: CcPlacement) -> Self {
        placement.position
    }
}

forward_ref_from!(CcPosition, from CcPlacement);

add_member_for_from!(Location, to_location, to CcPosition);

/// An shortcut for `CCPosition { cx, cy }`.
#[inline(always)]
pub const fn cc(cx: i32, cy: i32) -> CcPosition {
    CcPosition { cx, cy }
}


#[cfg(test)]
mod tests {
    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn cc_placement_to_position() {
        let placement = CcPlacement::new(piece!(TN), CcPosition::new(2, 3));
        assert_eq!(CcPosition::from(placement), cc(2, 3));
    }

    #[test]
    fn cc_position_operators() {
        assert_eq!(cc(1, 2) + dd(-2, 3), cc(-1, 5));

        let mut position = cc(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, cc(-1, 2));
    }
}
