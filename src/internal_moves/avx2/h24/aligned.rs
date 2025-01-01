use crate::coordinates::Location;
use std::arch::x86_64::{__m256i, _mm256_load_si256};

#[repr(align(32))]
#[derive(Clone, Debug)]
pub struct AlignedU24s {
    pub data: [u8; 32],
}

impl AlignedU24s {
    #[inline(always)]
    pub fn new(data: [u8; 32]) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn blank() -> Self {
        Self::new([0u8; 32])
    }

    #[inline(always)]
    pub fn set_at(mut self, location: Location) -> Self {
        let bit_position = location.y % 8;
        let index = location.x * 3 + location.y / 8;
        self.data[index as usize] |= 1 << bit_position;
        self
    }

    #[inline(always)]
    pub fn set_u32(&mut self, x: usize, bytes: u32) {
        self.data[x * 3] = bytes as u8;
        self.data[x * 3 + 1] = (bytes >> 8) as u8;
        self.data[x * 3 + 2] = (bytes >> 16) as u8;
    }

    #[inline(always)]
    pub fn load(&self) -> __m256i {
        unsafe { _mm256_load_si256(self.data.as_ptr() as *const __m256i) }
    }
}
