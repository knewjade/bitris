use crate::boards::Board64;
use crate::internal_moves::u64::opu64;
use crate::prelude::Location;

// The position where there is space to place a piece is represented by 1.
// The flags are aggregated to the position that corresponds to Bottom-Left.
#[derive(Debug, Clone)]
pub struct FreeSpace64 {
    pub cols: [u64; 10],
}

impl FreeSpace64 {
    #[inline(always)]
    pub fn new(cols: [u64; 10]) -> Self {
        Self { cols }
    }

    #[inline(always)]
    pub fn and(self, other: FreeSpace64) -> Self {
        let mut cols = self.cols;
        for index in 0..self.cols.len() {
            cols[index] &= other.cols[index];
        }
        Self::new(cols)
    }

    #[inline(always)]
    pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(self) -> Self {
        Self::new(opu64::shift::<LEFT, RIGHT, DOWN, UP>(self.cols))
    }

    #[inline(always)]
    pub fn is_free_at(&self, location: Location) -> bool {
        self.cols[location.x as usize] & (1u64 << location.y as usize) != 0
    }
}

impl From<&FreeSpace64> for Board64 {
    fn from(value: &FreeSpace64) -> Self {
        Board64::new(value.cols)
    }
}
