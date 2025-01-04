/// An alias for `Piece::new(shape, orientation)`.
#[macro_export]
macro_rules! piece {
    (TN) => {
        Piece::new(Shape::T, Orientation::North)
    };
    (TE) => {
        Piece::new(Shape::T, Orientation::East)
    };
    (TS) => {
        Piece::new(Shape::T, Orientation::South)
    };
    (TW) => {
        Piece::new(Shape::T, Orientation::West)
    };

    (IN) => {
        Piece::new(Shape::I, Orientation::North)
    };
    (IE) => {
        Piece::new(Shape::I, Orientation::East)
    };
    (IS) => {
        Piece::new(Shape::I, Orientation::South)
    };
    (IW) => {
        Piece::new(Shape::I, Orientation::West)
    };

    (ON) => {
        Piece::new(Shape::O, Orientation::North)
    };
    (OE) => {
        Piece::new(Shape::O, Orientation::East)
    };
    (OS) => {
        Piece::new(Shape::O, Orientation::South)
    };
    (OW) => {
        Piece::new(Shape::O, Orientation::West)
    };

    (LN) => {
        Piece::new(Shape::L, Orientation::North)
    };
    (LE) => {
        Piece::new(Shape::L, Orientation::East)
    };
    (LS) => {
        Piece::new(Shape::L, Orientation::South)
    };
    (LW) => {
        Piece::new(Shape::L, Orientation::West)
    };

    (JN) => {
        Piece::new(Shape::J, Orientation::North)
    };
    (JE) => {
        Piece::new(Shape::J, Orientation::East)
    };
    (JS) => {
        Piece::new(Shape::J, Orientation::South)
    };
    (JW) => {
        Piece::new(Shape::J, Orientation::West)
    };

    (SN) => {
        Piece::new(Shape::S, Orientation::North)
    };
    (SE) => {
        Piece::new(Shape::S, Orientation::East)
    };
    (SS) => {
        Piece::new(Shape::S, Orientation::South)
    };
    (SW) => {
        Piece::new(Shape::S, Orientation::West)
    };

    (ZN) => {
        Piece::new(Shape::Z, Orientation::North)
    };
    (ZE) => {
        Piece::new(Shape::Z, Orientation::East)
    };
    (ZS) => {
        Piece::new(Shape::Z, Orientation::South)
    };
    (ZW) => {
        Piece::new(Shape::Z, Orientation::West)
    };
}

pub use piece;
