/** This file is for internal */

use crate::boards::Board64;
use crate::coordinates::Location;

#[repr(align(32))]
pub struct AlignedU8s {
    pub data: [u8; 32],
}

impl AlignedU8s {
    pub fn new(data: [u8; 32]) -> Self {
        Self { data }
    }

    pub fn blank() -> Self {
        Self::new([0u8; 32])
    }

    pub fn set_at(mut self, location: Location) -> Self {
        let bit_position = location.y % 8;
        let index = location.x * 2 + location.y / 8;
        self.data[index as usize] |= 1 << bit_position;
        self
    }
}
