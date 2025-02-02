use itertools::Itertools;
use tinyvec::ArrayVec;

use crate::boards::{Board64, BoardOp, Lines};
use crate::coordinates::{xy, Location, Offset};
use crate::pieces::Piece;
use crate::placements::{place_according_to, BlPlacement};

/// The structure represents the pieces placed on the board.
///
/// It differs from `Piece` in that cleared lines are considered.
/// It's possible to represent pieces separated vertically.
///
/// Example) The following is equivalent to `PlacedPiece::new(piece!(TE), 2, array_vec![1, 3, 4])`
///   y=5..........
///     4 ..#.......
///     3 ..##......
///     2 .......... << interception
///     1 ..#.......
///     0 ..........
///         ^ lx=2
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PlacedPiece {
    pub piece: Piece,
    pub lx: u8,
    /// `ys` is sorted in ascending.
    pub ys: ArrayVec<[u8; 4]>,
}

impl PlacedPiece {
    /// Note that the `ys` must be sorted in ascending.
    #[inline]
    pub fn new(piece: Piece, lx: u8, ys: ArrayVec<[u8; 4]>) -> Self {
        assert_eq!(piece.height() as usize, ys.len());
        debug_assert_eq!(ys, ys.into_iter().sorted().collect::<ArrayVec<[u8; 4]>>());
        Self { piece, lx, ys }
    }

    /// Returns a placed piece at the position where the interception was inserted for the placement.
    /// ```
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let placed_piece = PlacedPiece::new_with_interception(piece!(TE).with(bl(2, 1)), Lines::new(0b000100));
    /// assert_eq!(placed_piece, PlacedPiece::new(piece!(TE), 2, array_vec![1, 3, 4]));
    /// ```
    #[inline]
    pub fn new_with_interception(placement: BlPlacement, interception: Lines) -> Self {
        let using_rows = placement.using_rows().intercept(interception);
        PlacedPiece::new(
            placement.piece,
            placement.position.lx as u8,
            using_rows.ys_iter().collect(),
        )
    }

    /// Returns a vec containing all placed pieces within the height.
    /// Only one of the different orientations in the same form is included.
    pub fn make_canonical_all_iter(height: usize) -> impl Iterator<Item = Self> {
        Piece::all_iter()
            .filter(|piece| piece.canonical().is_none())
            .flat_map(move |piece| {
                let piece_blocks = piece.to_piece_blocks();
                (0..height as u8)
                    .combinations(piece_blocks.height as usize)
                    .map(|ys| ys.into_iter().sorted().collect())
                    .flat_map(move |ys: ArrayVec<[u8; 4]>| {
                        let max = 10 - piece_blocks.width as u8 + 1;
                        (0..max).map(move |lx| PlacedPiece::new(piece, lx, ys))
                    })
            })
    }

    /// Returns a vec containing all placed pieces within the height.
    /// Only one of the different orientations in the same form is included.
    /// Generate according to space on Board.
    pub fn make_canonical_on_board_iter(
        board: Board64,
        height: usize,
    ) -> impl Iterator<Item = Self> {
        Piece::all_iter()
            .filter(|piece| piece.canonical().is_none())
            .map(|piece| piece.to_piece_blocks())
            .flat_map(move |piece_blocks| {
                struct Builder {
                    piece: Piece,
                    board: Board64,
                    dxs_each_dy: ArrayVec<[ArrayVec<[i32; 4]>; 4]>,
                    pieces: Vec<PlacedPiece>,
                    ys: ArrayVec<[u8; 4]>,
                }

                impl Builder {
                    fn run(&mut self, lx: i32, by: i32, height: i32, depth: usize) {
                        let dxs = self.dxs_each_dy[depth];
                        for y in by..=height {
                            let conflicted = dxs
                                .iter()
                                .map(|&dx| Location::new(lx + dx, y))
                                .any(|location| self.board.is_occupied_at(location));
                            if conflicted {
                                continue;
                            }

                            self.ys.push(y as u8);
                            if depth + 1 < self.dxs_each_dy.len() {
                                self.run(lx, y + 1, height + 1, depth + 1)
                            } else {
                                self.pieces
                                    .push(PlacedPiece::new(self.piece, lx as u8, self.ys));
                            }
                            self.ys.pop();
                        }
                    }
                }

                let offsets: ArrayVec<[Offset; 4]> = ArrayVec::from_iter(piece_blocks.offsets);
                let dxs_each_dy: ArrayVec<[ArrayVec<[i32; 4]>; 4]> = offsets
                    .iter()
                    .map(|offset| offset.dy)
                    .sorted()
                    .dedup()
                    .map(|dy| {
                        offsets
                            .iter()
                            .filter(|&offset| offset.dy == dy)
                            .map(|offset| offset - piece_blocks.bottom_left)
                            .map(|offset| offset.dx)
                            .collect()
                    })
                    .collect();

                let mut builder = Builder {
                    piece: piece_blocks.piece,
                    board,
                    dxs_each_dy,
                    pieces: Vec::<PlacedPiece>::new(),
                    ys: ArrayVec::<[u8; 4]>::new(),
                };

                let max_x = 10 - piece_blocks.width as i32 + 1;
                let height = height as i32 - piece_blocks.height as i32;
                for lx in 0..max_x {
                    builder.run(lx, 0, height, 0);
                }

                builder.pieces
            })
    }

