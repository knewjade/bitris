#pragma once

#include <assert.h>
#include <iostream>
// #include <simdpp/simd.h>
#include <experimental/simd>

namespace stdx = std::experimental;

#include "pieces.hpp"
#include "board.hpp"

namespace s {
    constexpr int set_at_constexpr(const int dy) {
        auto board = Board64{};

        for (uint32_t index = 0; index < (Board64::ceiling() / dy); ++index) {
            const auto y = dy * index;
            static_for<10>([&](auto x) {
                board.set_at(x, y);
            });
        }

        board.clear_lines();

        return static_cast<int>(board.count_blocks());
    }

    using CallbackType = void(*)(const CcPlacement &);

    template<typename Callback>
    constexpr void search_v1(const Callback callback) {
        callback(CcPlacement{0, 1});
        callback(CcPlacement{1, 2});
        callback(CcPlacement{2, 3});
    }

    namespace u16 {
        consteval size_t ceiling() {
            return 16;
        }

        consteval uint16_t full() {
            return std::numeric_limits<uint16_t>::max();
        }
    }

    constexpr uint32_t getMostSignificantBitUsingBuiltin(const uint32_t v) {
        if (v == 0) {
            return 0;
        }
        return 31 - __builtin_clz(v);
    }

    constexpr uint16_t spawn_bits(
        const std::array<uint16_t, 10> &board_bytes,
        const size_t cy
    ) {
        const uint16_t used_rows = static_packing_fold(
            []<typename... T>(T... s) {
                return (s | ...);
            },
            board_bytes
        );

        const uint16_t spawn_mask = 16 <= cy ? (1 << (cy + 1)) - 1 : 0xFFFF;
        const uint16_t masked_used_rows = used_rows & spawn_mask;

        const uint16_t builtin_clz = (1U << (getMostSignificantBitUsingBuiltin(masked_used_rows))) - 1;
        assert(builtin_clz <= spawn_mask);
        const uint16_t reachable_rows = spawn_mask - builtin_clz;

        return reachable_rows;
    }

    static constexpr int W = 10;
    static constexpr int width = W;
    static constexpr int height = 16;

    using under_t = std::uint16_t;

    template<typename T>
    using data_t = stdx::simd<T, stdx::simd_abi::fixed_size<10> >;

    // alignas(std::experimental::memory_alignment_v<data_t>) data_t data = 0;

    template<typename T, T Value>
    constexpr data_t<T> make_square() {
        return data_t{Value};
    }

    template<typename T>
    constexpr T make_square(const T value) {
        return data_t{value};
    }

    template<size_t Down, bool CeilOpen = false>
    constexpr data_t shift_down(const data_t &data) {
        if constexpr (Down == 0) {
            return data;
        }
        if constexpr (16 <= Down) {
            make_square<CeilOpen ? u16::full() : 0>();
        }
        return CeilOpen ? ~(~data >> Down) : data >> Down;
    }

    template<size_t Up>
    constexpr data_t shift_up(const data_t &data) {
        if constexpr (Up == 0) {
            return data;
        }
        if constexpr (16 <= Up) {
            return make_square<0>();
        }
        return data << Up;
    }

    template<size_t Right>
    constexpr data_t shift_right(const data_t &data) {
        if constexpr (Right == 0) {
            return data;
        }
        if constexpr (10 <= Right) {
            return make_square<0>();
        }

        return data_t([=][[gnu::always_inline]](auto i) {
            if constexpr (constexpr int32_t index = (int32_t)(i) - (int32_t)(Right); index < 0) {
                return 0;
            } else {
                return data[index];
            }
        });

        // alignas(32) std::array<uint16_t, 10 + Right> mem{};
        // data.copy_to(&mem[Right], stdx::vector_aligned);
        // return data_t{mem.data(), stdx::vector_aligned};
    }

    template<size_t Left>
    constexpr data_t shift_left(const data_t &data) {
        if constexpr (Left == 0) {
            return data;
        }

        if constexpr (10 <= Left) {
            return make_square<0>();
        }

        return data_t([=][[gnu::always_inline]](auto i) {
            if constexpr (constexpr size_t index = i + Left; index >= 10) {
                return 0;
            } else {
                return data[index];
            }
        });
        // alignas(32) std::array<uint16_t, 10 + Left> mem{};
        // data.copy_to(&mem[0], stdx::vector_aligned);
        // return data_t{mem.data() + Left, stdx::vector_aligned};
    }

    constexpr void show(const data_t &data) {
        std::array<uint64_t, 10> g2{};
        for (int i = 0; i < 10; ++i) {
            g2[i] = data[i];
        }
        const auto board2 = Board64{g2};
        std::cout << board2.to_string(16) << std::endl;
    }

    constexpr data_t search2(
        const std::array<uint16_t, 10> &board_bytes,
        const data_t &board,
        uint8_t spawn_piece,
        uint8_t spawn_orientation,
        uint8_t spawn_cx,
        uint8_t spawn_cy
    ) {
        const auto free_space_block = ~board;

        const auto a = shift_left<1>(free_space_block);
        const auto free_space = free_space_block & shift_down<1, true>(a) & a & shift_down<1, true>(free_space_block);

        const uint16_t value = spawn_bits(board_bytes, spawn_cy);
        data_t reachable;
        if (0 < value) {
            reachable = make_square(value) & free_space;
        } else if (16 <= spawn_cy) {
            reachable = make_square<1 << 15>() & free_space;
        } else {
            alignas(32) std::array<uint16_t, 10> b{};
            b[spawn_cx] = 1 << spawn_cy;
            reachable = data_t{b.data(), stdx::vector_aligned};
        }

        while (true) {
            const auto right = shift_right<1>(reachable);
            const auto left = shift_left<1>(reachable);
            const auto down = shift_down<1>(reachable);

            const auto next = (reachable | right | left | down) & free_space;

            if (all_of(next == reachable)) {
                break;
            }

            reachable = next;
        };

        const auto goal = ~shift_up<1>(free_space) & reachable;

        // std::array<uint16_t, 10> result{};
        // store_first(&result, goal, 10);

        return goal;
    }
}
