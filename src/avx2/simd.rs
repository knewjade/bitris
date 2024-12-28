use std::arch::x86_64::*;

#[inline(always)]
pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
    data: __m256i,
) -> __m256i {
    debug_assert!(0 <= LEFT && LEFT <= 4);
    debug_assert!(0 <= RIGHT && RIGHT <= 4);
    debug_assert!((LEFT == 0 && RIGHT == 0) || (0 < LEFT && RIGHT == 0) || (0 < RIGHT && LEFT == 0));

    debug_assert!(0 <= DOWN && DOWN <= 4);
    debug_assert!(0 <= UP && UP <= 4);
    debug_assert!((DOWN == 0 && UP == 0) || (0 < DOWN && UP == 0) || (0 < UP && DOWN == 0));

    if LEFT == 0 && RIGHT == 0 && DOWN == 0 && UP == 0 {
        return data;
    }

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

    // down or up
    let data = if 0 < DOWN {
        unsafe { _mm256_srli_epi16::<DOWN>(data) }
    } else if 0 < UP {
        unsafe { _mm256_slli_epi16::<UP>(data) }
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
        let mask = _mm256_permute2x128_si256::<0x08>(data, data);
        let shift = _mm256_alignr_epi8::<{ 16 - 2 * 1 }>(data, mask);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
    };

    // left
    let data = unsafe {
        let data = _mm256_insert_epi64::<3>(data, 0);
        let data = _mm256_insert_epi32::<5>(data, 0);
        let mask = _mm256_permute2x128_si256::<0x81>(data, data);
        let shift = _mm256_alignr_epi8::<2>(mask, data);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
    };

    // down
    unsafe {
        let shift = _mm256_srli_epi16::<1>(data);
        let candidate = _mm256_and_si256(free_space, shift);
        _mm256_or_si256(data, candidate)
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
pub fn to_bytes_u16(data: __m256i) -> [u16; 10] {
    let data = data_to_byte(data);
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
