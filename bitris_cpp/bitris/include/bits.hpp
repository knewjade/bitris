#pragma once

#include "templates.hpp"

template<typename>
struct bits {
};

template<typename Data>
struct bits_base {
    static constexpr Data zero = 0;
    static constexpr Data one = 1;

    [[gnu::always_inline]]
    static constexpr Data used_rows(const std::array<Data, 10> &board) {
        return static_packing_fold<Data>(
            []<typename... V>(V... s) {
                return (s | ...);
            },
            board
        );
    }

    [[gnu::always_inline]]
    static constexpr int most_significant_index(const Data v) {
        [[assume(bits<Data>::bit_size <= 64)]];
        if constexpr (bits<Data>::bit_size <= 32) {
            return 0 < v ? 31 - __builtin_clz(v) : -1;
        } else {
            return 0 < v ? 63 - __builtin_clzll(v) : -1;
        }
    }
};

template<>
struct bits<uint8_t> : bits_base<uint8_t> {
    using Data = uint8_t;
    static constexpr Data full = 0xFF;
    static constexpr Data bit_size = 8;
};

template<>
struct bits<uint16_t> : bits_base<uint16_t> {
    using Data = uint16_t;
    static constexpr Data full = 0xFFFF;
    static constexpr Data bit_size = 16;
};

template<>
struct bits<uint32_t> : bits_base<uint32_t> {
    using Data = uint32_t;
    static constexpr Data full = 0xFFFFFFFF;
    static constexpr Data bit_size = 32;
};

template<>
struct bits<uint64_t> : bits_base<uint64_t> {
    using Data = uint64_t;
    static constexpr Data full = 0xFFFFFFFFFFFFFFFFULL;
    static constexpr Data bit_size = 64;
};
