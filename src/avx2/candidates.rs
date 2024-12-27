use std::arch::x86_64::{__m256i, _mm256_and_si256, _mm256_load_si256, _mm256_set1_epi16, _mm256_set_epi16};
use crate::avx2::aligned::AlignedU8s;
use crate::avx2::board::BoardSimd;
use crate::boards::Board;
use crate::pieces::Shape;

pub(crate) fn reachable_starting_celling(free_spaces: &[BoardSimd; 4]) -> [BoardSimd; 4] {
    unsafe {
        let data = _mm256_set1_epi16(1i16 << 15);
        [
            BoardSimd::new(_mm256_and_si256(free_spaces[0].data, data)),
            BoardSimd::new(_mm256_and_si256(free_spaces[1].data, data)),
            BoardSimd::new(_mm256_and_si256(free_spaces[2].data, data)),
            BoardSimd::new(_mm256_and_si256(free_spaces[3].data, data)),
        ]
    }
}
