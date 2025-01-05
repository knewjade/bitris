#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("hello.hpp");

        fn set_at(v: i32) -> i32;
    }
}
