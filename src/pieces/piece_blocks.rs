use std::cmp;

use tinyvec::ArrayVec;

use crate::{Rotate, Rotation};
use crate::coordinates::{bl, BlPosition, cc, CcPosition, dd, Location, Offset, tr, TrPosition};
use crate::internal_macros::add_member_for_from;
use crate::pieces::{Orientation, Piece, Shape};

/// The offsets of blocks that make up a piece. Usually, you should obtain it from `PieceBlocksFactory`.
///
/// It holds shape and orientation and provides information on the positional relationship of the blocks
/// and provides information about the positional relationship of the blocks.
///
/// It prioritizes speed and records what can be calculated in advance.
/// Therefore, this struct is relatively large and slower when copied, so it's recommended that clone be avoided.
#[derive(Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct PieceBlocks {
    pub piece: Piece,
    pub offsets: [Offset; 4],
    pub width: u32,
    pub height: u32,
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
    const fn new(piece: Piece) -> Self {
        const fn offsets(piece: Piece) -> [Offset; 4] {
            let mut os = north_offsets(piece.shape);
            let mut index = 0;
            while index < 4 {
                let src = os[index];
                os[index] = match piece.orientation {
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

        let offsets = offsets(piece);
        let (min_dx, max_dx) = min_max_dx(&offsets);
        let (min_dy, max_dy) = min_max_dy(&offsets);
        PieceBlocks {
            piece,
            offsets,
            width: (max_dx - min_dx + 1) as u32,
            height: (max_dy - min_dy + 1) as u32,
            bottom_left: dd(min_dx, min_dy),
            top_right: dd(max_dx, max_dy),
        }
    }

    #[inline]
    pub const fn shape(&self) -> Shape {
        self.piece.shape
    }

    #[inline]
    pub const fn orientation(&self) -> Orientation {
        self.piece.orientation
    }

    #[inline]
    pub fn to_locations(&self, cc: CcPosition) -> [Location; 4] {
        let cc = cc.to_location();
        self.offsets.map(|offset| cc + offset)
    }

    /// Returns block locations of possible touch with the ground.
    /// Finds the y-coordinate of the lowest block in each x-coordinate.
    /// ```
    /// use tinyvec::{array_vec, ArrayVec};
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// assert_eq!(
    ///     piece!(JS).to_piece_blocks().touching_offsets().as_slice(),
    ///     ArrayVec::from([dd(-1, 0), dd(0, 0), dd(1, -1)]).as_slice(),
    /// );
    /// assert_eq!(
    ///     piece!(SN).to_piece_blocks().touching_offsets().as_slice(),
    ///     ArrayVec::from([dd(-1, 0), dd(0, 0), dd(1, 1)]).as_slice(),
    /// );
    /// ```
    #[inline]
    pub fn touching_offsets(&self) -> ArrayVec<[Offset; 4]> {
        let lx = self.bottom_left.dx;
        let min_dys = self.offsets.iter()
            .fold([i32::MAX; 4], |mut min_dys, offset| {
                let index = (offset.dx - lx) as usize;
                min_dys[index] = cmp::min(offset.dy, min_dys[index]);
                min_dys
            });

        let mut vec = ArrayVec::<[Offset; 4]>::new();
        for index in 0..self.width as usize {
            let dx = lx + index as i32;
            vec.push(Offset::new(dx, min_dys[index]));
        }
        vec
    }
}

add_member_for_from!(Piece, piece, to PieceBlocks);

// Converts another Position into a CcPosition according to the PieceBlocks provided
pub trait ToCcPosition<T = PieceBlocks>: Sized {
    fn to_cc_position(&self, piece_blocks: &T) -> CcPosition;
}

impl ToCcPosition<PieceBlocks> for BlPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(bl(1, 2).to_cc_position(&piece_blocks), cc(2, 2));
    /// ```
    #[inline]
    fn to_cc_position(&self, piece_blocks: &PieceBlocks) -> CcPosition {
        let bottom_left = piece_blocks.bottom_left;
        cc(self.lx - bottom_left.dx, self.by - bottom_left.dy)
    }
}

impl ToCcPosition<PieceBlocks> for TrPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(tr(3, 3).to_cc_position(&piece_blocks), cc(2, 2));
    /// ```
    #[inline]
    fn to_cc_position(&self, piece_blocks: &PieceBlocks) -> CcPosition {
        let top_right = piece_blocks.top_right;
        cc(self.rx - top_right.dx, self.ty - top_right.dy)
    }
}

