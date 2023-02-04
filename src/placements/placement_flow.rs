use thiserror::Error;
use tinyvec::ArrayVec;

use crate::{MoveRules, RotationSystem, SearchResult, With};
use crate::boards::{Board64, BoardOp, Lines};
use crate::pieces::Piece;
use crate::placements::{BlPlacement, CcPlacement, PlacedPiece};
use crate::prelude::{BlPosition, PlacedPieceBlocks, PlacedPieceBlocksFlow};

/// Returns true if all placements have been successful from the initial board according to the Rotation System.
///
/// `validator` receives (board after clearing, subsequent placement) and returns whether to continue searching the board.
///
fn satisfies_dyn(
    board: Board64,
    placements: &Vec<CcPlacement>,
    validator: impl Fn(&Board64, &CcPlacement) -> SearchResult,
) -> bool {
    let mut board = board.after_clearing();
    for placement in placements {
        if validator(&board, placement) == SearchResult::Pruned {
            return false;
        }

        if let None = placement.place_on_and_clear_lines(&mut board) {
            return false;
        }
    }
    true
}

/// This holds the initial board and the subsequent placements.
/// They are placed in order from the head.
///
/// Note that placement must consider line clearing and depends on the previous placement.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PlacementFlow {
    pub initial_board: Board64,
    pub placements: Vec<CcPlacement>,
}

impl PlacementFlow {
    #[inline]
    pub fn new<P: Into<CcPlacement>>(initial_board: Board64, placements: Vec<P>) -> Self {
        Self::from_iter(initial_board, placements.into_iter())
    }

    #[inline]
    pub fn from_iter<P: Into<CcPlacement>>(initial_board: Board64, placements: impl IntoIterator<Item=P>) -> Self {
        Self { initial_board, placements: placements.into_iter().map(Into::<CcPlacement>::into).collect() }
    }

