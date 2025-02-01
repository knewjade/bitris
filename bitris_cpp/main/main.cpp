#include <iostream>
#include <string>
#include <cstdint>
#include <array>

#include <chrono>

#include "pieces.hpp"
#include "templates.hpp"
#include "search.hpp"

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

template<int count = 1000000>
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
std::string to_string(const std::array<T, 10> &result, const std::array<T, 10> &board, const int height) {
    std::string str;
    for (int y = height - 1; y >= 0; --y) {
        for (int x = 0; x < 10; ++x) {
            if (result[x] & 1U << y) {
                str += 'v';
            } else if (board[x] & 1U << y) {
                str += '#';
            } else {
                str += '.';
            }
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


template<class T>
std::string to_string(const std::array<T, 40> &result, const std::array<T, 10> &board, const int height) {
    std::string str;
    for (int r = 0; r < 4; ++r) {
        const auto bias = r * 10;
        for (int y = height - 1; y >= 0; --y) {
            for (int x = 0; x < 10; ++x) {
                if (result[x + bias] & 1U << y) {
                    str += 'v';
                } else if (board[x] & 1U << y) {
                    str += '#';
                } else {
                    str += '.';
                }
            }
            str += '\n';
        }
        str += '\n';
    }
    return str;
}

template<typename T, size_t N>
size_t popcount(const std::array<T, N> &board) {
    size_t count = 0;
    for (size_t i = 0; i < N; ++i) {
        auto b = board[i];
        while (0 < b) {
            if (b & 1) {
                count++;
            }
            b >>= 1;
        }
    }
    return count;
}

template<typename Data, size_t N>
void test(const std::array<Data, 10> board, const size_t height, const std::array<Data, N> &result, const Shape shape,
          size_t expected) {
    constexpr auto SHAPE_NAMES = "TIOLJSZ";
    const char name = SHAPE_NAMES[static_cast<size_t>(shape)];
    if (const auto actual = popcount<Data>(result); actual == expected) {
        std::cout << name << ": OK" << std::endl;
    } else {
        std::cout << name << ": NG (actual=" << std::to_string(actual) << std::endl;
        std::cout << to_string(result, board, height) << std::endl;
        std::cout << popcount<Data>(result) << std::endl;
    }
}

void test1() {
    using Data = uint32_t;
    constexpr auto height = 10;
    const auto board = load_board<Data>("");
    std::cout << to_string(board, height) << std::endl;

    // T
    {
        constexpr auto shape = Shape::T;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // I
    {
        constexpr auto shape = Shape::I;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // J
    {
        constexpr auto shape = Shape::J;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // S
    {
        constexpr auto shape = Shape::S;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // Z
    {
        constexpr auto shape = Shape::Z;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // O
    {
        constexpr auto shape = Shape::O;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 9);
    }
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

void test2() {
    using Data = uint32_t;
    constexpr auto height = 10;
    const auto board = lemontea_tspin_board<Data>();
    std::cout << to_string(board, height) << std::endl;

    // T
    {
        constexpr auto shape = Shape::T;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 37);
    }
    // I
    {
        constexpr auto shape = Shape::I;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // J
    {
        constexpr auto shape = Shape::J;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 35);
    }
    // S
    {
        constexpr auto shape = Shape::S;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 35);
    }
    // Z
    {
        constexpr auto shape = Shape::Z;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // O
    {
        constexpr auto shape = Shape::O;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 9);
    }
}

template<typename Data>
std::array<Data, 10> lzt() {
    return load_board<Data>((
                                    "XXXX  XXX "
                                    "XXXXX XXXX"
                                    "      X  X"
                                    "          "
                                    "   X      "
                                    "    XX X X"
                                    "XXX X     "
                                    "XX  X    X"
                                    "X    X    "
                                    "XX  XXXX X"
                                    "X    XX   "
                                    "XX XXX    "
                                    "       X  "
                                    "      XXX "
                                    "XX X      "
                                    "X    X    "
                                    "X   X    X"
                                    "X  XX X XX"
                                    "XX  XXXXXX"
                            ));
}

void test3() {
    using Data = uint32_t;
    constexpr auto height = 22;
    const auto board = lzt<Data>();
    std::cout << to_string(board, height) << std::endl;

    // T
    {
        constexpr auto shape = Shape::T;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // I
    {
        constexpr auto shape = Shape::I;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 93);
    }
    // J
    {
        constexpr auto shape = Shape::J;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // S
    {
        constexpr auto shape = Shape::S;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 34);
    }
    // Z
    {
        constexpr auto shape = Shape::Z;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 129);
    }
    // O
    {
        constexpr auto shape = Shape::O;
        const auto result = s::searcher<Data, shape>::search(board, Orientation::North, 4, 20);
        test(board, height, result, shape, 9);
    }
}

int main() {
    test1();
    test2();
    test3();

    std::cout << std::endl;

    auto shape_names = "TIOLJSZ";
    // empty
    {
        std::cout << "# EMPTY" << std::endl;
        using T = uint32_t;
        alignas(32) const auto board_bytes = std::array<T, 10>{};

        static_for_t<{Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z}>([&]<Shape Shape>() {
            const auto t = bench(s::searcher<T, Shape>::search, board_bytes, Orientation::North, 4, 20);
            std::cout << "Elapsed time (" << shape_names[static_cast<int>(Shape)] << "): " << t << " ns" << std::endl;
        });
    }
    // lemontea tspin
    {
        std::cout << "# LEMONTEA" << std::endl;
        using T = uint16_t;
        alignas(32) const auto board_bytes = lemontea_tspin_board<T>();

        static_for_t<{Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z}>([&]<Shape Shape>() {
            const auto t = bench(s::searcher<T, Shape>::search, board_bytes, Orientation::North, 4, 20);
            std::cout << "Elapsed time (" << shape_names[static_cast<int>(Shape)] << "): " << t << " ns" << std::endl;
        });
    }
    // lzt
    {
        std::cout << "# LZT" << std::endl;
        using T = uint16_t;
        alignas(32) const auto board_bytes = lzt<T>();

        static_for_t<{Shape::I, Shape::J, Shape::L, Shape::O, Shape::S, Shape::T, Shape::Z}>([&]<Shape Shape>() {
            const auto t = bench(s::searcher<T, Shape>::search, board_bytes, Orientation::North, 4, 20);
            std::cout << "Elapsed time (" << shape_names[static_cast<int>(Shape)] << "): " << t << " ns" << std::endl;
        });
    }

    return 0;
}
