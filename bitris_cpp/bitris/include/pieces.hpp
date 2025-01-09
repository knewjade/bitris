#pragma once

#include <array>
#include <cstdint>
#include <templates.hpp>
#include <utility>

enum class Shape {
    T = 0,
    I = 1,
    O = 2,
    L = 3,
    J = 4,
    S = 5,
    Z = 6,
};

enum class Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
};

enum class Rotation {
    Cw = 0,
    Ccw = 1,
};

struct Location {
    int32_t x, y;
};

struct Offset {
    int32_t x, y;
};

struct Piece {
    Shape shape;
    Orientation orientation;
};

struct PieceBlocks {
    Piece piece;
    std::array<Location, 4> locations;
};

struct CcPlacement {
    int8_t cx;
    int8_t cy;
};

namespace const_ {
    consteval std::array<Location, 4> rotate_blocks_to_right(const std::array<Location, 4> &locations) {
        return std::array{
            Location{locations[0].y, -locations[0].x},
            Location{locations[1].y, -locations[1].x},
            Location{locations[2].y, -locations[2].x},
            Location{locations[3].y, -locations[3].x},
        };
    }

    consteval std::array<Location, 4> rotate_blocks_to_left(const std::array<Location, 4> &locations) {
        return std::array{
            Location{-locations[0].y, locations[0].x},
            Location{-locations[1].y, locations[1].x},
            Location{-locations[2].y, locations[2].x},
            Location{-locations[3].y, locations[3].x},
        };
    }

    consteval std::array<Location, 4> rotate_blocks_to_reverse(const std::array<Location, 4> &locations) {
        return std::array{
            Location{-locations[0].x, -locations[0].y},
            Location{-locations[1].x, -locations[1].y},
            Location{-locations[2].x, -locations[2].y},
            Location{-locations[3].x, -locations[3].y},
        };
    }

    consteval std::array<Location, 4> get_north_locations(const Shape shape) {
        switch (shape) {
            case Shape::T:
                return {Location{0, 0}, {-1, 0}, {1, 0}, {0, 1}};
            case Shape::I:
                return {Location{0, 0}, {-1, 0}, {1, 0}, {2, 0}};
            case Shape::O:
                return {Location{0, 0}, {1, 0}, {0, 1}, {1, 1}};
            case Shape::L:
                return {Location{0, 0}, {-1, 0}, {1, 0}, {1, 1}};
            case Shape::J:
                return {Location{0, 0}, {-1, 0}, {1, 0}, {-1, 1}};
            case Shape::S:
                return {Location{0, 0}, {-1, 0}, {0, 1}, {1, 1}};
            case Shape::Z:
                return {Location{0, 0}, {1, 0}, {0, 1}, {-1, 1}};
        }
        // ReSharper disable once CppDFAUnreachableCode
        std::unreachable();
    }

    consteval std::array<Location, 4> get_locations(const Piece piece) {
        const auto north_locations = get_north_locations(piece.shape);
        switch (piece.orientation) {
            case Orientation::North:
                return north_locations;
            case Orientation::East:
                return rotate_blocks_to_right(north_locations);
            case Orientation::South:
                return rotate_blocks_to_reverse(north_locations);
            case Orientation::West:
                return rotate_blocks_to_left(north_locations);
            default:
                std::unreachable();
        }
    }

    consteval PieceBlocks get_piece_blocks(const Piece piece) {
        const auto locations = get_locations(piece);
        return {piece, locations};
    }

