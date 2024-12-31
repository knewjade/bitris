use crate::internal_moves::avx2::aligned::AlignedU8s;
use crate::internal_moves::avx2::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::opsimd;
use std::arch::x86_64::*;
use crate::boards::Board16;
use crate::coordinates::{Location, Offset};

#[derive(Debug, Clone)]
pub struct ReachableSimd16 {
    // フィールド縦方向1列ごとに16ビット(8bit2つ)で表現される。
    // したがって、W10xH16のフィールドが表現されている。
    //
    // 使用されない末尾96bit(8*12)の状態は未定義とする(0 or 1か確定しない)
    // * data[0]: x=0 の (0<=y<8) を表現
    // * data[1]: x=0 の (8<=y<16) を表現
    // * data[2]: x=1 の (0<=y<8) を表現
    // * ...
    //
    // 8bitの中は下位ビットほどyが小さい
    // * (x=0, y=2)は data[0] = 0b00000100
    // * (x=0, y=9)は data[1] = 0b00000001
    //
    // レジスタ上ではymm[0] = data[0]に対応しているので注意
    pub data: __m256i,
}

impl PartialEq for ReachableSimd16 {
    fn eq(&self, other: &Self) -> bool {
        opsimd::equals_to(self.data, other.data)
    }
}

impl ReachableSimd16 {
    #[inline(always)]
    pub fn new(data: __m256i) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn blank() -> Self {
        Self::new(opsimd::zeros())
    }

    #[inline(always)]
    pub fn is_visited(&self, location: Location) -> bool {
        opsimd::extract(self.data, location.x) & (1 << location.y) != 0
    }

    #[inline(always)]
    pub fn clip(self, mask: u16) -> Self {
        Self::new(opsimd::clip(self.data, mask))
    }

    #[inline(always)]
    pub fn or(self, other: &ReachableSimd16) -> Self {
        Self::new(opsimd::or(self.data, other.data))
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        opsimd::is_all_zero(self.data)
    }

    // ボードを左右下方向にシフトしてマージ
    #[inline(always)]
    pub fn move1(self, free_space: &FreeSpaceSimd16) -> Self {
        Self::new(opsimd::move1(self.data, free_space.data))
    }

    #[inline(always)]
    pub fn jump_and<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        free_space: &FreeSpaceSimd16,
    ) -> Self {
        let shift = opsimd::shift::<LEFT, RIGHT, DOWN, UP>(self.data);
        let merged =  opsimd::and(free_space.data, shift);
        Self::new(merged)
    }

    #[inline(always)]
    pub fn jump_rev<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        dest_jumped: ReachableSimd16,
    ) -> Self {
        let shift = opsimd::shift::<LEFT, RIGHT, DOWN, UP>(dest_jumped.data);
        let filtered = opsimd::and_not(shift, self.data);
        Self::new(filtered)
    }

    #[inline(always)]
    pub fn or_shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        target: &ReachableSimd16,
    ) -> Self {
        let shift = opsimd::shift::<LEFT, RIGHT, DOWN, UP>(target.data);
        Self::new(opsimd::or(self.data, shift))
    }

    #[inline(always)]
    pub fn land(self, free_space: &FreeSpaceSimd16) -> Self {
        Self::new(opsimd::land(self.data, free_space.data))
    }

    #[inline(always)]
    pub fn to_bytes_u16(&self) -> [u16; 10] {
        opsimd::to_bytes_u16(self.data)
    }

    #[inline(always)]
    pub fn to_bytes_u32(&self) -> [u32; 10] {
        opsimd::to_bytes_u32(self.data)
    }
}

impl From<AlignedU8s> for ReachableSimd16 {
    fn from(value: AlignedU8s) -> Self {
        Self { data: value.load() }
    }
}

impl From<&ReachableSimd16> for Board16 {
    fn from(value: &ReachableSimd16) -> Self {
        Board16::new(value.to_bytes_u16())
    }
}
