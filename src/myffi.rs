#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("ffi.hpp");

        fn search(
            boards: [u16; 10], spawn_piece: u8, spawn_orientation: u8, spawn_bx: u8, spawn_by: u8,
        ) -> [u16; 10];
    }
}
