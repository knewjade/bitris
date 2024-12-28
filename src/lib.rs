#[cfg(test)]
use rstest_reuse;

pub use enums::*;
pub use kicks::*;
pub use moves::*;
pub use order::*;
pub use rotation::*;
pub use traits::*;

#[doc(hidden)]
pub mod prelude {
    pub use crate::{
        enums::*,
        kicks::*,
        moves::*,
        order::*,
        rotation::*,
        traits::*,
    };
    pub use crate::{
        boards::*,
        coordinates::*,
        pieces::*,
        placements::*,
        srs::SrsKickTable,
    };
}

// Exposed modules
pub mod boards;
pub mod coordinates;
pub mod macros;
pub mod pieces;
pub mod placements;

// Internals
mod internal_macros;
mod internal_moves;
mod enums;
mod kicks;
mod moves;
mod order;
mod rotation;
mod traits;
