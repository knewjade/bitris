#pragma once

#include <ranges>
#include <experimental/simd>

#include "bits.hpp"
#include "kicks.hpp"

namespace stdx = std::experimental;

template<typename T>
struct data {
    // TODO deduce?
    using type = stdx::simd<T, stdx::simd_abi::fixed_size<10> >;
    // using type = stdx::simd<T, stdx::simd_abi::deduce_t<T, 10> >;
    using bits_t = bits<T>;

    template<typename U>
    [[gnu::always_inline]]
    static constexpr typename data<U>::type load(const std::array<T, 10> &board_bytes) {
        if constexpr (std::is_same_v<T, U>) {
            return typename data<U>::type{board_bytes.data(), stdx::vector_aligned};
        } else {
            return typename data<U>::type([=][[gnu::always_inline]](auto i) {
                if constexpr (constexpr size_t index = i; index < 0) {
                    return 0;
                } else {
                    return static_cast<U>(board_bytes[index]);
                }
            });
        }
    }

    [[gnu::always_inline]]
    static constexpr type make_zero() {
        return make_square<0>();
    }

    template<T Value>
    [[gnu::always_inline]]
    static constexpr type make_square() {
        return type{Value};
    }

    [[gnu::always_inline]]
    static constexpr type make_square(const T value) {
        return type{value};
    }

    template<typename U>
    [[gnu::always_inline]]
    static constexpr typename data<U>::type to(const type &board) {
        return typename data<U>::type([=][[gnu::always_inline]](size_t i) {
            return static_cast<U>(board[i]);
        });
    }

    [[gnu::always_inline]]
    static constexpr type make_spawn(
            const type &free_space,
            const uint8_t spawn_cx,
            const uint8_t spawn_cy
    ) {
        if (is_continuous_line(free_space, spawn_cy)) {
            return make_spawn2(free_space, spawn_cy);
        }

        alignas(32) std::array<T, 10> b{};
        b[spawn_cx] = bits_t::one << spawn_cy;
        return type{b.data(), stdx::vector_aligned};
    }

    [[gnu::always_inline]]
    static constexpr bool is_continuous_line(
            const type &free_space,
            const uint8_t y
    ) {
        if (bits_t::bit_size <= y) {
            return true;
        }

        size_t count = 0;
        auto prev = bits_t::zero;
        const auto m = bits_t::one << y;
        static_for_t<10>([&]<size_t Index>() {
            const auto bit = free_space[Index] & m;
            if (prev && (prev ^ bit)) {
                count++;
            }
            prev = bit;
        });
        count += prev ? 1 : 0;
        return count <= 1;
    }

    // TODO rename
    [[gnu::always_inline]]
    static constexpr type make_spawn2(
            const type &free_space,
            const uint8_t spawn_cy
    ) {
        const auto a = (~free_space >> 1) & free_space;
        // show(a);

        const auto b = static_fold_t<10>([&]<size_t Index>(const auto acc) {
            return acc | a[Index];
        }, bits_t::zero);
        const int k = bits_t::bit_size - spawn_cy - 1;
        // std::cout << "k: " << std::to_string(k) << std::endl;

        const auto mask2 = 0 < k ? static_cast<T>(bits_t::full << k) >> k : bits_t::full;
        // std::cout << "mask2: " << std::hex << mask2 << std::endl;

        if (b == 0) {
            return make_square(mask2) & free_space;
        }

        const auto most_significant_index = bits_t::most_significant_index(b & (mask2 >> 1));
        // std::cout << "most_significant_index: " << most_significant_index << std::endl;
        const auto s = most_significant_index + 1;
        const auto mask = (bits_t::full >> s) << s;
        // std::cout << "mask: " << std::hex << mask << std::endl;

        const auto c = mask & mask2;
        // std::cout << "c: " << std::hex << c << std::endl;

        return make_square(c) & free_space;
    }

    template<size_t Down, bool CeilOpen = false>
    [[gnu::always_inline]]
    static constexpr type shift_down(const type &data) {
        if constexpr (Down == 0) {
            return data;
        }
        if constexpr (bits_t::bit_size <= Down) {
            make_square<CeilOpen ? bits_t::full : 0>();
        }
        return CeilOpen ? ~(~data >> Down) : data >> Down;
    }

    template<size_t Up>
    [[gnu::always_inline]]
    static constexpr type shift_up(const type &data) {
        if constexpr (Up == 0) {
            return data;
        }
        if constexpr (bits_t::bit_size <= Up) {
            return make_square<0>();
        }
        return data << Up;
    }

    template<size_t Right>
    [[gnu::always_inline]]
    static constexpr type shift_right(const type &data) {
        if constexpr (Right == 0) {
            return data;
        }
        if constexpr (10 <= Right) {
            return make_square<0>();
        }

        return type([=][[gnu::always_inline]](auto i) {
            if constexpr (constexpr auto index = static_cast<int>(i) - static_cast<int>(Right); index < 0) {
                return 0;
            } else {
                return data[index];
            }
        });
    }

    template<size_t Left>
    [[gnu::always_inline]]
    static constexpr type shift_left(const type &data) {
        if constexpr (Left == 0) {
            return data;
        }

        if constexpr (10 <= Left) {
            return make_square<0>();
        }

        return type([=][[gnu::always_inline]](auto i) {
            if constexpr (constexpr size_t index = i + Left; index >= 10) {
                return 0;
            } else {
                return data[index];
            }
        });
    }

    template<Offset Offset>
    [[gnu::always_inline]]
    static constexpr type shift(const type &data) {
        constexpr auto shift_vertical = [][[gnu::always_inline]](const auto &v) {
            if constexpr (0 < Offset.y) {
                constexpr auto Up = static_cast<size_t>(Offset.y);
                return shift_up<Up>(v);
            } else if constexpr (Offset.y < 0) {
                constexpr auto Down = static_cast<size_t>(-Offset.y);
                return shift_down<Down>(v);
            } else {
                return v;
            }
        };

        if constexpr (0 < Offset.x) {
            constexpr auto Right = static_cast<size_t>(Offset.x);
            return shift_vertical(shift_right<Right>(data));
        } else if constexpr (Offset.x < 0) {
            constexpr auto Left = static_cast<size_t>(-Offset.x);
            return shift_vertical(shift_left<Left>(data));
        } else {
            return shift_vertical(data);
        }
    }

    static constexpr void show(const type &data, const int height = bits_t::bit_size) {
        std::string str;
        for (int y = height - 1; y >= 0; --y) {
            for (int x = 0; x < 10; ++x) {
                auto is_occupied_at = data[x] & (bits_t::one << y);
                str += is_occupied_at ? '#' : '.';
            }
            str += '\n';
        }
        std::cout << str << std::endl;
    }

    static constexpr std::optional<type> from_str(const std::string &str) {
        type board = make_zero();
        const int ceiling = bits_t::bit_size;
        int index = 0;

        for (const char ch: std::ranges::reverse_view(str)) {
            switch (ch) {
                case '#':
                case 'X':
                    if (index >= 10 * ceiling) {
                        return std::nullopt; // ExceedBoardCeiling
                    }
                    board[9 - (index % 10)] |= bits_t::one << (index / 10);
                    ++index;
                    break;

                case '.':
                case '_':
                    ++index;
                    break;

                default: {
                    // noop
                }
            }
        }

        if (index % 10 != 0) {
            return std::nullopt; // MismatchedWidth
        }

        return std::make_optional(board);
    }
};
