#include <iostream>
#include <vector>
#include <chrono>
// #include <simdpp/simd.h>

#include "search.hpp"
#include "data.hpp"
#include "pieces.hpp"
#include "templates.hpp"
// #include "search2.hpp"
// #include "board.hpp"

// #ifdef _MSC_VER
// #include <intrin.h>
// #else
// #include <x86intrin.h>
// #endif

// from https://github.com/facebook/folly/blob/7a3f5e4e81bc83a07036e2d1d99d6a5bf5932a48/folly/lang/Hint-inl.h#L107
// Apache License 2.0
template<class Tp>
void DoNotOptimize(Tp &value) {
    asm ("" : : "m"(value) : "memory");
}

template<class Tp>
void DoNotOptimize(const Tp &value) {
    asm volatile("" : : "m"(value) : "memory");
}

template<class T>
std::array<T, 10> load_board(const std::string &board_str) {
    if (board_str.size() % 10 != 0) {
        throw std::runtime_error("board_str.size() % 10 != 0");
    }

    const auto height = board_str.size() / 10;

    std::array<T, 10> board{};
    for (size_t i = 0; i < board_str.size(); ++i) {
        const int x = i % 10;
        const int y = height - (i / 10) - 1;

        if (board_str[i] == '#' || board_str[i] == 'X') {
            board[x] |= 1U << y;
        }
    }

    return board;
}

template<class T>
std::string to_string(const std::array<T, 10> &board, const int height) {
    std::string str;
    for (int y = height - 1; y >= 0; --y) {
        for (int x = 0; x < 10; ++x) {
            str += board[x] & 1U << y ? '#' : '.';
        }
        str += '\n';
    }
    return str;
}

template<class T>
std::string to_string(const std::array<T, 40> &board, const int height) {
    std::string str;
    for (int r = 0; r < 4; ++r) {
        const auto bias = r * 10;
        for (int y = height - 1; y >= 0; --y) {
            for (int x = 0; x < 10; ++x) {
                str += board[x + bias] & 1U << y ? '#' : '.';
            }
            str += '\n';
        }
        str += '\n';
    }
    return str;
}

// template<typename T, size_t N>
// size_t popcount(const std::array<T, N> &board) {
//     size_t count = 0;
//     for (int x = 0; x < N; ++x) {
//         count += std::bitset<bits<T>::bit_size>(board[x]).count();
//     }
//     return count;
// }

template<int count = 50000000>
auto bench(auto f, auto &&... args) {
    int i = 0;
    auto start = std::chrono::steady_clock::now();
    for (int _ = 0; _ < count; ++_) {
        (void) (DoNotOptimize(args), ...);
        DoNotOptimize(f(args...));
        i += 1;
    }
    auto end = std::chrono::steady_clock::now();
    if (i != count) {
        throw std::runtime_error("bench: i != count");
    }
    return std::chrono::duration<double, std::nano>(end - start).count() / count;
}

template<typename Data>
std::array<Data, 10> lemontea_tspin_board() {
    return load_board<Data>((
        "X         "
        "X         "
        "XX      XX"
        "XXX    XXX"
        "XXXX   XXX"
        "XXXX  XXXX"
        "XXX   XXXX"
        "XXXX XXXXX"
    ));
}

void test() {
    // auto shape_names = "TIOLJSZ"; {
    //     using Data = uint32_t;
    //     const auto lemontea_tspin = lemontea_tspin_board<Data>();
    //     std::cout << to_string(lemontea_tspin, 10) << std::endl; {
    //         constexpr auto shape = Shape::T;
    //         constexpr auto expected = 37;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     } {
    //         constexpr auto shape = Shape::I;
    //         constexpr auto expected = 34;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     } {
    //         constexpr auto shape = Shape::J;
    //         constexpr auto expected = 35;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     } {
    //         constexpr auto shape = Shape::S;
    //         constexpr auto expected = 35;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     } {
    //         constexpr auto shape = Shape::Z;
    //         constexpr auto expected = 34;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     } {
    //         constexpr auto shape = Shape::O;
    //         constexpr auto expected = 9;
    //         const auto result = s::searcher<Data, shape>::search(lemontea_tspin, Orientation::North, 4, 20);
    //         if (popcount<Data>(result) == expected) {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": OK" << std::endl;
    //         } else {
    //             std::cout << shape_names[static_cast<size_t>(shape)] << ": NG" << std::endl;
    //             std::cout << to_string(result, 10) << std::endl;
    //             std::cout << popcount<Data>(result) << std::endl;
    //         }
    //     }
    // }

    /**    expected_all_moves: vec![
                (T, 37),
                (I, 34),
                (L, 35),
                (J, 35),
                (S, 35),
                (Z, 34),
                (O, 36),
            ],
            expected_minimized_moves: vec![
                (T, 37),
                (I, 17),
                (L, 35),
                (J, 35),
                (S, 18),
                (Z, 17),
                (O, 9),
            ],*/
}

int main() {
    // test();

    std::cout << std::endl;

    auto shape_names = "TIOLJSZ";
    // {
    //     std::cout << "# EMPTY" << std::endl;
    //     using T = uint32_t;
    //     alignas(32) const auto board_bytes = std::array<T, 10>{};
    //
    //     // static_for<{Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z}>([&]<Shape Shape>() {
    //     static_for<{Shape::O}>([&]<Shape Shape>() {
    //         const auto t = bench(s::searcher<T, Shape>::search, board_bytes, Orientation::North, 4, 20);
    //         std::cout << "Elapsed time (" << shape_names[static_cast<int>(Shape)] << "): " << t << " ns" << std::endl;
    //     });
    // }
    // {
    //     std::cout << "# LEMONTEA" << std::endl;
    //     using T = uint16_t;
    //     alignas(32) const auto board_bytes = lemontea_tspin_board<T>();
    //
    //     // static_for<{Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z}>([&]<Shape Shape>() {
    //     static_for<{Shape::O}>([&]<Shape Shape>() {
    //         const auto t = bench(s::searcher<T, Shape>::search, board_bytes, Orientation::North, 4, 20);
    //         std::cout << "Elapsed time (" << shape_names[static_cast<int>(Shape)] << "): " << t << " ns" << std::endl;
    //     });
    // }

    return 0;
}
