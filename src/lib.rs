pub use enums::*;
pub use kicks::*;
pub use moves::*;
pub use order::*;
pub use rotation::*;
pub use traits::*;

#[doc(hidden)]
pub mod prelude {
    pub use crate::{boards::*, coordinates::*, pieces::*, placements::*, srs::SrsKickTable};
    pub use crate::{enums::*, kicks::*, moves::*, order::*, rotation::*, traits::*};
}

// Exposed modules
pub mod boards;
pub mod coordinates;
pub mod macros;
pub mod pieces;
pub mod placements;

// Internals
mod enums;
mod internal_macros;
mod internal_moves;
mod kicks;
mod moves;
mod order;
mod rotation;
mod traits;
