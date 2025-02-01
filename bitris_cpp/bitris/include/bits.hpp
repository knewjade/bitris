#pragma once

#include "templates.hpp"

template<typename Data>
struct bits {
    static constexpr Data zero = 0;
    static constexpr Data one = 1;
    static constexpr Data full = ~zero;
    static constexpr size_t bit_size = std::numeric_limits<Data>::digits;

    [[gnu::always_inline]]
    static constexpr Data used_rows(const std::array<Data, 10> &board) {
        return static_packing_fold<Data>(
                []<typename... V> [[gnu::always_inline]](V... s) {
                    return (s | ...);
                },
                board
        );
    }

    [[gnu::always_inline]]
    static constexpr int most_significant_index(const Data v) {
        [[assume(bit_size <= 64)]];
        if constexpr (bit_size <= 32) {
            return 0 < v ? 31 - __builtin_clz(v) : -1;
        } else {
            return 0 < v ? 63 - __builtin_clzll(v) : -1;
        }
    }
};
