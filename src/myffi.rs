extern crate libc;

#[link(name = "bitris_cpp", kind = "static")]
extern "C" {
    fn set_at(v: i32) -> i32;
}

pub fn set_at2(dy: i32) -> i32 {
    unsafe { set_at(dy) }
}
