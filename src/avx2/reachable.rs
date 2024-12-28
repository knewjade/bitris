use crate::avx2::aligned::AlignedU8s;
use crate::avx2::free_space::FreeSpaceSimd16;
use crate::avx2::simd;
use std::arch::x86_64::*;
use crate::boards::Board16;

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
        unsafe { _mm256_testc_si256(self.data, other.data) != 0 }
    }
}

impl ReachableSimd16 {
    #[inline(always)]
    pub fn new(data: __m256i) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn blank() -> Self {
        unsafe { Self::new(_mm256_setzero_si256()) }
    }

    #[inline(always)]
    pub fn or(self, other: &ReachableSimd16) -> Self {
        unsafe { Self::new(_mm256_or_si256(self.data, other.data)) }
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        unsafe { _mm256_testz_si256(self.data, self.data) != 0 }
    }

    // ボードを左右下方向にシフトしてマージ
    #[inline(always)]
    pub fn move1(self, free_space: &FreeSpaceSimd16) -> Self {
        Self::new(simd::move1(self.data, free_space.data))
    }

    #[inline(always)]
    pub fn jump_and<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        free_space: &FreeSpaceSimd16,
    ) -> Self {
        let shift = simd::shift::<LEFT, RIGHT, DOWN, UP>(self.data);
        let merged = unsafe { _mm256_and_si256(free_space.data, shift) };
        Self::new(merged)
    }

    #[inline(always)]
    pub fn jump_rev<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        dest_jumped: ReachableSimd16,
    ) -> Self {
        let shift = simd::shift::<LEFT, RIGHT, DOWN, UP>(dest_jumped.data);
        let filtered = unsafe { _mm256_andnot_si256(shift, self.data) };
        Self::new(filtered)
    }

    #[inline(always)]
    pub fn land(self, free_space: &FreeSpaceSimd16) -> Self {
        unsafe {
            let up = _mm256_slli_epi16::<1>(free_space.data);
            let data = _mm256_andnot_si256(up, self.data);
            Self::new(data)
        }
    }

    #[inline(always)]
    pub fn to_bytes_u16(&self) -> [u16; 10] {
        simd::to_bytes_u16(self.data)
    }
}

impl From<AlignedU8s> for ReachableSimd16 {
    fn from(value: AlignedU8s) -> Self {
        unsafe {
            let aligned_data = _mm256_load_si256(value.data.as_ptr() as *const __m256i);
            Self { data: aligned_data }
        }
    }
}

impl From<&ReachableSimd16> for Board16 {
    fn from(value: &ReachableSimd16) -> Self {
        let bytes_u16 = value.to_bytes_u16();
        Board16::new(bytes_u16)
    }
}
