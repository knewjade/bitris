#pragma once

#include "bits.hpp"

namespace rows {
    template<typename Data>
    [[gnu::always_inline]]
    constexpr int top_y(
        const std::array<Data, 10> &board,
        const size_t spawn_cy
    ) {
        const auto used_rows = bits<Data>::used_rows(board);
        constexpr auto len = bits<Data>::bit_size;

        // spawn_cyとその下に1が立っているbit列
        const Data spawn_bits = (len - 1) <= spawn_cy ? bits<Data>::full : (bits<Data>::full >> (len - 1 - spawn_cy));
        const auto masked_used_rows = used_rows & spawn_bits;

        // spawn_cy以下でブロックが存在する最も高いy。ブロックが存在しない場合は-1
        return  bits<Data>::most_significant_index(masked_used_rows);
    }
}
