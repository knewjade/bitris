use crate::coordinates::Location;
use std::arch::x86_64::{__m256i, _mm256_load_si256};

#[repr(align(32))]
#[derive(Clone, Debug)]
pub struct AlignedU16s {
    pub data: [u16; 16],
}

impl AlignedU16s {
    #[inline(always)]
    pub fn new(data: [u16; 16]) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn blank() -> Self {
        Self::new([0u16; 16])
    }

    #[inline(always)]
    pub fn set_at(mut self, location: Location) -> Self {
        self.data[location.x as usize] |= 1 << location.y;
        self
    }

    #[inline(always)]
    pub fn set_u16(&mut self, x: usize, bytes: u16) {
        self.data[x] = bytes;
    }

    #[inline(always)]
    pub fn load(&self) -> __m256i {
        unsafe { _mm256_load_si256(self.data.as_ptr() as *const __m256i) }
    }
}
