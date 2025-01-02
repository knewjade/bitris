use std::arch::x86_64::*;
use crate::boards::Board16;
use crate::internal_moves::avx2::h16::aligned::AlignedU16s;

#[inline(always)]
pub fn load_from_unaligned(data: &[u8; 32]) -> __m256i {
    unsafe { _mm256_loadu_si256(data.as_ptr() as *const __m256i) }
}

#[inline(always)]
pub fn zeros() -> __m256i {
    unsafe { _mm256_setzero_si256() }
}

#[inline(always)]
pub fn fill_with(value: i16) -> __m256i {
    unsafe { _mm256_set1_epi16(value) }
}

#[inline(always)]
pub fn equals_to(left: __m256i, right: __m256i) -> bool {
    unsafe { _mm256_testc_si256(left, right) != 0 }
}

#[inline(always)]
pub fn is_all_zero(data: __m256i) -> bool {
    unsafe { _mm256_testz_si256(data, data) != 0 }
}

#[inline(always)]
#[allow(clippy::nonminimal_bool)]
pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
    data: __m256i,
) -> __m256i {
    debug_assert!(0 <= LEFT && LEFT <= 4);
    debug_assert!(0 <= RIGHT && RIGHT <= 4);
    debug_assert!(
        (LEFT == 0 && RIGHT == 0) || (0 < LEFT && RIGHT == 0) || (0 < RIGHT && LEFT == 0)
    );

    debug_assert!(0 <= DOWN && DOWN <= 16);
    debug_assert!(0 <= UP && UP <= 16);
    debug_assert!((DOWN == 0 && UP == 0) || (0 < DOWN && UP == 0) || (0 < UP && DOWN == 0));

    if LEFT == 0 && RIGHT == 0 && DOWN == 0 && UP == 0 {
        return data;
    }

    if DOWN == 16 || UP == 16 {
        return zeros();
    }

    // down or up
    let data = if 0 < DOWN {
        unsafe { _mm256_srli_epi16::<DOWN>(data) }
    } else if 0 < UP {
        unsafe { _mm256_slli_epi16::<UP>(data) }
    } else {
        data
    };

    // left or right
    let data = if 0 < LEFT {
        unsafe {
            // 上位ビットの初期化
            let data = _mm256_insert_epi64::<3>(data, 0);
            let data = _mm256_insert_epi32::<5>(data, 0);

            // レジスタ上では右シフト
            // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
            // mask = | 0(128bit) A(128bit; x=8~9) |  < 0x81
            // a    = | A(128bit; x=8~9) B(128bit; x=0~7) |
            // b    = | 0(128bit) A(128bit; x=8~9) |
            // シフトは最大8まで
            let mask = _mm256_permute2x128_si256::<0x81>(data, data);
            match LEFT {
                1 => _mm256_alignr_epi8::<{ 2 * 1 }>(mask, data),
                2 => _mm256_alignr_epi8::<{ 2 * 2 }>(mask, data),
                3 => _mm256_alignr_epi8::<{ 2 * 3 }>(mask, data),
                4 => _mm256_alignr_epi8::<{ 2 * 4 }>(mask, data),
                _ => panic!("Invalid RIGHT: {}", RIGHT),
            }
        }
    } else if 0 < RIGHT {
        unsafe {
            // レジスタ上では左シフト
            // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
            // mask = | B(128bit; x=0~7) 0(128bit) |  < 0x08
            // シフトは最大8まで
            let mask = _mm256_permute2x128_si256::<0x08>(data, data);
            match RIGHT {
                1 => _mm256_alignr_epi8::<{ 16 - 2 * 1 }>(data, mask),
                2 => _mm256_alignr_epi8::<{ 16 - 2 * 2 }>(data, mask),
                3 => _mm256_alignr_epi8::<{ 16 - 2 * 3 }>(data, mask),
                4 => _mm256_alignr_epi8::<{ 16 - 2 * 4 }>(data, mask),
                _ => panic!("Invalid RIGHT: {}", RIGHT),
            }
        }
    } else {
        data
    };

    data
}

// ボードを左右下方向にシフトしてマージ
#[inline(always)]
pub fn move1(data: __m256i, free_space: __m256i) -> __m256i {
    // right
    let data = unsafe {
        let shift = shift::<0, 1, 0, 0>(data);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
    };

    // left
    let data = unsafe {
        let shift = shift::<1, 0, 0, 0>(data);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
    };

    // down
    unsafe {
        let shift = shift::<0, 0, 1, 0>(data);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
    }
}

#[inline(always)]
pub fn land(data: __m256i, free_space: __m256i) -> __m256i {
    unsafe {
        let shifted_free_space = shift::<0, 0, 0, 1>(free_space);
        _mm256_andnot_si256(shifted_free_space, data)
    }
}

#[inline(always)]
pub fn clip(data: __m256i, mask: u16) -> __m256i {
    unsafe {
        let mask = fill_with(mask as i16);
        _mm256_and_si256(data, mask)
    }
}

#[inline(always)]
pub fn or(left: __m256i, right: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(left, right) }
}

#[inline(always)]
pub fn and(left: __m256i, right: __m256i) -> __m256i {
    unsafe { _mm256_and_si256(left, right) }
}

#[inline(always)]
pub fn and_not(left: __m256i, right: __m256i) -> __m256i {
    unsafe { _mm256_andnot_si256(left, right) }
}

#[inline(always)]
pub fn extract(data: __m256i, x: i32) -> u16 {
    unsafe {
        (match x {
            0 => _mm256_extract_epi16::<0>(data),
            1 => _mm256_extract_epi16::<1>(data),
            2 => _mm256_extract_epi16::<2>(data),
            3 => _mm256_extract_epi16::<3>(data),
            4 => _mm256_extract_epi16::<4>(data),
            5 => _mm256_extract_epi16::<5>(data),
            6 => _mm256_extract_epi16::<6>(data),
            7 => _mm256_extract_epi16::<7>(data),
            8 => _mm256_extract_epi16::<8>(data),
            9 => _mm256_extract_epi16::<9>(data),
            _ => panic!("Index out of bounds: {}", x),
        }) as u16
    }
}

#[inline(always)]
pub fn data_to_byte(data: __m256i) -> AlignedU16s {
    let mut aligned = AlignedU16s::blank();
    unsafe {
        _mm256_store_si256(aligned.data.as_mut_ptr() as *mut __m256i, data);
    }
    aligned
}

#[inline(always)]
pub fn to_bytes_u16(data: __m256i) -> [u16; 10] {
    let data = data_to_byte(data).data;
    [
        data[0],
        data[1],
        data[2],
        data[3],
        data[4],
        data[5],
        data[6],
        data[7],
        data[8],
        data[9],
    ]
}

#[inline(always)]
pub fn to_bytes_u32(data: __m256i) -> [u32; 10] {
    let data = data_to_byte(data).data;
    [
        data[0] as u32,
        data[1] as u32,
        data[2] as u32,
        data[3] as u32,
        data[4] as u32,
        data[5] as u32,
        data[6] as u32,
        data[7] as u32,
        data[8] as u32,
        data[9] as u32,
    ]
}
