#pragma once

#include <cstdint>

#include "pieces.hpp"
#include "rotate.hpp"
#include "templates.hpp"

struct Offset {
    int32_t x, y;
};

[[gnu::always_inline]]
constexpr Offset operator-(const Offset &offset) {
    return {-offset.x, -offset.y};
}

template<Shape Shape>
struct offsets {
    static_assert(!(Shape == Shape::I || Shape == Shape::O));

    static constexpr size_t N = 5;

    template<Orientation Orientation>
    static consteval std::array<Offset, N> get() {
        switch (Orientation) {
            case Orientation::North:
                return {Offset{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}};
            case Orientation::East:
                return {Offset{0, 0}, {1, 0}, {1, -1}, {0, 2}, {1, 2}};
            case Orientation::South:
                return {Offset{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}};
            case Orientation::West:
                return {Offset{0, 0}, {-1, 0}, {-1, -1}, {0, 2}, {-1, 2}};
        }
        std::unreachable();
    }
};

template<>
struct offsets<Shape::O> {
    static constexpr size_t N = 1;

    template<Orientation Orientation>
    static consteval std::array<Offset, N> get() {
        switch (Orientation) {
            case Orientation::North:
                return {Offset{0, 0}};
            case Orientation::East:
                return {Offset{0, -1}};
            case Orientation::South:
                return {Offset{-1, -1}};
            case Orientation::West:
                return {Offset{-1, 0}};
        }
        std::unreachable();
    }
};

template<>
struct offsets<Shape::I> {
    static constexpr size_t N = 5;

    template<Orientation Orientation>
    static consteval std::array<Offset, N> get() {
        switch (Orientation) {
            case Orientation::North:
                return {Offset{0, 0}, {-1, 0}, {2, 0}, {-1, 0}, {2, 0}};
            case Orientation::East:
                return {Offset{-1, 0}, {0, 0}, {0, 0}, {0, 1}, {0, -2}};
            case Orientation::South:
                return {Offset{-1, 1}, {1, 1}, {-2, 1}, {1, 0}, {-2, 0}};
            case Orientation::West:
                return {Offset{0, 1}, {0, 1}, {0, 1}, {0, -1}, {0, 2}};
        }
        std::unreachable();
    }
};

template<Piece FromPiece, Rotation Rotation>
consteval std::array<Offset, offsets<FromPiece.shape>::N> get_offsets() {
    constexpr auto from_orientation = FromPiece.orientation;
    constexpr auto from_offsets = offsets<FromPiece.shape>::template get<from_orientation>();

    constexpr auto to_orientation = rotate_to(from_orientation, Rotation);
    constexpr auto to_offsets = offsets<FromPiece.shape>::template get<to_orientation>();

    return static_zip2<Offset>([](auto from, auto to) {
        return Offset{from.x - to.x, from.y - to.y};
    }, from_offsets, to_offsets);
}
