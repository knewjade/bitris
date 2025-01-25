// #pragma once
//
// #include <cassert>
//
// #include "bits.hpp"
//
// namespace rows {
//     template<typename Data>
//     [[gnu::always_inline]]
//     constexpr std::pair<int, Data> spawn_bits(
//         const Data used_rows,
//         const size_t spawn_cy
//     ) {
//         constexpr auto len = bits<Data>::bit_size;
//
//         // spawn_cyとその下に1が立っているbit列
//         const Data spawn_bits = (len - 1) <= spawn_cy ? bits<Data>::full : (bits<Data>::full >> (len - 1 - spawn_cy));
//         const auto masked_used_rows = used_rows & spawn_bits;
//
//         // spawn_cy以下でブロックが存在する最も高いy。ブロックが存在しない場合は-1
//         const auto top_y = bits<Data>::most_significant_index(masked_used_rows);
//
//         // masked_used_rowsで最も上にある1とその下に1が立っているbit列。ここでは到達しないと判断する範囲
//         const Data obstacle_mask = (bits<Data>::one << (top_y + 1)) - 1;
//         assert(spawn_mask & obstacle_mask == obstacle_mask);
//         const auto reachable_rows = spawn_bits - obstacle_mask;
//
//         return std::pair(top_y, reachable_rows);
//     }
// }
