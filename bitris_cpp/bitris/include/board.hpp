#ifndef BOARD_HPP
#define BOARD_HPP

#include <array>
#include <bitset>
#include <string>
#include <simdpp/simd.h>

#include "templates.hpp"

using namespace simdpp;

class Location {
public:
    int32_t x, y;

    Location(const int x, const int y) : x(x), y(y) {
    }
};

class Board64 {
public:
    static constexpr int WIDTH = 10;
    static constexpr int HEIGHT = 64;

    using under_t = uint64_t;
    std::array<under_t, WIDTH> cols = std::array<under_t, WIDTH>{};

    constexpr Board64(): cols({}) {
    }

    // using under_t = uint64_t;

    // template <std::size_t N>
    // using simd_of = std::experimental::simd<under_t, std::experimental::simd_abi::deduce_t<under_t, N>>;
    //
    // using data_t = simd_of<WIDTH>;
    //
    // alignas(std::experimental::memory_alignment_v<data_t>) data_t cols;
    //
    // constexpr  Board64(): cols(0) {}
    //
    // constexpr explicit Board64(const std::array<under_t, WIDTH> &d): cols(d.data(), std::experimental::element_aligned) {}

    static constexpr Board64 blank() {
        return {};
    }

    // static constexpr Board64 filled_up_to(int height) {
    //     Board64 board{};
    //     auto mask = (1ULL << height) - 1;
    //     board.cols = data_t(mask);
    //     return board;
    // }

    static constexpr uint32_t ceiling() {
        return HEIGHT;
    }

    // [[nodiscard]] constexpr uint32_t well_top() const {
    //     const auto used_rows = used_row_key();
    //     return HEIGHT - __builtin_clzll(used_rows);
    // }

    constexpr void set_at(const size_t x, const size_t y) {
        cols[x] |= 1ULL << y;
    }

    // constexpr void set_at2(const int x, const int y) {
    //     cols[x] |= (1ULL << y);
    // }
    //
    // // void unset_at(const Location &location);

    [[nodiscard]] constexpr bool is_occupied_at(const Location &location) const {
        return cols[location.x] & (1ULL << location.y);
    }

    // // [[nodiscard]] bool is_free_at(const Location &location) const;
    // //
    // // [[nodiscard]] bool is_empty() const;

    [[nodiscard]] constexpr size_t count_blocks() const {
        return static_fold<uint64_t>([](const auto acc, auto col) {
            return acc + std::bitset<64>(col).count();
        }, 0, cols);
    }

    // [[nodiscard]] constexpr uint64_t used_row_key() const {
    //     uint64_t key = 0;
    //     for (int x = 0; x < WIDTH; ++x) {
    //         key |= cols[x];
    //     }
    //     return key;
    // }

    [[nodiscard]] constexpr uint64_t filled_row_key() const {
        return static_packing_fold(
            []<typename... T>(T... s) {
                return (s & ...);
            },
            cols,
            std::make_index_sequence<10>()
        );
    }

    constexpr void clear_lines() {
        auto key = filled_row_key();
        while (key) {
            const auto mask = (key - 1) & ~key;
            const auto inverted_mask = ~mask;
            for (int x = 0; x < WIDTH; ++x) {
                const auto bottom = cols[x] & mask;
                const auto slided_upper = (cols[x] >> 1) & inverted_mask;
                cols[x] = slided_upper | bottom;
            }
            key = (key >> 1) & inverted_mask;
        }
    }

    // void invert();
    //
    // [[nodiscard]] bool overlaps(const Board64 &other) const;
    //
    // void merge(const Board64 &other);
    //
    // void remove_all(const Board64 &other);

    [[nodiscard]] std::string to_string() const;
};

#endif // BOARD_HPP