    #[inline]
    pub fn from_slice<'a, P>(initial_board: Board64, placements: &'a [P]) -> Self where &'a P: Into<CcPlacement> {
        Self::from_iter(initial_board, placements)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.placements.len()
    }

    /// Returns the board after all have been placed.
    pub fn board_all_placed(&self) -> Option<Board64> {
        let mut board = self.initial_board.after_clearing();
        for placement in &self.placements {
            if let None = placement.place_on_and_clear_lines(&mut board) {
                return None;
            }
        }
        Some(board)
    }

    /// Returns true if all placements have been successfully placed from the initial board.
    ///
    /// Note that it does not depend on Rotation System. It depends only on spaces and landing.
    ///
    /// If you want to make a decision that considers the Rotation System, use `can_stack_all()`.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str("
    ///     ...#######
    ///     ...#######
    ///     ...#######
    ///     ...#######
    /// ").unwrap();
    ///
    /// let flow = PlacementFlow::new(
    ///     board,
    ///     vec![
    ///         piece!(LN).with(cc(1, 0)),
    ///         piece!(ON).with(cc(0, 0)),
    ///         piece!(JS).with(cc(1, 1)),
    ///     ],
    ///  );
    ///  assert!(flow.can_place_all());
    ///
    /// let flow = PlacementFlow::new(
    ///     board,
    ///     vec![
    ///         piece!(ON).with(cc(1, 1)),
    ///     ],
    ///  );
    ///  assert!(!flow.can_place_all());
    /// ```
    #[inline]
    pub fn can_place_all(&self) -> bool {
        self.board_all_placed().is_some()
    }

    /// Returns true if all placements have been successful from the initial board according to the Rotation System.
    ///
    /// Note that the same form will succeed regardless of the orientation.
    /// If you want to be strict, use `can_stack_all_strictly()`.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// use Shape::*;
    /// use Orientation::*;
    ///
    /// let board = Board64::from_str("
    ///     .....#####
    ///     #...######
    /// ").unwrap();
    ///
    /// let flow = PlacementFlow::new(
    ///     board,
    ///     vec![
    ///         T.with(North).with(cc(1, 1)),
    ///         S.with(South).with(cc(3, 1)),
    ///     ],
    ///  );
    ///  assert!(flow.can_stack_all(&MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///  assert!(flow.can_stack_all_strictly(&MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///  assert!(!flow.can_stack_all(&MoveRules::srs(AllowMove::Harddrop), bl(3, 20)));
    ///
    /// let flow = PlacementFlow::new(
    ///     board,
    ///     vec![
    ///         // S-North is unreachable, but S-South is reachable in SRS.
    ///         T.with(North).with(cc(1, 1)),
    ///         S.with(North).with(cc(3, 0)),
    ///     ],
    ///  );
    ///  assert!(flow.can_stack_all(&MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///  assert!(!flow.can_stack_all_strictly(&MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///  assert!(!flow.can_stack_all(&MoveRules::srs(AllowMove::Harddrop), bl(3, 20)));
    /// ```
    #[inline]
    pub fn can_stack_all<T: RotationSystem>(&self, move_rules: &MoveRules<T>, spawn: BlPosition) -> bool {
        self.can_stack_all_dyn(move_rules, move |_, _| Some(spawn))
    }

    /// It's similar to `can_stack_all()` except that spawn can be set dynamically.
    #[inline]
    pub fn can_stack_all_dyn<T: RotationSystem>(&self, move_rules: &MoveRules<T>, spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>) -> bool {
        satisfies_dyn(self.initial_board, &self.placements, |&board, placement| {
            if let Some(spawn) = spawn_func(placement.piece, &board) {
                if move_rules.can_reach(placement.to_bl_placement(), board, placement.piece.with(spawn)) {
                    return SearchResult::Success;
                }
            }
            SearchResult::Pruned
        })
    }

    /// It's similar to `can_stack_all()` except that the orientation is strictly checked.
    #[inline]
    pub fn can_stack_all_strictly<T: RotationSystem>(&self, move_rules: &MoveRules<T>, spawn: BlPosition) -> bool {
        self.can_stack_all_strictly_dyn(move_rules, move |_, _| Some(spawn))
    }

    /// It's similar to `can_stack_all_strictly()` except that spawn can be set dynamically.
    #[inline]
    pub fn can_stack_all_strictly_dyn<T: RotationSystem>(&self, move_rules: &MoveRules<T>, spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>) -> bool {
        satisfies_dyn(self.initial_board, &self.placements, |&board, placement| {
            if let Some(spawn) = spawn_func(placement.piece, &board) {
                if move_rules.can_reach_strictly(placement.to_bl_placement(), board, placement.piece.with(spawn)) {
                    return SearchResult::Success;
                }
            }
            SearchResult::Pruned
        })
    }

    /// Returns the flow bound with placed piece blocks.
    /// `PlacedPieceBlocks` is a direct representation of the actual placement locations.
    /// However, `PlacedPieceBlocks` is expensive to make because it caches several computations besides the placement locations.
    /// Therefore, bind a reference to `PlacedPieceBlocks` from this function.
    /// This makes it possible to create a `PlacedPieceBlocksFlow` with less overhead.
    ///
    /// You will need to prepare the `PlacedPieceBlocks` to bind by yourself.
    /// Please refer to the following code to cache them in advance.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str("
    ///     X........X
    ///     XXXXXXXXXX
    ///     X........X
    /// ").unwrap();
    /// let placement_flow = PlacementFlow::new(
    ///     board,
    ///     vec![piece!(SE).with(bl(3, 0))],
    /// );
    ///
    /// // Case 1: Create PlacedPieceBlocksFlow directly.
    /// {
    ///     let binding: Vec<PlacedPieceBlocks> = placement_flow.to_placed_pieces()
    ///         .expect("Failed to place")
    ///         .into_iter()
    ///         .map(PlacedPieceBlocks::from)
    ///         .collect();
    ///      let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(
    ///          placement_flow.initial_board,
    ///          binding.iter().collect(),
    ///      );
    ///
    ///      assert_eq!(placed_piece_blocks_flow.len(), 1);
    ///  }
    ///
    ///  // Case 2: Injecting from cache.
    ///  {
    ///      // Generate all placed pieces that fit under 5 in height.
    ///      let height = 5;
    ///      let cache: Vec<PlacedPieceBlocks> = PlacedPiece::make_canonical_all_iter(height)
    ///          .map(PlacedPieceBlocks::from)
    ///          .collect();
    ///
    ///      let placed_piece_blocks_flow = placement_flow.bind_blocks(|placed_piece| {
    ///          cache.iter()
    ///              .find(|&it| it.placed_piece == placed_piece)
    ///              .expect("Placed piece blocks not found in cache")
    ///      }).expect("Failed to place");
    ///
    ///      assert_eq!(placed_piece_blocks_flow.len(), 1);
    /// }
    /// ```
    #[inline]
    pub fn bind_blocks<'a>(&self, binder: impl Fn(PlacedPiece) -> &'a PlacedPieceBlocks) -> Option<PlacedPieceBlocksFlow<'a>> {
        self.to_placed_pieces().map(|placed_pieces| {
            PlacedPieceBlocksFlow::new(self.initial_board, placed_pieces.into_iter().map(binder).collect())
        })
    }

    /// Convert to placed piece vec.
    /// Returns None if the placement is not possible.
    ///
    /// It can be used mainly for binding to `PlacedPieceBlocks`.
    pub fn to_placed_pieces(&self) -> Option<Vec<PlacedPiece>> {
        let mut board = self.initial_board.after_clearing();
        let mut placed_pieces = Vec::<PlacedPiece>::with_capacity(self.len());
        let mut interception = Lines::blank();

        for placement in &self.placements {
            match placement.place_on_and_clear_lines(&mut board) {
                Some(lines_cleared) => {
                    let ys: ArrayVec<[u8; 4]> = placement.using_rows().intercept(interception)
                        .ys_iter()
                        .collect();

                    let bl = placement.to_bl_placement().position;
                    placed_pieces.push(PlacedPiece::new(placement.piece, bl.lx as u8, ys));

                    interception |= lines_cleared.intercept(interception);
                }
                None => return None
            }
        }

        Some(placed_pieces)
    }
}


