/// An alias for `Piece::new(shape, orientation)`.
#[macro_export]
macro_rules! piece {
    (TN) => { Piece::new(Shape::T, Orientation::North) };
    (TE) => { Piece::new(Shape::T, Orientation::East) };
    (TS) => { Piece::new(Shape::T, Orientation::South) };
    (TW) => { Piece::new(Shape::T, Orientation::West) };

    (IN) => { Piece::new(Shape::I, Orientation::North) };
    (IE) => { Piece::new(Shape::I, Orientation::East) };
    (IS) => { Piece::new(Shape::I, Orientation::South) };
    (IW) => { Piece::new(Shape::I, Orientation::West) };

    (ON) => { Piece::new(Shape::O, Orientation::North) };
    (OE) => { Piece::new(Shape::O, Orientation::East) };
    (OS) => { Piece::new(Shape::O, Orientation::South) };
    (OW) => { Piece::new(Shape::O, Orientation::West) };

    (LN) => { Piece::new(Shape::L, Orientation::North) };
    (LE) => { Piece::new(Shape::L, Orientation::East) };
    (LS) => { Piece::new(Shape::L, Orientation::South) };
    (LW) => { Piece::new(Shape::L, Orientation::West) };

    (JN) => { Piece::new(Shape::J, Orientation::North) };
    (JE) => { Piece::new(Shape::J, Orientation::East) };
    (JS) => { Piece::new(Shape::J, Orientation::South) };
    (JW) => { Piece::new(Shape::J, Orientation::West) };

    (SN) => { Piece::new(Shape::S, Orientation::North) };
    (SE) => { Piece::new(Shape::S, Orientation::East) };
    (SS) => { Piece::new(Shape::S, Orientation::South) };
    (SW) => { Piece::new(Shape::S, Orientation::West) };

    (ZN) => { Piece::new(Shape::Z, Orientation::North) };
    (ZE) => { Piece::new(Shape::Z, Orientation::East) };
    (ZS) => { Piece::new(Shape::Z, Orientation::South) };
    (ZW) => { Piece::new(Shape::Z, Orientation::West) };
}

pub use piece;

/// An alias for `BlocksFactory.get(shape, orientation)`.
#[macro_export]
macro_rules! blocks {
    (TN) => { (PieceBlocksFactory).get(Piece::new(Shape::T, Orientation::North)) };
    (TE) => { (PieceBlocksFactory).get(Piece::new(Shape::T, Orientation::East)) };
    (TS) => { (PieceBlocksFactory).get(Piece::new(Shape::T, Orientation::South)) };
    (TW) => { (PieceBlocksFactory).get(Piece::new(Shape::T, Orientation::West)) };

    (IN) => { (PieceBlocksFactory).get(Piece::new(Shape::I, Orientation::North)) };
    (IE) => { (PieceBlocksFactory).get(Piece::new(Shape::I, Orientation::East)) };
    (IS) => { (PieceBlocksFactory).get(Piece::new(Shape::I, Orientation::South)) };
    (IW) => { (PieceBlocksFactory).get(Piece::new(Shape::I, Orientation::West)) };

    (ON) => { (PieceBlocksFactory).get(Piece::new(Shape::O, Orientation::North)) };
    (OE) => { (PieceBlocksFactory).get(Piece::new(Shape::O, Orientation::East)) };
    (OS) => { (PieceBlocksFactory).get(Piece::new(Shape::O, Orientation::South)) };
    (OW) => { (PieceBlocksFactory).get(Piece::new(Shape::O, Orientation::West)) };

    (LN) => { (PieceBlocksFactory).get(Piece::new(Shape::L, Orientation::North)) };
    (LE) => { (PieceBlocksFactory).get(Piece::new(Shape::L, Orientation::East)) };
    (LS) => { (PieceBlocksFactory).get(Piece::new(Shape::L, Orientation::South)) };
    (LW) => { (PieceBlocksFactory).get(Piece::new(Shape::L, Orientation::West)) };

    (JN) => { (PieceBlocksFactory).get(Piece::new(Shape::J, Orientation::North)) };
    (JE) => { (PieceBlocksFactory).get(Piece::new(Shape::J, Orientation::East)) };
    (JS) => { (PieceBlocksFactory).get(Piece::new(Shape::J, Orientation::South)) };
    (JW) => { (PieceBlocksFactory).get(Piece::new(Shape::J, Orientation::West)) };

    (SN) => { (PieceBlocksFactory).get(Piece::new(Shape::S, Orientation::North)) };
    (SE) => { (PieceBlocksFactory).get(Piece::new(Shape::S, Orientation::East)) };
    (SS) => { (PieceBlocksFactory).get(Piece::new(Shape::S, Orientation::South)) };
    (SW) => { (PieceBlocksFactory).get(Piece::new(Shape::S, Orientation::West)) };

    (ZN) => { (PieceBlocksFactory).get(Piece::new(Shape::Z, Orientation::North)) };
    (ZE) => { (PieceBlocksFactory).get(Piece::new(Shape::Z, Orientation::East)) };
    (ZS) => { (PieceBlocksFactory).get(Piece::new(Shape::Z, Orientation::South)) };
    (ZW) => { (PieceBlocksFactory).get(Piece::new(Shape::Z, Orientation::West)) };
}

pub use blocks;
