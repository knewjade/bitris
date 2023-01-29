use std::{fmt, ops};

use crate::internal_macros::forward_ref_op;

/// The difference from a base point to a point on the board.
/// ```
/// use bitris::prelude::*;
/// assert_eq!(Offset::default(), Offset { dx: 0, dy: 0 });
/// assert_eq!(dd(1, -2), Offset { dx: 1, dy: -2 });
/// assert_eq!(format!("{}", Offset::new(1, -1)), "offset (1, -1)");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Offset {
    pub dx: i32,
    pub dy: i32,
}

impl Offset {
    #[inline]
    pub const fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "offset ({}, {})", self.dx, self.dy)
    }
}

impl ops::Add<Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset { dx: self.dx + rhs.dx, dy: self.dy + rhs.dy }
    }
}

impl ops::AddAssign<Offset> for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl ops::Sub<Offset> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Offset) -> Self::Output {
        Offset { dx: self.dx - rhs.dx, dy: self.dy - rhs.dy }
    }
}

impl ops::SubAssign<Offset> for Offset {
    fn sub_assign(&mut self, rhs: Offset) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy;
    }
}

impl ops::Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Offset { dx: -self.dx, dy: -self.dy }
    }
}

forward_ref_op! { Offset, + Offset, = Offset }
forward_ref_op! { Offset, += Offset }
forward_ref_op! { Offset, - Offset, = Offset }
forward_ref_op! { Offset, -= Offset }
forward_ref_op! { - Offset }

/// A shortcut for `Offset { dx, dy }`.
#[inline(always)]
pub const fn dd(dx: i32, dy: i32) -> Offset {
    Offset { dx, dy }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn operators() {
        assert_eq!(dd(1, 2) + dd(-2, 3), dd(-1, 5));
        assert_eq!(dd(1, 2) - dd(-2, 3), dd(3, -1));
        assert_eq!(-dd(-2, 3), dd(2, -3));

        let mut offset = dd(-2, 3);
        offset += dd(1, -1);
        assert_eq!(offset, dd(-1, 2));
        offset -= dd(1, -1);
        assert_eq!(offset, dd(-2, 3));
    }
}