    /// Returns the bottom y-location of the piece.
    #[inline]
    pub fn min_y(&self) -> u8 {
        self.ys[0]
    }

    /// Returns the bottom left location of the piece.
    #[inline]
    pub fn bottom_left(&self) -> Location {
        xy(self.lx as i32, self.min_y() as i32)
    }

    /// Returns the top right location of the piece.
    #[inline]
    pub fn top_right(&self) -> Location {
        xy(
            self.lx as i32 + self.piece.width() as i32 - 1,
            *self.ys.last().unwrap() as i32,
        )
    }

    #[inline]
    pub fn locations(&self) -> [Location; 4] {
        let piece_blocks = self.piece.to_piece_blocks();
        piece_blocks
            .offsets
            .map(|offset| offset - piece_blocks.bottom_left)
            .map(|offset| {
                Location::new(
                    self.lx as i32 + offset.dx,
                    self.ys[offset.dy as usize] as i32,
                )
            })
    }

    /// Returns blank rows between the separated pieces.
    /// For example, for a T-East consisting of `y=[1, 4, 5]`, the interception_rows holds `y=[2, 3]`.
    /// ```
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let placed_piece = PlacedPiece::new(piece!(TE), 2, array_vec![1, 4, 5]);
    /// assert_eq!(placed_piece.intercepted_rows(), Lines::new(0b0001100));
    /// ```
    #[inline]
    pub fn intercepted_rows(&self) -> Lines {
        self.ys
            .iter()
            .skip(1)
            .fold((self.ys[0], Lines::blank()), |(prev_y, lines), &y| {
                let current_using_row = Lines::filled_up_to(y);
                let prev_using_row = Lines::filled_up_to(prev_y + 1);
                let intercepted = current_using_row ^ prev_using_row;
                (y, lines | intercepted)
            })
            .1
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        board.set_all(&self.locations());
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        board.unset_all(&self.locations());
    }

    /// Rows on which the block exists.
    /// For example, for a T-East consisting of `y=[1, 4, 5]`, the interception_rows holds `y=[1, 4, 5]`.
    /// ```
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let placed_piece = PlacedPiece::new(piece!(TE), 2, array_vec![1, 4, 5]);
    /// assert_eq!(placed_piece.using_rows(), Lines::new(0b0110010));
    /// ```
    #[inline]
    pub fn using_rows(&self) -> Lines {
        self.ys
            .iter()
            .fold(Lines::blank(), |lines, &y| lines | Lines::new_at(y))
    }

    /// Converts to placement according to the board.
    ///
    /// Returns None if it cannot be placed due to spaces or lock.
    /// ```
    /// use std::str::FromStr;
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let board = Board64::from_str("
    ///     ..........
    ///     .........X
    ///     XXXXXXXXXX
    ///     X...X...XX
    ///     XXXXX...XX
    /// ").unwrap();
    ///
    /// // Success
    /// let placed_piece = PlacedPiece::new(piece!(TE), 2, array_vec![1, 3, 4]);
    /// assert_eq!(placed_piece.place_according_to(board), Some(piece!(TE).with(bl(2, 1))));
    ///
    /// // A block exists under a cleared line.
    /// let placed_piece = PlacedPiece::new(piece!(ON), 7, array_vec![3, 4]);
    /// assert_eq!(placed_piece.place_according_to(board), Some(piece!(ON).with(bl(7, 2))));
    ///
    /// // A block does not exist under a cleared line.
    /// let placed_piece = PlacedPiece::new(piece!(ON), 6, array_vec![3, 4]);
    /// assert_eq!(placed_piece.place_according_to(board), None);
    ///
    /// // The piece is on the cleared line.
    /// let placed_piece_blocks = PlacedPiece::new(piece!(ON), 7, array_vec![2, 3]);
    /// assert_eq!(placed_piece.place_according_to(board), None);
    /// ```
    #[inline]
    pub fn place_according_to<T: BoardOp + Clone>(&self, board: T) -> Option<BlPlacement> {
        place_according_to(board, *self, self.using_rows(), self.intercepted_rows())
    }
}

#[cfg(test)]
mod tests {
    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn it_works() {
        let placed_piece = PlacedPiece::new(piece!(TE), 2, [1, 3, 4].into_iter().collect());
        assert_eq!(placed_piece.bottom_left(), xy(2, 1));
        assert_eq!(placed_piece.top_right(), xy(3, 4));
    }

    #[test]
    fn make_canonical_all_iter() {
        for (height, expected) in [(0, 0), (1, 7), (2, 87), (3, 312), (4, 764), (5, 1535)] {
            assert_eq!(
                PlacedPiece::make_canonical_all_iter(height).count(),
                expected
            );

            let board = Board64::blank();
            assert_eq!(
                PlacedPiece::make_canonical_on_board_iter(board, height).count(),
                expected
            );
        }
    }

    #[test]
    fn make_canonical_on_board_iter_high_board() {
        let mut board = Board64::blank();
        for y in 0..64_i32 {
            for x in 0..9 {
                board.set_at(xy(x, y));
            }
        }
        assert_eq!(
            PlacedPiece::make_canonical_on_board_iter(board, 64).count(),
            635376
        );
    }
}
