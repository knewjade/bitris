use std::{fmt, ops};

use crate::coordinates::{Location, Offset};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};
use crate::placements::TrPlacement;

/// The position of the upper-right of the piece on the board; UR means that x points to the right of the piece and y points to the upper of the piece.
///
/// Note that upper-right is not robust to the rotation since the rotation takes place in the center of the piece.
/// If you use rotation, consider using `CCPlacement`.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(TrPosition::default(), TrPosition { rx: 0, ty: 0 });
/// assert_eq!(tr(3, 4), TrPosition { rx: 3, ty: 4 });
/// assert_eq!(format!("{}", TrPosition { rx: 3, ty: 4 }), "tr (3, 4)");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TrPosition {
    pub rx: i32,
    pub ty: i32,
}

impl TrPosition {
    #[inline]
    pub const fn new(rx: i32, ty: i32) -> Self {
        Self { rx, ty }
    }
}

impl fmt::Display for TrPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tr ({}, {})", self.rx, self.ty)
    }
}

impl ops::Add<Offset> for TrPosition {
    type Output = TrPosition;

    fn add(self, rhs: Offset) -> Self::Output {
        TrPosition { rx: self.rx + rhs.dx, ty: self.ty + rhs.dy }
    }
}

impl ops::AddAssign<Offset> for TrPosition {
    fn add_assign(&mut self, rhs: Offset) {
        self.rx += rhs.dx;
        self.ty += rhs.dy;
    }
}

forward_ref_op! { TrPosition, + Offset, = TrPosition }
forward_ref_op! { TrPosition, += Offset }

impl From<TrPlacement> for TrPosition {
    fn from(placement: TrPlacement) -> Self {
        placement.position
    }
}

forward_ref_from!(TrPosition, from TrPlacement);

add_member_for_from!(Location, to_location, to TrPosition);

/// An shortcut for `URPosition { rx, uy }`.
#[inline(always)]
pub const fn tr(rx: i32, ty: i32) -> TrPosition {
    TrPosition { rx, ty }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn tr_position_operators() {
        assert_eq!(tr(1, 2) + dd(-2, 3), tr(-1, 5));

        let mut position = tr(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, tr(-1, 2));
    }
}
