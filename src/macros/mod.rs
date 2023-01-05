/// An alias for `Piece { shape, orientation }`.
#[macro_export]
macro_rules! piece {
    (TN) => { Piece { shape: Shape::T, orientation: Orientation::North } };
    (TE) => { Piece { shape: Shape::T, orientation: Orientation::East } };
    (TS) => { Piece { shape: Shape::T, orientation: Orientation::South } };
    (TW) => { Piece { shape: Shape::T, orientation: Orientation::West } };

    (IN) => { Piece { shape: Shape::I, orientation: Orientation::North } };
    (IE) => { Piece { shape: Shape::I, orientation: Orientation::East } };
    (IS) => { Piece { shape: Shape::I, orientation: Orientation::South } };
    (IW) => { Piece { shape: Shape::I, orientation: Orientation::West } };

    (ON) => { Piece { shape: Shape::O, orientation: Orientation::North } };
    (OE) => { Piece { shape: Shape::O, orientation: Orientation::East } };
    (OS) => { Piece { shape: Shape::O, orientation: Orientation::South } };
    (OW) => { Piece { shape: Shape::O, orientation: Orientation::West } };

    (LN) => { Piece { shape: Shape::L, orientation: Orientation::North } };
    (LE) => { Piece { shape: Shape::L, orientation: Orientation::East } };
    (LS) => { Piece { shape: Shape::L, orientation: Orientation::South } };
    (LW) => { Piece { shape: Shape::L, orientation: Orientation::West } };

    (JN) => { Piece { shape: Shape::J, orientation: Orientation::North } };
    (JE) => { Piece { shape: Shape::J, orientation: Orientation::East } };
    (JS) => { Piece { shape: Shape::J, orientation: Orientation::South } };
    (JW) => { Piece { shape: Shape::J, orientation: Orientation::West } };

    (SN) => { Piece { shape: Shape::S, orientation: Orientation::North } };
    (SE) => { Piece { shape: Shape::S, orientation: Orientation::East } };
    (SS) => { Piece { shape: Shape::S, orientation: Orientation::South } };
    (SW) => { Piece { shape: Shape::S, orientation: Orientation::West } };

    (ZN) => { Piece { shape: Shape::Z, orientation: Orientation::North } };
    (ZE) => { Piece { shape: Shape::Z, orientation: Orientation::East } };
    (ZS) => { Piece { shape: Shape::Z, orientation: Orientation::South } };
    (ZW) => { Piece { shape: Shape::Z, orientation: Orientation::West } };
}

pub use piece;

/// An alias for `BlocksFactory.get(shape, orientation)`.
#[macro_export]
macro_rules! blocks {
    (TN) => { (PieceBlocksFactory).get(Shape::T, Orientation::North) };
    (TE) => { (PieceBlocksFactory).get(Shape::T, Orientation::East) };
    (TS) => { (PieceBlocksFactory).get(Shape::T, Orientation::South) };
    (TW) => { (PieceBlocksFactory).get(Shape::T, Orientation::West) };

    (IN) => { (PieceBlocksFactory).get(Shape::I, Orientation::North) };
    (IE) => { (PieceBlocksFactory).get(Shape::I, Orientation::East) };
    (IS) => { (PieceBlocksFactory).get(Shape::I, Orientation::South) };
    (IW) => { (PieceBlocksFactory).get(Shape::I, Orientation::West) };

    (ON) => { (PieceBlocksFactory).get(Shape::O, Orientation::North) };
    (OE) => { (PieceBlocksFactory).get(Shape::O, Orientation::East) };
    (OS) => { (PieceBlocksFactory).get(Shape::O, Orientation::South) };
    (OW) => { (PieceBlocksFactory).get(Shape::O, Orientation::West) };

    (LN) => { (PieceBlocksFactory).get(Shape::L, Orientation::North) };
    (LE) => { (PieceBlocksFactory).get(Shape::L, Orientation::East) };
    (LS) => { (PieceBlocksFactory).get(Shape::L, Orientation::South) };
    (LW) => { (PieceBlocksFactory).get(Shape::L, Orientation::West) };

    (JN) => { (PieceBlocksFactory).get(Shape::J, Orientation::North) };
    (JE) => { (PieceBlocksFactory).get(Shape::J, Orientation::East) };
    (JS) => { (PieceBlocksFactory).get(Shape::J, Orientation::South) };
    (JW) => { (PieceBlocksFactory).get(Shape::J, Orientation::West) };

    (SN) => { (PieceBlocksFactory).get(Shape::S, Orientation::North) };
    (SE) => { (PieceBlocksFactory).get(Shape::S, Orientation::East) };
    (SS) => { (PieceBlocksFactory).get(Shape::S, Orientation::South) };
    (SW) => { (PieceBlocksFactory).get(Shape::S, Orientation::West) };

    (ZN) => { (PieceBlocksFactory).get(Shape::Z, Orientation::North) };
    (ZE) => { (PieceBlocksFactory).get(Shape::Z, Orientation::East) };
    (ZS) => { (PieceBlocksFactory).get(Shape::Z, Orientation::South) };
    (ZW) => { (PieceBlocksFactory).get(Shape::Z, Orientation::West) };
}

pub use blocks;