    inline constinit auto pieces = std::array{
        get_piece_blocks(Piece{Shape::T, Orientation::North}),
        get_piece_blocks(Piece{Shape::T, Orientation::East}),
        get_piece_blocks(Piece{Shape::T, Orientation::South}),
        get_piece_blocks(Piece{Shape::T, Orientation::West}),

        get_piece_blocks(Piece{Shape::I, Orientation::North}),
        get_piece_blocks(Piece{Shape::I, Orientation::East}),
        get_piece_blocks(Piece{Shape::I, Orientation::South}),
        get_piece_blocks(Piece{Shape::I, Orientation::West}),

        get_piece_blocks(Piece{Shape::O, Orientation::North}),
        get_piece_blocks(Piece{Shape::O, Orientation::East}),
        get_piece_blocks(Piece{Shape::O, Orientation::South}),
        get_piece_blocks(Piece{Shape::O, Orientation::West}),

        get_piece_blocks(Piece{Shape::L, Orientation::North}),
        get_piece_blocks(Piece{Shape::L, Orientation::East}),
        get_piece_blocks(Piece{Shape::L, Orientation::South}),
        get_piece_blocks(Piece{Shape::L, Orientation::West}),

        get_piece_blocks(Piece{Shape::J, Orientation::North}),
        get_piece_blocks(Piece{Shape::J, Orientation::East}),
        get_piece_blocks(Piece{Shape::J, Orientation::South}),
        get_piece_blocks(Piece{Shape::J, Orientation::West}),

        get_piece_blocks(Piece{Shape::S, Orientation::North}),
        get_piece_blocks(Piece{Shape::S, Orientation::East}),
        get_piece_blocks(Piece{Shape::S, Orientation::South}),
        get_piece_blocks(Piece{Shape::S, Orientation::West}),

        get_piece_blocks(Piece{Shape::Z, Orientation::North}),
        get_piece_blocks(Piece{Shape::Z, Orientation::East}),
        get_piece_blocks(Piece{Shape::Z, Orientation::South}),
        get_piece_blocks(Piece{Shape::Z, Orientation::West}),
    };

    struct Constants {
        size_t offset_length;
    };

    consteval Constants constants(const Shape shape) {
        if (shape == Shape::O) {
            return {1};
        }
        return {5};
    }

    template<Shape shape, size_t N = constants(shape).offset_length>
    consteval std::array<std::array<Offset, N>, 4> get_others_offsets() {
        if constexpr (shape == Shape::O) {
            return std::array{
                std::array{Offset{0, 0}},
                std::array{Offset{0, -1}},
                std::array{Offset{-1, -1}},
                std::array{Offset{-1, 0}},
            };
        }
        switch (shape) {
            case Shape::I:
                return std::array{
                    std::array<Offset, 5>{Offset{0, 0}, {-1, 0}, {2, 0}, {-1, 0}, {2, 0}},
                    std::array<Offset, 5>{Offset{-1, 0}, {0, 0}, {0, 0}, {0, 1}, {0, -2}},
                    std::array<Offset, 5>{Offset{-1, 1}, {1, 1}, {-2, 1}, {1, 0}, {-2, 0}},
                    std::array<Offset, 5>{Offset{0, 1}, {0, 1}, {0, 1}, {0, -1}, {0, 2}},
                };
            default:
                return std::array{
                    std::array<Offset, 5>{Offset{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}},
                    std::array<Offset, 5>{Offset{0, 0}, {1, 0}, {1, -1}, {0, 2}, {1, 2}},
                    std::array<Offset, 5>{Offset{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}},
                    std::array<Offset, 5>{Offset{0, 0}, {-1, 0}, {-1, -1}, {0, 2}, {-1, 2}},
                };
        }
        std::unreachable();
    }

    template<Shape shape, size_t N = constants(shape).offset_length>
    consteval std::array<Offset, N> get_offsets(const Orientation from_orientation, const Rotation rotation) {
        const auto offsets = get_others_offsets<shape>();

        const auto from_index = static_cast<size_t>(from_orientation);
        const auto to_index = [&] {
            switch (rotation) {
                case Rotation::Cw:
                    return (from_index + 1) % N;
                case Rotation::Ccw:
                    return (from_index + N - 1) % N;
            }
            std::unreachable();
        }();

        return static_zip2<Offset>([](auto from, auto to) {
            return Offset{from.x - to.x, from.y - to.y};
        }, offsets[from_index], offsets[to_index]);
    }
}
