extern crate core;

#[cfg(test)]
use rstest_reuse;

pub use boards::*;
pub use kicks::*;
pub use location::*;
pub use moves::*;
pub use offset::*;
pub use operators::*;
pub use orientation::*;
pub use piece::*;
pub use piece_blocks::*;
pub use placements::*;
pub use positions::*;
pub use rotation::*;
pub use shape::*;
pub use traits::*;

#[doc(hidden)]
pub mod prelude {
    pub use crate::{
        boards::*,
        kicks::*,
        location::*,
        moves::*,
        offset::*,
        operators::*,
        orientation::*,
        piece::*,
        piece_blocks::*,
        placements::*,
        positions::*,
        rotation::*,
        shape::*,
        srs::SrsKickTable,
        traits::*,
    };
    pub use crate::macros;
    pub use crate::srs;
}

/// Operations based on SRS.
pub mod srs;

/// Defines macros
pub mod macros;

mod internal_macros;
mod internal_moves;

mod boards;
mod kicks;
mod location;
mod offset;
mod operators;
mod orientation;
mod piece;
mod piece_blocks;
mod placements;
mod positions;
mod rotation;
mod shape;
mod traits;
mod moves;
