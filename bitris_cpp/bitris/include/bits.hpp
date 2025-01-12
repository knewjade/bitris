#pragma once

#include <cassert>

#include "templates.hpp"

template<typename T>
struct bits {
};

template<typename T>
struct bits_base {
    static constexpr T one = 1;

    static constexpr T used_rows(const std::array<T, 10> &board) {
        return static_packing_fold<T>(
            []<typename... V>(V... s) {
                return (s | ...);
            },
            board
        );
    }

    static constexpr int most_significant_index(const T v) {
        [[assume(bits<T>::bit_size <= 64)]];
        if constexpr (bits<T>::bit_size <= 32) {
            return 0 < v ? 31 - __builtin_clz(v) : -1;
        } else {
            return 0 < v ? 63 - __builtin_clzll(v) : -1;
        }
    }
};

template<>
struct bits<uint8_t> : bits_base<uint8_t> {
    using T = uint8_t;
    static constexpr T full = 0xFF;
    static constexpr T bit_size = 8;
};

template<>
struct bits<uint16_t> : bits_base<uint16_t> {
    using T = uint16_t;
    static constexpr T full = 0xFFFF;
    static constexpr T bit_size = 16;
};

template<>
struct bits<uint32_t> : bits_base<uint32_t> {
    using T = uint32_t;
    static constexpr T full = 0xFFFFFFFF;
    static constexpr T bit_size = 32;
};

template<>
struct bits<uint64_t> : bits_base<uint64_t> {
    using T = uint64_t;
    static constexpr T full = 0xFFFFFFFFFFFFFFFF;
    static constexpr T bit_size = 64;
};
