use crate::avx2::aligned::AlignedU8s;
use crate::avx2::simd;
use std::arch::x86_64::*;
use crate::avx2::reachable::ReachableSimd16;
use crate::boards::Board16;

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
        unsafe { Self::new(_mm256_and_si256(self.data, other.data)) }
    }

    #[inline(always)]
    pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(self) -> Self {
        Self::new(simd::shift::<LEFT, RIGHT, DOWN, UP>(self.data))
    }

    #[inline(always)]
    pub fn to_bytes_u16(&self) -> [u16; 10] {
        simd::to_bytes_u16(self.data)
    }
}

impl From<AlignedU8s> for FreeSpaceSimd16 {
    fn from(value: AlignedU8s) -> Self {
        unsafe {
            let aligned_data = _mm256_load_si256(value.data.as_ptr() as *const __m256i);
            Self { data: aligned_data }
        }
    }
}

impl From<&FreeSpaceSimd16> for Board16 {
    fn from(value: &FreeSpaceSimd16) -> Self {
        let bytes_u16 = value.to_bytes_u16();
        Board16::new(bytes_u16)
    }
}
