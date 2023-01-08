use crate::{dd, Offset, Orientation, Piece, Rotate, Rotation, Shape};
use crate::internal_macros::add_member_for_from;

/// The offsets of blocks that make up a piece. Usually, you should obtain it from `PieceBlocksFactory`.
///
/// It holds shape and orientation and provides information on the positional relationship of the blocks
/// and provides information about the positional relationship of the blocks.
///
/// It prioritizes speed and records what can be calculated in advance.
/// Therefore, this struct is relatively large and slower when copied, so copy/clone are not allowed.
#[derive(Clone, Hash, Debug)]
pub struct PieceBlocks {
    pub piece: Piece,
    pub offsets: [Offset; 4],
    pub width: i32,
    pub height: i32,
    pub bottom_left: Offset,
    pub top_right: Offset,
}

impl Rotate for PieceBlocks {
    type Item = &'static PieceBlocks;

    #[inline]
    fn rotate(&self, rotation: Rotation) -> &'static PieceBlocks {
        PieceBlocksFactory.get(self.piece.rotate(rotation))
    }
}

impl PieceBlocks {
    #[inline]
    pub const fn shape(&self) -> Shape {
        self.piece.shape
    }

    #[inline]
    pub const fn orientation(&self) -> Orientation {
        self.piece.orientation
    }
}

add_member_for_from!(Piece, piece, to PieceBlocks);


/// Default piece blocks factory to generate Tetrominoes.
#[derive(Copy, Clone, Hash, Debug, Default)]
pub struct PieceBlocksFactory;

impl PieceBlocksFactory {
    const PIECE_BLOCKS: [PieceBlocks; 4 * 7] = [
        Self::create_piece(Shape::T, Orientation::North),
        Self::create_piece(Shape::T, Orientation::East),
        Self::create_piece(Shape::T, Orientation::South),
        Self::create_piece(Shape::T, Orientation::West),
        Self::create_piece(Shape::I, Orientation::North),
        Self::create_piece(Shape::I, Orientation::East),
        Self::create_piece(Shape::I, Orientation::South),
        Self::create_piece(Shape::I, Orientation::West),
        Self::create_piece(Shape::O, Orientation::North),
        Self::create_piece(Shape::O, Orientation::East),
        Self::create_piece(Shape::O, Orientation::South),
        Self::create_piece(Shape::O, Orientation::West),
        Self::create_piece(Shape::L, Orientation::North),
        Self::create_piece(Shape::L, Orientation::East),
        Self::create_piece(Shape::L, Orientation::South),
        Self::create_piece(Shape::L, Orientation::West),
        Self::create_piece(Shape::J, Orientation::North),
        Self::create_piece(Shape::J, Orientation::East),
        Self::create_piece(Shape::J, Orientation::South),
        Self::create_piece(Shape::J, Orientation::West),
        Self::create_piece(Shape::S, Orientation::North),
        Self::create_piece(Shape::S, Orientation::East),
        Self::create_piece(Shape::S, Orientation::South),
        Self::create_piece(Shape::S, Orientation::West),
        Self::create_piece(Shape::Z, Orientation::North),
        Self::create_piece(Shape::Z, Orientation::East),
        Self::create_piece(Shape::Z, Orientation::South),
        Self::create_piece(Shape::Z, Orientation::West),
    ];

    const fn create_piece(shape: Shape, orientation: Orientation) -> PieceBlocks {
        let offsets = Self::offsets(shape, orientation);
        let (min_dx, max_dx) = Self::min_max_dx(&offsets);
        let (min_dy, max_dy) = Self::min_max_dy(&offsets);
        PieceBlocks {
            offsets,
            piece: Piece { shape, orientation },
            width: max_dx - min_dx + 1,
            height: max_dy - min_dy + 1,
            bottom_left: dd(min_dx, min_dy),
            top_right: dd(max_dx, max_dy),
        }
    }

    const fn offsets(shape: Shape, orientation: Orientation) -> [Offset; 4] {
        let mut os = Self::north_offsets(shape);
        let mut index = 0;
        while index < 4 {
            let src = os[index];
            os[index] = match orientation {
                Orientation::North => src,
                Orientation::East => dd(src.dy, -src.dx),
                Orientation::South => dd(-src.dx, -src.dy),
                Orientation::West => dd(-src.dy, src.dx),
            };
            index += 1;
        }
        os
    }

