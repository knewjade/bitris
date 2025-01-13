#pragma once

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

struct Piece {
    Shape shape;
    Orientation orientation;
};
