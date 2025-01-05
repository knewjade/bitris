#include "hello.hpp"

#include "board.hpp"
#include "templates.hpp"

constexpr int set_at_constexpr(const int dy) {
    auto board = Board64{};

    for (int index = 0; index < Board64::ceiling() / dy; ++index) {
        const auto y = dy * index;
        static_for<10>([&](auto x) {
            board.set_at(x, y);
        });
    }

    board.clear_lines();

    return static_cast<int>(board.count_blocks());
}

int set_at(const int y) {
    return set_at_constexpr(y);
}
