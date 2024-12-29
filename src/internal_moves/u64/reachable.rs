use crate::boards::Board64;
use crate::coordinates::{Location, Offset};
use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::internal_moves::u64::opu64;

#[derive(Debug, Clone)]
pub struct Reachable64 {
    pub(crate) cols: [u64; 10],
}

impl PartialEq for Reachable64 {
    fn eq(&self, other: &Self) -> bool {
        self.cols == other.cols
    }
}

impl Reachable64 {
    #[inline(always)]
    pub fn new(cols: [u64; 10]) -> Self {
        Self { cols }
    }

    #[inline(always)]
    pub fn blank() -> Self {
        Self::new([0; 10])
    }

    #[inline(always)]
    pub fn set_at(self, location: Location) -> Self {
        let mut cols = self.cols;
        cols[location.x as usize] |= 1 << location.y;
        Self::new(cols)
    }

    #[inline(always)]
    pub fn or(self, other: &Reachable64) -> Self {
        Self::new(opu64::or(self.cols, other.cols))
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.cols.iter().all(|&v| v == 0)
    }

    // ボードを左右下方向にシフトしてマージ
    #[inline(always)]
    pub fn move_n(self, free_space: &FreeSpace64, left: bool) -> Self {
        if left {
            Self::new(opu64::move_nl(self.cols, free_space.cols))
        } else {
            Self::new(opu64::move_nr(self.cols, free_space.cols))
        }
    }

    #[inline(always)]
    pub fn jump_and(self, free_space: &FreeSpace64, offset: Offset) -> Self {
        let shift = opu64::shift_by_offset(self.cols, offset);
        Self::new(opu64::and(free_space.cols, shift))
    }

    #[inline(always)]
    pub fn jump_rev(self, dest_jumped: Reachable64, offset: Offset) -> Self {
        let shift = opu64::shift_by_offset(dest_jumped.cols, offset);
        Self::new(opu64::and_not(shift, self.cols))
    }

    #[inline(always)]
    pub fn or_shift(self, target: &Reachable64, offset: Offset) -> Self {
        let shift = opu64::shift_by_offset(target.cols, offset);
        Self::new(opu64::or(self.cols, shift))
    }

    #[inline(always)]
    pub fn land(self, free_space: &FreeSpace64) -> Self {
        Self::new(opu64::land(self.cols, free_space.cols))
    }
}

impl From<&Reachable64> for Board64 {
    fn from(value: &Reachable64) -> Self {
        Board64::new(value.cols)
    }
}
