use crate::boards::{BoardOp, Lines};
use crate::coordinates::{bl, Location};
use crate::internal_macros::forward_ref_from;
use crate::placements::{BlPlacement, PlacedPiece};
use crate::With;

/// The offsets of blocks that make up a placed piece.
///
/// This caches the results of the placed piece calculation.
/// It prioritizes speed and records what can be calculated in advance.
/// Therefore, this struct is relatively large and slower when copied, so it's recommended to do less make/clone.
#[derive(Clone, Hash, Debug)]
pub struct PlacedPieceBlocks {
    pub placed_piece: PlacedPiece,
    pub locations: [Location; 4],

    /// Blank rows between the separated pieces.
    /// See `PlacedPiece::intercepted_rows()`
    pub intercepted_rows: Lines,

    /// Rows on which the block exists.
    /// See `PlacedPiece::using_rows()`
    pub using_rows: Lines,
}

impl PlacedPieceBlocks {
    /// Generate `PlacedPieceBlocks`.
    /// Note that this is a bit more demanding computationally than `new`.
    pub fn make(placed_piece: PlacedPiece) -> Self {
        Self {
            placed_piece,
            locations: placed_piece.locations(),
            intercepted_rows: placed_piece.intercepted_rows(),
            using_rows: placed_piece.using_rows(),
        }
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    #[inline]
    pub fn set_all(&self, board: &mut impl BoardOp) {
        board.set_all(self.locations.as_slice());
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    #[inline]
    pub fn unset_all(&self, board: &mut impl BoardOp) {
        board.unset_all(self.locations.as_slice());
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
    /// let placed_piece_blocks: PlacedPieceBlocks = PlacedPiece::new(piece!(TE), 2, array_vec![1, 3, 4]).into();
    /// assert_eq!(placed_piece_blocks.place_according_to(board), Some(piece!(TE).with(bl(2, 1))));
    ///
    /// // A block exists under a cleared line.
    /// let placed_piece_blocks: PlacedPieceBlocks = PlacedPiece::new(piece!(ON), 7, array_vec![3, 4]).into();
    /// assert_eq!(placed_piece_blocks.place_according_to(board), Some(piece!(ON).with(bl(7, 2))));
    ///
    /// // A block does not exist under a cleared line.
    /// let placed_piece_blocks: PlacedPieceBlocks = PlacedPiece::new(piece!(ON), 6, array_vec![3, 4]).into();
    /// assert_eq!(placed_piece_blocks.place_according_to(board), None);
    ///
    /// // The piece is on the cleared line.
    /// let placed_piece_blocks: PlacedPieceBlocks = PlacedPiece::new(piece!(ON), 7, array_vec![2, 3]).into();
    /// assert_eq!(placed_piece_blocks.place_according_to(board), None);
    /// ```
    #[inline]
    pub fn place_according_to<T: BoardOp + Clone>(&self, board: T) -> Option<BlPlacement> {
        let mut board = board.clone();
        let lines_cleared = board.clear_lines();

        // Whether cleared rows and the piece overlap or not.
        if !(self.using_rows & lines_cleared).is_blank() {
            return None;
        }

        // Whether all required rows have been cleared.
        if (self.intercepted_rows & lines_cleared) != self.intercepted_rows {
            return None;
        }

        let piece_by = self.placed_piece.min_y() as u32;
        let lines_cleared_below_piece = lines_cleared & Lines::filled_up_to(piece_by);
        let by = piece_by as i32 - lines_cleared_below_piece.count() as i32;
        let lx = self.placed_piece.lx as i32;

        let placement = self.placed_piece.piece.with(bl(lx, by));
        if placement.can_place_on(&board) {
            Some(placement)
        } else {
            None
        }
    }
}

impl From<PlacedPiece> for PlacedPieceBlocks {
    fn from(placed_piece: PlacedPiece) -> Self {
        Self::make(placed_piece)
    }
}

forward_ref_from!(PlacedPieceBlocks, from PlacedPiece);
