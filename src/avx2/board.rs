use std::arch::x86_64::*;
use crate::avx2::aligned::AlignedU8s;
use crate::boards::Board16;

#[derive(Debug, Clone)]
pub struct BoardSimd {
    // Represents an AVX2 256-bit register
    // フィールド縦方向1列ごとに16ビット(8bit2つ)で表現される。
    // 末尾64bit+32bit(8*12)はすべて未定義とする(0 or 1)
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

impl PartialEq for BoardSimd {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            _mm256_testc_si256(self.data, other.data) != 0
        }
    }
}

impl From<AlignedU8s> for BoardSimd {
    fn from(value: AlignedU8s) -> Self {
        unsafe {
            let aligned_data = _mm256_load_si256(value.data.as_ptr() as *const __m256i);
            Self { data: aligned_data }
        }
    }
}

impl BoardSimd {
    pub fn new(data: __m256i) -> Self {
        Self { data }
    }

    pub fn and(self, other: BoardSimd) -> Self {
        unsafe {
            Self::new(_mm256_and_si256(self.data, other.data))
        }
    }

//     pub fn set(self, x: usize, y: usize) -> Self {
//         let aligned = AlignedU8s::blank().set(x, y);
//         self.or(aligned)
//     }
//

//
//     pub fn column0(&self) -> u16 {
//         unsafe { _mm256_extract_epi16::<0>(self.data) as u16 }
//     }
//
//     pub fn column1(&self) -> u16 {
//         unsafe { _mm256_extract_epi16::<1>(self.data) as u16 }
//     }
//
//     pub fn column9(&self) -> u16 {
//         unsafe { _mm256_extract_epi16::<9>(self.data) as u16 }
//     }
//
//     pub fn count(&self) -> usize {
//         unsafe {
//             let count = _mm256_extract_epi64::<0>(self.data).count_ones()
//                 + _mm256_extract_epi64::<1>(self.data).count_ones()
//                 + _mm256_extract_epi32::<4>(self.data).count_ones();
//             count as usize
//         }
//     }

    // ボードを右方向にシフト
    pub fn shift_right<const N: i32>(self) -> Self {
        assert!(0 <= N && N <= 4);
        if N == 0 {
            return self;
        }

        unsafe {
            // レジスタ上では左シフト
            // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
            // mask = | B(128bit; x=0~7) 0(128bit) |  < 0x08
            // シフトは最大8まで
            let mask = _mm256_permute2x128_si256::<0x08>(self.data, self.data);
            match N {
                1 => Self::new(_mm256_alignr_epi8::<{16 - 2 * 1}>(self.data, mask)),
                2 => Self::new(_mm256_alignr_epi8::<{16 - 2 * 2}>(self.data, mask)),
                3 => Self::new(_mm256_alignr_epi8::<{16 - 2 * 3}>(self.data, mask)),
                4 => Self::new(_mm256_alignr_epi8::<{16 - 2 * 4}>(self.data, mask)),
                _ => todo!()
            }
        }
    }

//     // x方向左に1つずらす
//     pub fn slide_left(self) -> Self {
//         unsafe {
//             // 上位ビットの初期化
//             let data = _mm256_insert_epi64::<3>(self.data, 0);
//             let data = _mm256_insert_epi32::<5>(data, 0);
//
//             // レジスタ上では右シフト
//             // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
//             // mask = | 0(128bit) A(128bit; x=8~9) |  < 0x81
//             // a    = | A(128bit; x=8~9) B(128bit; x=0~7) |
//             // b    = | 0(128bit) A(128bit; x=8~9) |
//             // シフトは最大8まで
//             let mask = _mm256_permute2x128_si256::<0x81>(data, data);
//             Self::new(_mm256_alignr_epi8::<2>(mask, data))
//         }
//     }
//
//     // y方向下に1つずらす
//     pub fn slide_down(self) -> Self {
//         unsafe { Self::new(_mm256_srli_epi16::<1>(self.data)) }
//     }

    // ボードを上方向にシフト
    pub fn shift_up<const N: i32>(self) -> Self {
        assert!(0 <= N && N <= 16);
        if N == 0 {
            return self;
        }

        if N == 16 {
            unsafe {
                return Self::new(_mm256_setzero_si256());
            }
        }

        unsafe {
            Self::new(_mm256_slli_epi16::<N>(self.data))
        }
    }

    // ボードを下方向にシフトしてマージ
    pub fn move1(self, free_space: &BoardSimd) -> Self {
        unsafe {
            let data = self.data;
            let free_space = free_space.data;

            // right
            let mask = _mm256_permute2x128_si256::<0x08>(data, data);
            let shift = _mm256_alignr_epi8::<{16 - 2 * 1}>(data, mask);
            let candidate = _mm256_and_si256(free_space, shift);
            let data = _mm256_or_si256(data, candidate);

            // left
            let data = _mm256_insert_epi64::<3>(data, 0);
            let data = _mm256_insert_epi32::<5>(data, 0);
            let mask = _mm256_permute2x128_si256::<0x81>(data, data);
            let shift = _mm256_alignr_epi8::<2>(mask, data);
            let candidate = _mm256_and_si256(free_space, shift);
            let data = _mm256_or_si256(data, candidate);

            // down
            let shift = _mm256_srli_epi16::<1>(data);
            let candidate = _mm256_and_si256(free_space, shift);
            let data = _mm256_or_si256(data, candidate);

            Self::new(data)
        }
    }

    pub fn lock(self) -> Self {
        unsafe {
            let data = self.data;

            let up = _mm256_slli_epi16::<1>(data);
            let data = _mm256_andnot_si256(up, data);

            Self::new(data)
        }
    }

    fn data_to_byte(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        unsafe {
            _mm256_storeu_si256(bytes.as_mut_ptr() as *mut __m256i, self.data);
        }
        bytes
    }

    pub fn to_bytes_u16(&self) -> [u16; 10] {
        let data = self.data_to_byte();
        [
            (data[0] as u16) | (data[1] as u16) << 8,
            (data[2] as u16) | (data[3] as u16) << 8,
            (data[4] as u16) | (data[5] as u16) << 8,
            (data[6] as u16) | (data[7] as u16) << 8,
            (data[8] as u16) | (data[9] as u16) << 8,
            (data[10] as u16) | (data[11] as u16) << 8,
            (data[12] as u16) | (data[13] as u16) << 8,
            (data[14] as u16) | (data[15] as u16) << 8,
            (data[16] as u16) | (data[17] as u16) << 8,
            (data[18] as u16) | (data[19] as u16) << 8,
        ]
    }

//     pub fn to_bytes_u32(&self) -> [u32; 10] {
//         let data = self.data_to_byte();
//         [
//             (data[0] as u32) | (data[1] as u32) << 8,
//             (data[2] as u32) | (data[3] as u32) << 8,
//             (data[4] as u32) | (data[5] as u32) << 8,
//             (data[6] as u32) | (data[7] as u32) << 8,
//             (data[8] as u32) | (data[9] as u32) << 8,
//             (data[10] as u32) | (data[11] as u32) << 8,
//             (data[12] as u32) | (data[13] as u32) << 8,
//             (data[14] as u32) | (data[15] as u32) << 8,
//             (data[16] as u32) | (data[17] as u32) << 8,
//             (data[18] as u32) | (data[19] as u32) << 8,
//         ]
//     }
}

impl From<&BoardSimd> for Board16 {
    fn from(value: &BoardSimd) -> Self {
        let bytes_u16 = value.to_bytes_u16();
        Board16::new(bytes_u16)
    }
}