// Converts another Position into a BlPosition according to the PieceBlocks provided
pub trait ToBlPosition<T>: Sized {
    fn to_bl_position(&self, piece_blocks: &T) -> BlPosition;
}

impl ToBlPosition<PieceBlocks> for CcPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(cc(2, 2).to_bl_position(&piece_blocks), bl(1, 2));
    /// ```
    #[inline]
    fn to_bl_position(&self, piece_blocks: &PieceBlocks) -> BlPosition {
        let bottom_left = piece_blocks.bottom_left;
        bl(self.cx + bottom_left.dx, self.cy + bottom_left.dy)
    }
}

impl ToBlPosition<PieceBlocks> for TrPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(tr(3, 3).to_bl_position(&piece_blocks), bl(1, 2));
    /// ```
    #[inline]
    fn to_bl_position(&self, piece_blocks: &PieceBlocks) -> BlPosition {
        self.to_cc_position(piece_blocks).to_bl_position(piece_blocks)
    }
}

// Converts another Position into a TrPosition according to the PieceBlocks provided
pub trait ToTrPosition<T>: Sized {
    fn to_tr_position(&self, piece_blocks: &T) -> TrPosition;
}

impl ToTrPosition<PieceBlocks> for CcPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(cc(2, 2).to_tr_position(&piece_blocks), tr(3, 3));
    /// ```
    #[inline]
    fn to_tr_position(&self, piece_blocks: &PieceBlocks) -> TrPosition {
        let top_right = piece_blocks.top_right;
        tr(self.cx + top_right.dx, self.cy + top_right.dy)
    }
}

impl ToTrPosition<PieceBlocks> for BlPosition {
    /// ```
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let piece_blocks = piece!(TN).to_piece_blocks();
    /// assert_eq!(bl(1, 2).to_tr_position(&piece_blocks), tr(3, 3));
    /// ```
    #[inline]
    fn to_tr_position(&self, piece_blocks: &PieceBlocks) -> TrPosition {
        self.to_cc_position(piece_blocks).to_tr_position(piece_blocks)
    }
}

/// Default piece blocks factory to generate Tetrominoes.
#[derive(Copy, Clone, Hash, Debug, Default)]
pub struct PieceBlocksFactory;

impl PieceBlocksFactory {
    const PIECE_BLOCKS: [PieceBlocks; 4 * 7] = [
        PieceBlocks::new(Piece::new(Shape::T, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::T, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::T, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::T, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::I, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::I, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::I, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::I, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::O, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::O, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::O, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::O, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::L, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::L, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::L, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::L, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::J, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::J, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::J, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::J, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::S, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::S, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::S, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::S, Orientation::West)),
        PieceBlocks::new(Piece::new(Shape::Z, Orientation::North)),
        PieceBlocks::new(Piece::new(Shape::Z, Orientation::East)),
        PieceBlocks::new(Piece::new(Shape::Z, Orientation::South)),
        PieceBlocks::new(Piece::new(Shape::Z, Orientation::West)),
    ];

    #[inline]
    pub fn get(&self, piece: Piece) -> &'static PieceBlocks {
        &Self::PIECE_BLOCKS[piece.shape as usize * 4 + piece.orientation as usize]
    }
}

impl From<Piece> for PieceBlocks {
    fn from(piece: Piece) -> Self {
        Self::new(piece)
    }
}


#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use rstest::*;
    use rstest_reuse::*;

    use crate::piece;
    use crate::prelude::*;

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

        let blocks = piece!(TW).to_piece_blocks();
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
        let locations = [xy(4, 5), xy(5, 5), xy(6, 5), xy(5, 6)];

        assert!(board.is_free_at(xy(5, 5)));

        board.set_all(&locations);

        assert_eq!(board.count_blocks(), 4);

        assert!(board.is_occupied_at(xy(5, 5)));

        board.unset_all(&locations);

        assert_eq!(board.count_blocks(), 0);

        assert!(board.is_free_at(xy(5, 5)));
    }
}
