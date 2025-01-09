#pragma once

#include <assert.h>
#include <iostream>
#include <cstddef>

#include "pieces.hpp"
#include "board.hpp"

// #ifdef _MSC_VER
// #include <intrin.h>
// #else
// #include <x86intrin.h>
// #endif

using size_t = std::size_t;

namespace s2 {
    namespace u16 {
        consteval size_t ceiling() {
            return 16;
        }

        consteval uint16_t full() {
            return std::numeric_limits<uint16_t>::max();
        }
    }

    // template<uint16_t Value>
    // constexpr __m256i make_square2() {
    //     if (Value == 0) {
    //         return _mm256_setzero_si256();
    //     }
    //     return _mm256_setr_epi8(
    //         Value, Value, Value, Value, Value,
    //         Value, Value, Value, Value, Value,
    //         0, 0, 0, 0, 0, 0
    //     );
    // }
    //
    // constexpr __m256i make_square2(const uint16_t value) {
    //     if (value == 0) {
    //         return _mm256_setzero_si256();
    //     }
    //     return _mm256_setr_epi8(
    //         value, value, value, value, value,
    //         value, value, value, value, value,
    //         0, 0, 0, 0, 0, 0
    //     );
    // }
    //
    // template<size_t Down, bool CeilOpen = false>
    // constexpr __m256i shift_down(const __m256i &data) {
    //     if constexpr (Down == 0) {
    //         return data;
    //     }
    //     if constexpr (16 <= Down) {
    //         make_square2<CeilOpen ? u16::full() : 0>();
    //     }
    //     if (CeilOpen) {
    //         return _mm256_or_si256(_mm256_slli_epi16<Down>(data), make_square2<(u16::full() >> (16 - Down)) << (16 - Down)>());
    //     } else {
    //         return _mm256_slli_epi16<Down>(data);
    //     }
    //     return CeilOpen ? ~(~data >> Down) : data >> Down;
    // }
    //
    // template<size_t Up>
    // constexpr __m256i shift_up(const __m256i &data) {
    //     if constexpr (Up == 0) {
    //         return data;
    //     }
    //     if constexpr (16 <= Up) {
    //         return _mm256_setzero_si256();
    //     }
    //     return _mm256_slli_epi16<Up>(data);
    // }
    //
    // template<size_t Right>
    // constexpr __m256i shift_right(const __m256i &data) {
    //     if constexpr (Right == 0) {
    //         return data;
    //     }
    //     if constexpr (10 <= Right) {
    //         return _mm256_setzero_si256();
    //     }
    //
    //     auto mask = _mm256_permute2x128_si256<0x08>(data, data);
    //     return _mm256_alignr_epi8<16 - 2 * 1>(data, mask);
    // }
    //
    // template<size_t Left>
    // constexpr __m256i shift_left(const __m256i &data) {
    //     if constexpr (Left == 0) {
    //         return data;
    //     }
    //
    //     if constexpr (10 <= Left) {
    //         return _mm256_setzero_si256();
    //     }
    //
    //     // 上位ビットの初期化
    //     auto data = _mm256_insert_epi64<3>(data, 0);
    //     auto data = _mm256_insert_epi32<5>(data, 0);
    //
    //     // レジスタ上では右シフト
    //     // data = | A(128bit; x=8~9) B(128bit; x=0~7) | とすると、
    //     // mask = | 0(128bit) A(128bit; x=8~9) |  < 0x81
    //     // a    = | A(128bit; x=8~9) B(128bit; x=0~7) |
    //     // b    = | 0(128bit) A(128bit; x=8~9) |
    //     // シフトは最大8まで
    //     auto mask = _mm256_permute2x128_si256<0x81>(data, data);
    //     return _mm256_alignr_epi8<2 * 1>(mask, data);
    // }
    //
    // constexpr uint32_t getMostSignificantBitUsingBuiltin(const uint32_t v) {
    //     if (v == 0) {
    //         return 0;
    //     }
    //     return 31 - __builtin_clz(v);
    // }
    //
    // constexpr uint16_t spawn_bits(
    //     const std::array<uint16_t, 10> &board_bytes,
    //     const size_t cy
    // ) {
    //
    //     const uint16_t used_rows = static_packing_fold(
    //         []<typename... T>(T... s) {
    //             return (s | ...);
    //         },
    //         board_bytes
    //     );
    //
    //     const uint16_t spawn_mask = 16 <= cy ? (1 << cy + 1) - 1 : 0xFFFF;
    //     const uint16_t masked_used_rows = used_rows & spawn_mask;
    //
    //     const uint16_t builtin_clz = (1U << (getMostSignificantBitUsingBuiltin(masked_used_rows))) - 1;
    //     assert(builtin_clz <= spawn_mask);
    //     const uint16_t reachable_rows = spawn_mask - builtin_clz;
    //
    //     return reachable_rows;
    // }

    constexpr __m256i search(
        const std::array<uint16_t, 10> &board_bytes,
        const __m256i &board,
        uint8_t spawn_piece,
        uint8_t spawn_orientation,
        uint8_t spawn_cx,
        uint8_t spawn_cy
    ) {
        // const uint16x16 free_space_block = bit_not(board);
        //
        // const uint16x16 a = shift_left<1>(free_space_block);
        // const uint16x16 b = bit_and(a, shift_down<1>(free_space_block));
        // const uint16x16 c = bit_and(free_space_block, shift_down<1>(a));
        // const uint16x16 free_space = bit_and(b, c);

        // const uint16_t value = spawn_bits(board_bytes, spawn_cy);
        // uint16x16 reachable;
        // if (0 < value) {
        //     reachable = make_square(value);
        // } else {
        //     alignas(32) std::array<uint16_t, 10> b{};
        //     b[spawn_cx] = 1 << spawn_cy;
        //     reachable = load(b.data());
        // }
        //
        // while (true) {
        //     const uint16x16 right = shift_right<1>(reachable);
        //     const uint16x16 left = shift_left<1>(reachable);
        //     const uint16x16 down = shift_down<1>(reachable);
        //
        //     const uint16x16 next = bit_or(
        //         bit_or(reachable, right),
        //         bit_or(left, down)
        //     );
        //
        //     if (const int memcmp1 = memcmp(&next, &next, 10); memcmp1 == 0) {
        //         break;
        //     }
        //
        //     reachable = next;
        // };
        //
        // const uint16x16 goal = bit_andnot(
        //     reachable,
        //     shift_up<1>(free_space)
        // );

        // std::array<uint16_t, 10> result{};
        // store_first(&result, goal, 10);

        return board;
    }
}

