use crate::boards::Board32;
use crate::coordinates::Location;
use crate::internal_moves::avx2::h24::aligned::AlignedU24s;
use crate::internal_moves::avx2::h24::opsimd;
use std::arch::x86_64::__m256i;

#[derive(Debug, Clone)]
pub struct FreeSpaceSimd24 {
    // フィールド縦方向1列ごとに24ビット(8bit3つ)で表現される。
    // したがって、W10xH24のフィールドが表現されている。
    //
    // 使用されない末尾16bit(8*2)の状態は未定義とする(0 or 1か確定しない)
    // * data[0]: x=0 の (0<=y<8) を表現
    // * data[1]: x=0 の (8<=y<y16) を表現
    // * data[2]: x=0 の (y16<=y<24) を表現
    // * ...
    //
    // 8bitの中は下位ビットほどyが小さい
    // * (x=0, y=2 )は data[0] = 0b00000100
    // * (x=0, y=10)は data[1] = 0b00000010
    // * (x=0, y=y16)は data[2] = 0b00000001
    //
    // レジスタ上ではymm[0] = data[0..3]に対応しているので注意
    pub data: __m256i,
}

impl FreeSpaceSimd24 {
    #[inline(always)]
    pub fn new(data: __m256i) -> Self {
        Self { data }
    }

    #[inline(always)]
    pub fn and(self, other: &FreeSpaceSimd24) -> Self {
        Self::new(opsimd::and(self.data, other.data))
    }

    #[inline(always)]
    pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(&self) -> Self {
        Self::new(opsimd::shift::<LEFT, RIGHT, DOWN, UP>(self.data))
    }

    #[inline(always)]
    pub fn or_shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
        self,
        target: &FreeSpaceSimd24,
    ) -> Self {
        let shift = opsimd::shift::<LEFT, RIGHT, DOWN, UP>(target.data);
        Self::new(opsimd::or(self.data, shift))
    }

    #[inline(always)]
    pub fn is_free_at(&self, location: Location) -> bool {
        opsimd::is_one_at(self.data, location)
    }

    #[inline(always)]
    pub fn col(&self, x: usize) -> u32 {
        opsimd::extract(self.data, x as i32)
    }

    #[inline(always)]
    pub fn to_bytes_u32(&self) -> [u32; 10] {
        opsimd::to_bytes_u32(self.data)
    }
}

impl From<AlignedU24s> for FreeSpaceSimd24 {
    fn from(value: AlignedU24s) -> Self {
        Self { data: value.load() }
    }
}

impl From<&FreeSpaceSimd24> for Board32 {
    fn from(value: &FreeSpaceSimd24) -> Self {
        Board32::new(value.to_bytes_u32())
    }
}
