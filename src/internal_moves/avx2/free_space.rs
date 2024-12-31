use std::arch::x86_64::__m256i;
use crate::boards::Board16;
use crate::coordinates::Location;
use crate::internal_moves::avx2::aligned::AlignedU8s;
use crate::internal_moves::avx2::opsimd;
use crate::internal_moves::avx2::reachable::ReachableSimd16;

#[derive(Debug, Clone)]
pub struct FreeSpaceSimd16 {
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

impl FreeSpaceSimd16 {
    #[inline(always)]
    pub fn new(data: __m256i) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn and(self, other: FreeSpaceSimd16) -> Self {
        Self::new(opsimd::and(self.data, other.data))
    }

    #[inline(always)]
    pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(self) -> Self {
        Self::new(opsimd::shift::<LEFT, RIGHT, DOWN, UP>(self.data))
    }

    #[inline(always)]
    pub fn or_shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        target: &FreeSpaceSimd16,
    ) -> Self {
        let shift = opsimd::shift::<LEFT, RIGHT, DOWN, UP>(target.data);
        Self::new(opsimd::or(self.data, shift))
    }

    #[inline(always)]
    pub fn is_free_at(&self, location: Location) -> bool {
        opsimd::extract(self.data, location.x) & (1 << location.y) != 0
    }

    #[inline(always)]
    pub fn col(&self, x: usize) -> u16 {
        opsimd::extract(self.data, x as i32)
    }

    #[inline(always)]
    pub fn to_bytes_u16(&self) -> [u16; 10] {
        opsimd::to_bytes_u16(self.data)
    }
}

impl From<AlignedU8s> for FreeSpaceSimd16 {
    fn from(value: AlignedU8s) -> Self {
        Self { data: value.load() }
    }
}

impl From<&FreeSpaceSimd16> for Board16 {
    fn from(value: &FreeSpaceSimd16) -> Self {
        Board16::new(value.to_bytes_u16())
    }
}