/// A collection of errors that occur when making the placement flow.
#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PlacementFlowTryFromError {
    #[error("It contains placement that cannot be placed.")]
    NoPlaceable,
}

impl TryFrom<PlacedPieceBlocksFlow<'_>> for PlacementFlow {
    type Error = PlacementFlowTryFromError;

    fn try_from(flow: PlacedPieceBlocksFlow) -> Result<Self, Self::Error> {
        let mut placements = Vec::<BlPlacement>::with_capacity(flow.refs.len());
        let mut board = flow.initial_board;

        for placed_piece_blocks in flow.refs {
            if let Some(placement) = placed_piece_blocks.place_according_to(board) {
                placements.push(placement);
                board.set_all(&placed_piece_blocks.locations);
            } else {
                return Err(Self::Error::NoPlaceable);
            }
        }

        Ok(PlacementFlow::new(flow.initial_board, placements))
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tinyvec::array_vec;

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn works_from_placements() {
        let cc_flow = PlacementFlow::new(
            Board64::blank(),
            vec![
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 1)),
                piece!(JS).with(cc(1, 3)),
            ],
        );

        let bl_flow = PlacementFlow::new(
            Board64::blank(),
            vec![
                piece!(LN).with(bl(0, 0)),
                piece!(ON).with(bl(0, 1)),
                piece!(JS).with(bl(0, 2)),
            ],
        );
        assert_eq!(cc_flow, bl_flow);

        let tr_flow = PlacementFlow::new(
            Board64::blank(),
            vec![
                piece!(LN).with(tr(2, 1)),
                piece!(ON).with(tr(1, 2)),
                piece!(JS).with(tr(2, 3)),
            ],
        );
        assert_eq!(cc_flow, tr_flow);
    }

    #[test]
    fn board_all_placed() {
        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                ##########
                ##########
                ##########
                ##########
            ").unwrap(),
            &[
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 1)),
                piece!(JS).with(cc(1, 3)),
            ],
        );
        assert!(flow.can_place_all());
        let board_placed = flow.board_all_placed().unwrap();
        assert_eq!(board_placed.count_blocks(), 12);

        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                ...#######
                ...#######
                ...#######
                ...#######
            ").unwrap(),
            &[
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 0)),
                piece!(JS).with(cc(1, 1)),
            ],
        );
        assert!(flow.can_place_all());
        let board_placed = flow.board_all_placed().unwrap();
        assert_eq!(board_placed.count_blocks(), 0);
    }

    #[test]
    fn can_place_all() {
        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                ##########
                ##########
                ##########
                ##########
            ").unwrap(),
            &[
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 1)),
            ],
        );
        assert!(flow.can_place_all());

        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                ...#######
                ...#######
                ...#######
                ...#######
            ").unwrap(),
            &[
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 1)),
            ],
        );
        assert!(!flow.can_place_all());
    }

    #[test]
    fn can_stack_all() {
        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                ##########
                ##########
                ##########
                ##########
            ").unwrap(),
            &[
                piece!(LE).with(cc(0, 1)),
            ],
        );
        assert!(flow.can_stack_all(&MoveRules::default(), bl(4, 20)));

        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                .#########
                .#########
                ..########
            ").unwrap(),
            &[
                piece!(LE).with(cc(0, 1)),
            ],
        );
        assert!(flow.can_place_all());
        assert!(!flow.can_stack_all(&MoveRules::default(), bl(4, 20)));

        let flow = PlacementFlow::from_slice(
            Board64::from_str("
                #..#######
                ..########
            ").unwrap(),
            &[
                piece!(SN).with(cc(1, 0)),
            ],
        );
        assert!(flow.can_place_all());
        assert!(flow.can_stack_all(&MoveRules::srs(AllowMove::Softdrop), bl(4, 20)));
        assert!(!flow.can_stack_all(&MoveRules::srs(AllowMove::Harddrop), bl(4, 20)));
    }

    #[test]
    fn from_placed_piece_flow() {
        let board = Board64::from_str("
            ####....##
            ####...###
            ####..####
            ####...###
        ").unwrap();

        {
            let placed_pieces: Vec<PlacedPieceBlocks> = vec![
                PlacedPiece::new(piece!(TE), 4, array_vec![0, 1, 2]).into(),
                PlacedPiece::new(piece!(LW), 4, array_vec![0, 2, 3]).into(),
                PlacedPiece::new(piece!(JE), 6, array_vec![0, 2, 3]).into(),
            ];
            let flow = PlacedPieceBlocksFlow::new(board, placed_pieces.iter().collect());

            assert_eq!(PlacementFlow::try_from(flow), Ok(PlacementFlow::from_slice(
                board,
                &[
                    piece!(TE).with(cc(4, 1)),
                    piece!(LW).with(cc(5, 1)),
                    piece!(JE).with(cc(6, 1)),
                ],
            )));
        }

        {
            let placed_pieces: Vec<PlacedPieceBlocks> = vec![
                PlacedPiece::new(piece!(LW), 4, array_vec![0, 2, 3]).into(),
                PlacedPiece::new(piece!(JE), 6, array_vec![0, 2, 3]).into(),
                PlacedPiece::new(piece!(TE), 4, array_vec![0, 1, 2]).into(),
            ];
            let flow = PlacedPieceBlocksFlow::new(board, placed_pieces.iter().collect());

            assert_eq!(PlacementFlow::try_from(flow), Err(PlacementFlowTryFromError::NoPlaceable));
        }
    }

    #[test]
    fn empty() {
        let placement_flow = PlacementFlow::new(Board64::blank(), Vec::<CcPlacement>::new());
        assert_eq!(placement_flow.len(), 0);
        assert!(placement_flow.can_place_all());
        assert!(placement_flow.can_stack_all(&MoveRules::default(), bl(4, 20)));
        assert!(placement_flow.can_stack_all_strictly(&MoveRules::default(), bl(4, 20)));
    }
}
