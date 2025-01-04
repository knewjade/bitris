use std::fmt;
use std::slice::Iter;

use crate::boards::BoardOp;
use crate::coordinates::Offset;
use crate::pieces::{Piece, Shape};
use crate::placements::CcPlacement;
use crate::{Rotate, Rotation};

/// The amount of movement based on the center of the piece when rotating.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Kick {
    pub offset: Offset,
}

impl Kick {
    #[inline]
    pub const fn new(offset: Offset) -> Self {
        Self { offset }
    }
}

impl fmt::Display for Kick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kick ({}, {})", self.offset.dx, self.offset.dy)
    }
}

/// Represents the test results of the kick.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TestKickResult {
    pub test_index: usize,
    pub kick: Kick,
    pub destination: CcPlacement,
}

/// Control the rotation of the piece
pub trait RotationSystem {
    /// Returns kicks
    fn iter_kicks(&self, piece: Piece, rotation: Rotation) -> Iter<'_, Kick>;

    /// Returns true if the shape may change position due to rotation.
    /// For example, in SRS, Shape::O does not move when rotated, so it's false.
    fn is_moving_in_rotation(&self, shape: Shape) -> bool;

    /// Test the kick of the piece as it rotates on the board.
    ///
    /// Returns the final kick and placement if the test passes.
    /// Returns None if the rotation is not possible.
    fn test_kick(
        &self,
        board: &impl BoardOp,
        placement: impl Into<CcPlacement>,
        rotation: Rotation,
    ) -> Option<TestKickResult> {
        let from: CcPlacement = placement.into();
        let to = from.rotate(rotation);

        for (test_index, kick) in self.iter_kicks(from.piece, rotation).enumerate() {
            let destination = to + kick.offset;
            if destination.is_in_free_space(board) {
                return Some(TestKickResult {
                    test_index,
                    kick: *kick,
                    destination,
                });
            }
        }

        None
    }
}