    const fn north_offsets(shape: Shape) -> [Offset; 4] {
        match shape {
            Shape::T => [dd(-1, 0), dd(0, 0), dd(1, 0), dd(0, 1)],
            Shape::I => [dd(-1, 0), dd(0, 0), dd(1, 0), dd(2, 0)],
            Shape::O => [dd(0, 0), dd(1, 0), dd(0, 1), dd(1, 1)],
            Shape::L => [dd(-1, 0), dd(0, 0), dd(1, 0), dd(1, 1)],
            Shape::J => [dd(-1, 1), dd(-1, 0), dd(0, 0), dd(1, 0)],
            Shape::S => [dd(-1, 0), dd(0, 0), dd(0, 1), dd(1, 1)],
            Shape::Z => [dd(-1, 1), dd(0, 1), dd(0, 0), dd(1, 0)],
        }
    }

    const fn min_max_dx(offsets: &[Offset; 4]) -> (i32, i32) {
        let mut index = 0;
        let mut min_dx = i32::MAX;
        let mut max_dx = i32::MIN;
        while index < 4 {
            if offsets[index].dx < min_dx {
                min_dx = offsets[index].dx;
            }
            if max_dx < offsets[index].dx {
                max_dx = offsets[index].dx;
            }
            index += 1;
        }
        (min_dx, max_dx)
    }

    const fn min_max_dy(offsets: &[Offset; 4]) -> (i32, i32) {
        let mut index = 0;
        let mut min_dy = i32::MAX;
        let mut max_dy = i32::MIN;
        while index < 4 {
            if offsets[index].dy < min_dy {
                min_dy = offsets[index].dy;
            }
            if max_dy < offsets[index].dy {
                max_dy = offsets[index].dy;
            }
            index += 1;
        }
        (min_dy, max_dy)
    }

    #[inline]
    pub fn get(&self, piece: Piece) -> &'static PieceBlocks {
        &Self::PIECE_BLOCKS[piece.shape as usize * 4 + piece.orientation as usize]
    }
}


#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use rstest::*;
    use rstest_reuse::*;

    use crate::*;

    #[fixture]
    pub fn board8() -> Board8 { Board8::blank() }

    #[fixture]
    pub fn board16() -> Board16 { Board16::blank() }

    #[fixture]
    pub fn board32() -> Board32 { Board32::blank() }

    #[fixture]
    pub fn board64() -> Board64 { Board64::blank() }

    #[test]
    fn piece_blocks_works() {
        assert_eq!(size_of::<PieceBlocks>(), 60);

        let blocks = blocks!(TW);
        assert_eq!(blocks.piece(), piece!(TW));

        let blocks = blocks.piece().to_piece_blocks();
        assert_eq!(blocks.shape(), Shape::T);
        assert_eq!(blocks.orientation(), Orientation::West);
        assert_eq!(blocks.width, 2);
        assert_eq!(blocks.height, 3);

        assert_eq!(blocks.piece(), piece!(TW));
        assert_eq!(blocks.cw().piece(), piece!(TN));
        assert_eq!(blocks.ccw().piece(), piece!(TS));
        assert_eq!(blocks.r180().piece(), piece!(TE));
    }

    #[template]
    #[rstest]
    #[case::board8(board8())]
    #[case::board16(board16())]
    #[case::board32(board32())]
    #[case::board64(board64())]
    fn all_boards(#[case] board: Board<T>) {}

    #[apply(all_boards)]
    fn cc_blocks_board_op(mut board: impl BoardOp) {
        assert!(board.is_free_at(xy(5, 5)));

        board.set_all(blocks!(TN), cc(5, 5));
        assert_eq!(board.count_blocks(), 4);

        assert!(board.is_occupied_at(xy(5, 5)));

        board.unset_all(blocks!(TN), cc(5, 5));
        assert_eq!(board.count_blocks(), 0);

        assert!(board.is_free_at(xy(5, 5)));
    }
}
