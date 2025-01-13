#pragma once

#include <experimental/simd>

namespace stdx = std::experimental;

template<typename T>
struct data {
    using type = stdx::simd<T, stdx::simd_abi::fixed_size<10> >;

    template<typename U>
    static constexpr typename data<U>::type load(const std::array<T, 10> &board_bytes) {
        if constexpr (std::is_same_v<T, U>) {
            return typename data<U>::type{board_bytes.data(), stdx::vector_aligned};
        } else {
            return typename data<U>::type([=][[gnu::always_inline]](auto i) {
                if constexpr (constexpr size_t index = i; index < 0) {
                    return 0;
                } else {
                    return board_bytes[index];
                }
            });
        }
    }

    static constexpr type make_zero() {
        return make_square<0>();
    }

    template<T Value>
    static constexpr type make_square() {
        return type{Value};
    }

    static constexpr type make_square(const T value) {
        return type{value};
    }

    template<typename U>
    static constexpr typename data<U>::type to(const type &board) {
        return typename data<U>::type([=][[gnu::always_inline]](auto i) {
            if constexpr (constexpr size_t index = i; index < 0) {
                return 0;
            } else {
                return static_cast<U>(board[index]);
            }
        });
    }

    static constexpr type make_spawn(
        const T reachable_rows,
        const type &free_space,
        const uint8_t spawn_cx,
        const uint8_t spawn_cy
    ) {
        if (0 < reachable_rows) {
            return make_square(reachable_rows) & free_space;
        }
        if (bits<T>::bit_size <= spawn_cy) {
            return make_square<bits<T>::one << (bits<T>::bit_size - 1)>() & free_space;
        }

        alignas(32) std::array<T, 10> b{};
        b[spawn_cx] = 1 << spawn_cy;
        return type{b.data(), stdx::vector_aligned};
    }

    template<size_t Down, bool CeilOpen = false>
    static constexpr type shift_down(const type &data) {
        if constexpr (Down == 0) {
            return data;
        }
        if constexpr (bits<T>::bit_size <= Down) {
            make_square<CeilOpen ? bits<T>::full : 0>();
        }
        return CeilOpen ? ~(~data >> Down) : data >> Down;
    }

    template<size_t Up>
    static constexpr type shift_up(const type &data) {
        if constexpr (Up == 0) {
            return data;
        }
        if constexpr (bits<T>::bit_size <= Up) {
            return make_square<0>();
        }
        return data << Up;
    }

    template<size_t Right>
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

    static constexpr void show(const type &data, const int height = bits<T>::bit_size) {
        std::string str;
        for (int y = height - 1; y >= 0; --y) {
            for (int x = 0; x < 10; ++x) {
                auto is_occupied_at = data[x] & (1 << y);
                str += is_occupied_at ? '#' : '.';
            }
            str += '\n';
        }

        std::cout << str << std::endl;
    }
};
