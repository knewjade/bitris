use std::{fmt, ops};

use crate::{BlPlacement, CcPlacement, Location, Offset, TrPlacement};
use crate::internal_macros::{add_member_for_from, forward_ref_from, forward_ref_op};

/// The position of the rotation center of the piece on the board; CC means that xy both point to the center of the piece.
/// ```
/// use bitris::{CcPosition, cc};
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


/// The position of the bottom-left of the piece on the board; BL means that x points to the left of the piece and y points to the bottom of the piece.
///
/// Note that bottom-left is not robust to the rotation since the rotation takes place in the center of the piece.
/// If you use rotation, consider using `CCPlacement`.
/// ```
/// use bitris::{BlPosition, bl};
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

/// An shortcut for `BLPosition { lx, by }`.
#[inline(always)]
pub const fn bl(lx: i32, by: i32) -> BlPosition {
    BlPosition { lx, by }
}


/// The position of the upper-right of the piece on the board; UR means that x points to the right of the piece and y points to the upper of the piece.
///
/// Note that upper-right is not robust to the rotation since the rotation takes place in the center of the piece.
/// If you use rotation, consider using `CCPlacement`.
/// ```
/// use bitris::{TrPosition, tr};
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
    use crate::*;

    #[test]
    fn cc_position_operators() {
        assert_eq!(cc(1, 2) + dd(-2, 3), cc(-1, 5));

        let mut position = cc(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, cc(-1, 2));
    }

    #[test]
    fn bl_position_operators() {
        assert_eq!(bl(1, 2) + dd(-2, 3), bl(-1, 5));

        let mut position = bl(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, bl(-1, 2));
    }

    #[test]
    fn tr_position_operators() {
        assert_eq!(tr(1, 2) + dd(-2, 3), tr(-1, 5));

        let mut position = tr(-2, 3);
        position += dd(1, -1);
        assert_eq!(position, tr(-1, 2));
    }
}
