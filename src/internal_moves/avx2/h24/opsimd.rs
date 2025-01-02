use crate::internal_moves::avx2::h24::aligned::AlignedU24s;
use crate::placements::CcPlacement;
use crate::prelude::Location;
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
pub fn ones() -> __m256i {
    unsafe { _mm256_set1_epi8(0xFFu8 as i8) }
}

#[inline(always)]
pub fn fill_with(value: i32) -> __m256i {
    let a1 = value as i8;
    let a2 = (value >> 8) as i8;
    let a3 = (value >> 16) as i8;
    unsafe {
        _mm256_setr_epi8(
            0, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3,
            a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, a1, a2, a3, 0, 
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

// すべてが1の行を取り出す
#[inline(always)]
fn empty_lines(data: __m256i) -> u32 {
    const FULL: i8 = 0xFFu16 as i8;
    const CLEAR: i8 = 0xF0u8 as i8;

    unsafe {
        let empties = data;

        // 下位128bit: x=3にその上のx=4を反映
        // 上位128bit: x=8にその上のx=9を反映
        let empties = and(
            empties,
            _mm256_shuffle_epi8(
                empties,
                _mm256_setr_epi8(
                    CLEAR, 1, 2, 3, 4, 5, 6, 7, 8, 9, 13, 14, 15, CLEAR, CLEAR, CLEAR,
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 13, 14, CLEAR, CLEAR, CLEAR, CLEAR,
                ),
            ),
        );

        // 下位128bit: x=0..1にその上のx=2..3を反映
        // 上位128bit: x=5..6にその上のx=7..8を反映
        let empties = and(
            empties,
            _mm256_shuffle_epi8(
                empties,
                _mm256_setr_epi8(
                    CLEAR, 7, 8, 9, 10, 11, 12, CLEAR, CLEAR, CLEAR,CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR,
                    6, 7, 8, 9, 10, 11, CLEAR, CLEAR, CLEAR, CLEAR,CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR,
                ),
            ),
        );

        // 下位128bit: x=0にその上のx=1を反映
        // 上位128bit: x=5にその上のx=6を反映
        let empties = and(
            empties,
            _mm256_shuffle_epi8(
                empties,
                _mm256_setr_epi8(
                    CLEAR, 4, 5, 6, CLEAR, CLEAR, CLEAR,CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR,
                    3, 4, 5, CLEAR, CLEAR, CLEAR,CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR, CLEAR,
                ),
            ),
        );

        // 2つのレーンの下位16bitを直接反映
        extract(empties, 0) & extract(empties, 8)
    }
}

#[inline(always)]
pub fn spawn(spawn: CcPlacement, free_space_block: __m256i, free_space: __m256i) -> __m256i {
    unsafe {
        // spawn以下の行を取り出すmask
        let spawn_mask = (1 << (spawn.position.cy as usize + 1)) - 1;
        let masked_empties = empty_lines(free_space_block) & spawn_mask;

        // 上から連続した1を取り出す (空ではない行までは無条件で移動可能とする)
        // ビット列を反転して、最も下位にある連続した1を取り出して、再度ビット列を反転
        let bits = masked_empties.reverse_bits();
        let lowest_bit_mask = bits & (-(bits as i32) as u32);
        let removed_lowest_ones = bits.wrapping_add(lowest_bit_mask);
        let bits = bits & !removed_lowest_ones;
        let reachable_lines = bits.reverse_bits();

        if 0 < reachable_lines {
            and(fill_with(reachable_lines as i32), free_space)
        } else {
            AlignedU24s::blank()
                .set_at(spawn.position.to_location())
                .load()
        }
    }
}

// 上端が空いている場合は `Opened=true` を指定する。シフト時にfreeが挿入される
#[inline(always)]
#[allow(clippy::nonminimal_bool)]
pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32, const Opened: bool>(
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

    if UP == 24 {
        return zeros();
    }

    if DOWN == 24 {
        return if Opened {
            ones()
        } else {
            zeros()
        }
    }

    // down or up
    let data = if 0 < DOWN {
        unsafe {
            // 64bitごとに右シフト
            let shifted1 = _mm256_srli_epi64::<DOWN>(data);

            // 全体を32bit左シフトしてから、64bitごとに右シフトして、元に戻す
            let shifted2 = srli32(_mm256_srli_epi64::<DOWN>(slli32(data)));

            // レーンを繰り越した上でDOWNだけ右シフトされたビット列
            let shifted = or(shifted1, shifted2);

            // 24bitごとの上位ビットをMASK
            let clipped = clip(shifted, u32::MAX >> (DOWN + 8));

            if Opened {
                let adding = (0xFFFFFFu32 >> (24 - DOWN)) << (24 - DOWN);
                or(clipped, fill_with(adding as i32))
            } else {
                clipped
            }
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
            let data = _mm256_insert_epi8::<31>(data, 0);

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
            // 下位ビットの初期化
            let data = _mm256_insert_epi8::<0>(data, 0);

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
    let candidate = unsafe {
        shift::<0, 1, 0, 0, false>(data)
    };

    // left
    let candidate = unsafe {
        let shift = shift::<1, 0, 0, 0, false>(data);
        _mm256_or_si256(candidate, shift)
    };

    // down
    let candidate = unsafe {
        let shift = shift::<0, 0, 1, 0, false>(data);
        _mm256_or_si256(candidate, shift)
    };

    unsafe {
        _mm256_or_si256(data, _mm256_and_si256(free_space, candidate))
    }
}

#[inline(always)]
pub fn land(data: __m256i, free_space: __m256i) -> __m256i {
    unsafe {
        let shifted_free_space = shift::<0, 0, 0, 1, false>(free_space);
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
pub fn extract(data: __m256i, x: i32) -> u32 {
    unsafe {
        match x {
            0 => _mm256_extract_epi8::<1>(data) as u32 | (_mm256_extract_epi16::<1>(data) << 8) as u32,
            1 => _mm256_extract_epi16::<2>(data) as u32 | (_mm256_extract_epi8::<6>(data) << 16) as u32,
            2 => _mm256_extract_epi8::<7>(data) as u32 | (_mm256_extract_epi16::<4>(data) << 8) as u32,
            3 => _mm256_extract_epi16::<5>(data) as u32 | (_mm256_extract_epi8::<12>(data) << 16) as u32,
            4 => _mm256_extract_epi8::<13>(data) as u32 | (_mm256_extract_epi16::<7>(data) << 8) as u32,
            5 => _mm256_extract_epi16::<8>(data) as u32 | (_mm256_extract_epi8::<18>(data) << 16) as u32,
            6 => _mm256_extract_epi8::<19>(data) as u32 | (_mm256_extract_epi16::<10>(data) << 8) as u32,
            7 => _mm256_extract_epi16::<11>(data) as u32 | (_mm256_extract_epi8::<24>(data) << 16) as u32,
            8 => _mm256_extract_epi8::<25>(data) as u32 | (_mm256_extract_epi16::<13>(data) << 8) as u32,
            9 => _mm256_extract_epi16::<14>(data) as u32 | (_mm256_extract_epi8::<30>(data) << 16) as u32,
            _ => panic!("Index out of bounds: {}", x),
        }
    }
}

#[inline(always)]
pub fn is_one_at(data: __m256i, location: Location) -> bool {
    let index = (location.x * 3) + (location.y % 8);
    let value = unsafe {
        (match index {
            0 => _mm256_extract_epi16::<1>(data),
            1 => _mm256_extract_epi16::<2>(data),
            2 => _mm256_extract_epi16::<3>(data),
            3 => _mm256_extract_epi16::<4>(data),
            4 => _mm256_extract_epi16::<5>(data),
            5 => _mm256_extract_epi16::<6>(data),
            6 => _mm256_extract_epi16::<7>(data),
            7 => _mm256_extract_epi16::<8>(data),
            8 => _mm256_extract_epi16::<9>(data),
            9 => _mm256_extract_epi16::<10>(data),
            10 => _mm256_extract_epi16::<11>(data),
            11 => _mm256_extract_epi16::<12>(data),
            12 => _mm256_extract_epi16::<13>(data),
            13 => _mm256_extract_epi16::<14>(data),
            14 => _mm256_extract_epi16::<15>(data),
            15 => _mm256_extract_epi16::<16>(data),
            16 => _mm256_extract_epi8::<17>(data),
            17 => _mm256_extract_epi8::<18>(data),
            18 => _mm256_extract_epi8::<19>(data),
            19 => _mm256_extract_epi8::<20>(data),
            20 => _mm256_extract_epi8::<21>(data),
            21 => _mm256_extract_epi8::<22>(data),
            22 => _mm256_extract_epi8::<23>(data),
            23 => _mm256_extract_epi8::<24>(data),
            24 => _mm256_extract_epi8::<25>(data),
            25 => _mm256_extract_epi8::<26>(data),
            26 => _mm256_extract_epi8::<27>(data),
            27 => _mm256_extract_epi8::<28>(data),
            28 => _mm256_extract_epi8::<29>(data),
            29 => _mm256_extract_epi8::<30>(data),
            30 => _mm256_extract_epi8::<31>(data),
            _ => panic!("Index out of bounds: {}", index),
        }) as u8
    };
    let mask = 1 << (location.y / 8);
    value & mask != 0
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
        (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16,
        (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16,
        (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16,
        (data[10] as u32) | (data[11] as u32) << 8 | (data[12] as u32) << 16,
        (data[13] as u32) | (data[14] as u32) << 8 | (data[15] as u32) << 16,
        (data[16] as u32) | (data[17] as u32) << 8 | (data[18] as u32) << 16,
        (data[19] as u32) | (data[20] as u32) << 8 | (data[21] as u32) << 16,
        (data[22] as u32) | (data[23] as u32) << 8 | (data[24] as u32) << 16,
        (data[25] as u32) | (data[26] as u32) << 8 | (data[27] as u32) << 16,
        (data[28] as u32) | (data[29] as u32) << 8 | (data[30] as u32) << 16,
    ]
}
