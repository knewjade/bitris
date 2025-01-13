#pragma once

enum class Rotation {
    Cw = 0,
    Ccw = 1,
};

consteval Orientation rotate_to(const Orientation from, const Rotation rotation) {
    const auto index = static_cast<size_t>(from);
    switch (rotation) {
        case Rotation::Cw:
            return static_cast<Orientation>((index + 1) % 4);
        case Rotation::Ccw:
            return static_cast<Orientation>((index + 3) % 4);
    }
    std::unreachable();
}
