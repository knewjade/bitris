use std::fmt;

use crate::{Offset, Piece, Rotation, Shape};

/// The amount of movement based on the center of the piece when rotating.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Kick {
    pub offset: Offset,
}

impl fmt::Display for Kick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kick ({}, {})", self.offset.dx, self.offset.dy)
    }
}

/// Control kicks from the piece's rotation.
pub trait KickTable<'a> {
    /// Returns kicks
    fn get_kicks(&self, piece: Piece, rotation: Rotation) -> &'a [Kick];

    /// Returns true if the shape may change position due to rotation.
    /// For example, in SRS, Shape::O does not move when rotated, so it's false.
    fn is_moving_in_rotation(&self, shape: Shape) -> bool;
}
