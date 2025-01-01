use std::arch::x86_64::*;

#[inline(always)]
pub fn load_from_unaligned(data: &[u8; 32]) -> __m256i {
    unsafe { _mm256_loadu_si256(data.as_ptr() as *const __m256i) }
}

#[inline(always)]
pub fn zeros() -> __m256i {
    unsafe { _mm256_setzero_si256() }
}

#[inline(always)]
pub fn fill_with(value: i32) -> __m256i {
    let a1 = value as i8;
    let a2 = (value >> 8) as i8;
    let a3 = (value >> 16) as i8;
    unsafe {
        _mm256_setr_epi8(
            a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1,
            a2, a3, a1, a2, a3, a1, a2, a3, 0, 0,
        )
    }
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

    debug_assert!(0 <= DOWN && DOWN <= 24);
    debug_assert!(0 <= UP && UP <= 24);
    debug_assert!((DOWN == 0 && UP == 0) || (0 < DOWN && UP == 0) || (0 < UP && DOWN == 0));

    if LEFT == 0 && RIGHT == 0 && DOWN == 0 && UP == 0 {
        return data;
    }

    if DOWN == 24 || UP == 24 {
        return zeros();
    }

    // down or up // FIXME
    let data = if 0 < DOWN {
        unsafe {
            // 64bitごとに右シフト
            let shifted1 = _mm256_srli_epi64::<DOWN>(data);

            // 全体を32bit左シフトしてから、64bitごとに右シフトして、元に戻す
            let shifted2 = srli32(_mm256_srli_epi64::<DOWN>(slli32(data)));

            // レーンを繰り越した上でDOWNだけ右シフトされたビット列
            let shifted = or(shifted1, shifted2);

            // 24bitごとの上位ビットをMASK
            clip(shifted, u32::MAX >> (DOWN + 8))
        }
    } else if 0 < UP {
        unsafe {
            // 64bitごとに左シフト
            let shifted1 = _mm256_slli_epi64::<UP>(data);

            // 全体を32bit左シフトしてから、64bitごとに左シフトして、元に戻す
            let shifted2 = srli32(_mm256_slli_epi64::<UP>(slli32(data)));

            // レーンを繰り越した上でUPだけ左シフトされたビット列
            let shifted = or(shifted1, shifted2);

            // 24bitごとの下位ビットをMASK
            clip(shifted, u32::MAX << UP)
        }
    } else {
        data
    };

    // left or right
    let data = if 0 < LEFT {
        unsafe {
            // 上位ビットの初期化
            let data = _mm256_insert_epi64::<3>(data, 0);
            let data = _mm256_insert_epi32::<7>(data, 0);

            // レジスタ上では右シフト
            // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
            // mask = | 0(128bit) A(128bit; x=8~9) |  < 0x81
            // a    = | A(128bit; x=8~9) B(128bit; x=0~7) |
            // b    = | 0(128bit) A(128bit; x=8~9) |
            // シフトは最大8まで
            let mask = _mm256_permute2x128_si256::<0x81>(data, data);
            match LEFT {
                1 => _mm256_alignr_epi8::<{ 3 * 1 }>(mask, data),
                2 => _mm256_alignr_epi8::<{ 3 * 2 }>(mask, data),
                3 => _mm256_alignr_epi8::<{ 3 * 3 }>(mask, data),
                4 => _mm256_alignr_epi8::<{ 3 * 4 }>(mask, data),
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
                1 => _mm256_alignr_epi8::<{ 16 - 3 * 1 }>(data, mask),
                2 => _mm256_alignr_epi8::<{ 16 - 3 * 2 }>(data, mask),
                3 => _mm256_alignr_epi8::<{ 16 - 3 * 3 }>(data, mask),
                4 => _mm256_alignr_epi8::<{ 16 - 3 * 4 }>(data, mask),
                _ => panic!("Invalid RIGHT: {}", RIGHT),
            }
        }
    } else {
        data
    };

    data
}

// レジスタ上全体で8bit分右シフト
#[inline(always)]
fn srli32(data: __m256i) -> __m256i {
    unsafe {
        // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
        // mask = | 0(128bit) A(128bit; x=8~9) |  < 0x81
        // a    = | A(128bit; x=8~9) B(128bit; x=0~7) |
        // b    = | 0(128bit) A(128bit; x=8~9) |
        let mask = _mm256_permute2x128_si256::<0x81>(data, data);
        _mm256_alignr_epi8::<4>(mask, data)
    }
}

// レジスタ上全体で8bit分左シフト
#[inline(always)]
fn slli32(data: __m256i) -> __m256i {
    unsafe {
        // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
        // mask = | B(128bit; x=0~7) 0(128bit) |  < 0x08
        let mask = _mm256_permute2x128_si256::<0x08>(data, data);
        _mm256_alignr_epi8::<{ 16 - 4 }>(data, mask)
    }
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
pub fn clip(data: __m256i, mask: u32) -> __m256i {
    unsafe {
        let mask = fill_with(mask as i32);
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
            0 => _mm256_extract_epi16::<0>(data) | (_mm256_extract_epi8::<2>(data) << 16),
            1 => _mm256_extract_epi8::<3>(data) | (_mm256_extract_epi16::<2>(data) << 8),
            2 => _mm256_extract_epi16::<3>(data) | (_mm256_extract_epi8::<8>(data) << 16),
            3 => _mm256_extract_epi8::<9>(data) | (_mm256_extract_epi16::<5>(data) << 8),
            4 => _mm256_extract_epi16::<6>(data) | (_mm256_extract_epi8::<14>(data) << 16),
            5 => _mm256_extract_epi8::<15>(data) | (_mm256_extract_epi16::<8>(data) << 8),
            6 => _mm256_extract_epi16::<9>(data) | (_mm256_extract_epi8::<20>(data) << 16),
            7 => _mm256_extract_epi8::<21>(data) | (_mm256_extract_epi16::<11>(data) << 8),
            8 => _mm256_extract_epi16::<12>(data) | (_mm256_extract_epi8::<26>(data) << 16),
            9 => _mm256_extract_epi8::<27>(data) | (_mm256_extract_epi16::<14>(data) << 8),
            _ => panic!("Index out of bounds: {}", x),
        }) as u16
    }
}

#[inline(always)]
pub fn data_to_byte(data: __m256i) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    unsafe {
        _mm256_storeu_si256(bytes.as_mut_ptr() as *mut __m256i, data);
    }
    bytes
}

#[inline(always)]
pub fn to_bytes_u32(data: __m256i) -> [u32; 10] {
    let data = data_to_byte(data);
    [
        (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16,
        (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16,
        (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16,
        (data[9] as u32) | (data[10] as u32) << 8 | (data[11] as u32) << 16,
        (data[12] as u32) | (data[13] as u32) << 8 | (data[14] as u32) << 16,
        (data[15] as u32) | (data[16] as u32) << 8 | (data[17] as u32) << 16,
        (data[18] as u32) | (data[19] as u32) << 8 | (data[20] as u32) << 16,
        (data[21] as u32) | (data[22] as u32) << 8 | (data[23] as u32) << 16,
        (data[24] as u32) | (data[25] as u32) << 8 | (data[26] as u32) << 16,
        (data[27] as u32) | (data[28] as u32) << 8 | (data[29] as u32) << 16,
    ]
}
