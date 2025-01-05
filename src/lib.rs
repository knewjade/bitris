pub use enums::*;
pub use kicks::*;
pub use moves::*;
pub use order::*;
pub use rotation::*;
pub use traits::*;
use crate::boards::Board64;
use crate::coordinates::xy;

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
mod array_map;
mod myffi;

pub fn set_at2(dy: i32) -> i32 {
    if cfg!(feature="japanese") {
        myffi::set_at2(dy)
    } else {
        -999
    }
}
