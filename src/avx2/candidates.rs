use crate::avx2::reachable::ReachableSimd16;
use crate::avx2::free_space::FreeSpaceSimd16;
use std::arch::x86_64::{
    _mm256_and_si256, _mm256_set1_epi16,
};

pub(crate) fn reachable_starting_celling(
    free_spaces: &[FreeSpaceSimd16; 4],
) -> [ReachableSimd16; 4] {
    unsafe {
        let data = _mm256_set1_epi16(1i16 << 15);
        [
            ReachableSimd16::new(_mm256_and_si256(free_spaces[0].data, data)),
            ReachableSimd16::new(_mm256_and_si256(free_spaces[1].data, data)),
            ReachableSimd16::new(_mm256_and_si256(free_spaces[2].data, data)),
            ReachableSimd16::new(_mm256_and_si256(free_spaces[3].data, data)),
        ]
    }
}
