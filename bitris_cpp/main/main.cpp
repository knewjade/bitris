#include <iostream>
#include <vector>
#include <chrono>
// #include <simdpp/simd.h>

#include "search.hpp"
#include "kicks.hpp"
// #include "search2.hpp"
// #include "board.hpp"

// #ifdef _MSC_VER
// #include <intrin.h>
// #else
// #include <x86intrin.h>
// #endif

// from https://github.com/facebook/folly/blob/7a3f5e4e81bc83a07036e2d1d99d6a5bf5932a48/folly/lang/Hint-inl.h#L107
// Apache License 2.0
template <class Tp>
inline void DoNotOptimize(Tp& value) {
    asm  ("" : : "m"(value) : "memory");
}

template <class Tp>
inline void DoNotOptimize(const Tp &value) {
    asm volatile("" : : "m"(value) : "memory");
}

template <int count = 1000000>
auto bench(auto f, auto &&...args) {
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

// using namespace simdpp;

int main() {
    // Board64 board = Board64::blank();
    // board.set_at(2, 1);
    // std::cout << board.to_string() << std::endl;
    //
    // constexpr auto offsets = const_::get_offsets<Shape::T>(Orientation::North, Rotation::Cw);
    // std::cout << offsets.size() << std::endl;
    //
    // const int at = s::set_at_constexpr(1);
    // std::cout << at << std::endl;
    //
    // std::vector<CcPlacement> placements{};
    // s::search_v1([&placements](auto placement) {
    //     std::cout << static_cast<int>(placement.cx) << " " << static_cast<int>(placement.cy) << std::endl;
    //     placements.push_back(placement);
    // });
    //
    // std::cout << "---" << std::endl;

    using T = uint16_t;

    alignas(32) constexpr auto board_bytes = std::array<T, 10>{};
    // auto board = s::data_t<T>{board_bytes.data(), stdx::vector_aligned};

    // auto g = s::search2(board_bytes, board, 0, 0, 4, 20);
    //
    // size_t count = 0;
    // std::array<uint64_t, 10> g2{};
    // for (int i = 0; i < 10; ++i) {
    //     g2[i] = g[i];
    // }
    // const auto board2 = Board64{g2};
    // std::cout << board2.to_string() << std::endl;
    //
    const auto o = bench(s::searcher<uint16_t, Shape::O>::search, board_bytes, 0, 4, 20);
    std::cout << "Elapsed time (O): " << o << " ns" << std::endl;

    const auto t = bench(s::searcher<uint16_t, Shape::T>::search, board_bytes, 0, 4, 20);
    std::cout << "Elapsed time (T): " << t << " ns" << std::endl;

    return 0;
}
