use std::pin::Pin;
pub use enums::*;
pub use kicks::*;
pub use moves::*;
pub use order::*;
pub use rotation::*;
pub use traits::*;
use crate::boards::Board64;
use crate::coordinates::xy;
use crate::myffi::MultiBuf;

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
pub mod myffi;

pub fn set_at2(dy: i32) -> i32 {
    if cfg!(feature="japanese") {
        myffi::ffi::set_at(dy)
    } else {
        -999
    }
}

pub fn search() -> i32 {
    if cfg!(feature="japanese") {
        let search1 = myffi::ffi::search([0; 10], 0, 0, 4, 20);
        (search1[0] - search1[0]) as i32
    } else {
        -999
    }
}

pub fn search3(buf: &mut MultiBuf) -> i32 {
    if cfg!(feature="japanese") {
        // myffi::ffi::search() as i32
        // (myffi::ffi::search2().len() - 512) as i32
        // unsafe {
        //     // let ptr = myffi::ffi::new_blobstore_client();
        //     0
        // }

        buf.clear();
        myffi::ffi::search3(Pin::new(buf));
        (buf.pos() - 3) as i32
    } else {
        -999
    }
}

fn create() -> Vec<i32> {
    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    vec2
}

pub fn search_rust() -> i32 {
    if cfg!(feature="japanese") {
        // (create().len() - 3) as i32
        // myffi::ffi::search().size as i32
        // (myffi::ffi::search2()[0]) as i32
        0
    } else {
        -999
    }
}
