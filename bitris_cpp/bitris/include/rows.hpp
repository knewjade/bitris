#pragma once

#include <cassert>

#include "bits.hpp"

namespace rows {
    constexpr size_t leading_zeros_u32(const uint32_t v) {
        return 0 < v ? 31 - __builtin_clz(v) : 0;
    }

    constexpr size_t leading_zeros_u64(const uint64_t v) {
        return 0 < v ? 63 - __builtin_clzll(v) : 0;
    }

    template<typename T>
    constexpr std::pair<int, T> spawn_bits(
        const T used_rows,
        const size_t spawn_cy
    ) {
        constexpr auto len = bits<T>::bit_size;

        // spawn_cyとその下に1が立っているbit列
        const T spawn_bits = (len - 1) <= spawn_cy ? bits<T>::full : (bits<T>::full >> (len - 1 - spawn_cy));
        const auto masked_used_rows = used_rows & spawn_bits;

        // spawn_cy以下でブロックが存在する最も高いy。ブロックが存在しない場合は-1
        const auto top_y = bits<T>::most_significant_index(masked_used_rows);

        // masked_used_rowsで最も上にある1とその下に1が立っているbit列。ここでは到達しないと判断する範囲
        const T obstacle_mask = (bits<T>::one << (top_y + 1)) - 1;
        assert(spawn_mask & obstacle_mask == obstacle_mask);
        const auto reachable_rows = spawn_bits - obstacle_mask;

        return std::pair(top_y, reachable_rows);
    }
}
