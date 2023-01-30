use fxhash::FxHashSet;

use crate::{MoveRules, RotationSystem, SearchResult, With};
use crate::boards::{Board64, BoardOp};
use crate::coordinates::BlPosition;
use crate::pieces::Piece;
use crate::placements::PlacedPieceBlocks;
use crate::prelude::BlPlacement;

/// Returns a flow finds that all placements have been successful from the initial board.
///
/// `validator` receives (board before clearing, subsequent placement) and returns whether to continue searching the board.
///
/// The search status is based on the combination of blocks used to determine if they have already been explored.
/// Note that it's not possible to consider the order.
/// For example, if a board created with "SZ" is first explored with S->Z, the board will not be explored with Z->S.
fn find_one_dyn(
    initial_board: Board64,
    refs: Vec<&PlacedPieceBlocks>,
    validator: impl Fn(&Board64, BlPlacement) -> SearchResult,
) -> Option<PlacedPieceBlocksFlow> {
    if refs.is_empty() {
        return Some(PlacedPieceBlocksFlow::new(initial_board, refs));
    }

    assert!(refs.len() < 64, "refs length supports up to 64.");

    struct Builder<'a> {
        refs: Vec<&'a PlacedPieceBlocks>,
        results: Vec<&'a PlacedPieceBlocks>,
        visited: FxHashSet<u64>,
    }

    impl Builder<'_> {
        fn build(
            &mut self,
            board: Board64,
            remaining: u64,
            validator: &impl Fn(&Board64, BlPlacement) -> SearchResult,
        ) -> bool {
            let mut candidates = remaining;
            while 0 < candidates {
                let next_candidates = candidates & (candidates - 1);
                let bit = candidates - next_candidates;
                let next_remaining = remaining - bit;

                candidates = next_candidates;

                if self.visited.contains(&next_remaining) {
                    // Already searched.
                    continue;
                }

                let placed_piece_blocks = self.refs[bit.trailing_zeros() as usize];

                if let Some(placement) = placed_piece_blocks.place_according_to(board) {
                    if validator(&board, placement) == SearchResult::Pruned {
                        continue;
                    }

                    self.results.push(placed_piece_blocks);

                    if next_remaining == 0 {
                        return true;
                    }

                    self.visited.insert(remaining);

                    let mut next_board = board;
                    next_board.set_all(&placed_piece_blocks.locations);

                    if self.build(next_board, next_remaining, validator) {
                        return true;
                    }
                    self.results.pop();
                }
            }

            false
        }
    }

    let len = refs.len();
    let mut builder = Builder {
        refs,
        results: Vec::with_capacity(len),
        visited: FxHashSet::default(),
    };

    if builder.build(initial_board, (1u64 << len) - 1, &validator) {
        Some(PlacedPieceBlocksFlow::new(initial_board, builder.results))
    } else {
        None
    }
}

/// This holds the initial board and reference of the subsequent placed piece blocks.
/// They are placed in order from the head.
///
/// PlacedPieceBlocks are held by reference due to the high cost of generation.
#[derive(Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct PlacedPieceBlocksFlow<'a> {
    pub initial_board: Board64,
    pub refs: Vec<&'a PlacedPieceBlocks>,
}

impl<'a> PlacedPieceBlocksFlow<'a> {
    #[inline]
    pub fn new(initial_board: Board64, refs: Vec<&'a PlacedPieceBlocks>) -> Self {
        Self { initial_board, refs }
    }

    /// Returns a flow finds that all placements have been successful from the initial board.
    /// Returns None if placements is not placeable.
    ///
    /// Note that it does not depend on Rotation System. It depends only on spaces and landing.
    pub fn find_one_placeable(initial_board: Board64, refs: Vec<&'a PlacedPieceBlocks>) -> Option<Self> {
        find_one_dyn(initial_board, refs, |_, _| {
            SearchResult::Success
        })
    }

    /// Returns a flow finds that all placements have been successful from the initial board according to the Rotation System.
    /// Returns None if placements is not stackable.
    ///
    /// Note that the same form will succeed regardless of the orientation.
    /// If you want to be strict, use `find_one_stackable_strictly()`.
    #[inline]
    pub fn find_one_stackable<T: RotationSystem>(
        initial_board: Board64,
        refs: Vec<&'a PlacedPieceBlocks>,
        move_rules: MoveRules<'a, T>,
        spawn: BlPosition,
    ) -> Option<Self> {
        Self::find_one_stackable_dyn(initial_board, refs, move_rules, |_, _| Some(spawn))
    }

    /// It's similar to `find_one_stackable()` except that spawn can be set dynamically.
    pub fn find_one_stackable_dyn<T: RotationSystem>(
        initial_board: Board64,
        refs: Vec<&'a PlacedPieceBlocks>,
        move_rules: MoveRules<T>,
        spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>,
    ) -> Option<Self> {
        find_one_dyn(initial_board, refs, |board, placement| {
            let board_to_place = board.after_clearing();
            if let Some(spawn) = spawn_func(placement.piece, &board_to_place) {
                if move_rules.can_reach(placement, board_to_place, placement.piece.with(spawn)) {
                    return SearchResult::Success;
                }
            }
            SearchResult::Pruned
        })
    }

    /// It's similar to `find_one_stackable()` except that the orientation is strictly checked.
    #[inline]
    pub fn find_one_stackable_strictly<T: RotationSystem>(
        initial_board: Board64,
        refs: Vec<&'a PlacedPieceBlocks>,
        move_rules: MoveRules<'a, T>,
        spawn: BlPosition,
    ) -> Option<Self> {
        Self::find_one_stackable_strictly_dyn(initial_board, refs, move_rules, move |_, _| Some(spawn))
    }

    /// It's similar to `find_one_stackable_strictly()` except that spawn can be set dynamically.
    pub fn find_one_stackable_strictly_dyn<T: RotationSystem>(
        initial_board: Board64,
        refs: Vec<&'a PlacedPieceBlocks>,
        move_rules: MoveRules<T>,
        spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>,
    ) -> Option<Self> {
        find_one_dyn(initial_board, refs, |board, placement| {
            let board_to_place = board.after_clearing();
            if let Some(spawn) = spawn_func(placement.piece, &board_to_place) {
                if move_rules.can_reach_strictly(placement, board_to_place, placement.piece.with(spawn)) {
                    return SearchResult::Success;
                }
            }
            SearchResult::Pruned
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.refs.len()
    }

    /// Returns the board with all blocks merged.
    /// The result is always returned, regardless of block overlap, no placeable combinations, etc.
    /// ```
    /// use std::str::FromStr;
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// let placed_piece_blocks = vec![
    ///     PlacedPieceBlocks::make(PlacedPiece::new(piece!(JW), 1, array_vec![0, 1, 2])),
    ///     PlacedPieceBlocks::make(PlacedPiece::new(piece!(ON), 0, array_vec![1, 2])),
    ///     PlacedPieceBlocks::make(PlacedPiece::new(piece!(LS), 0, array_vec![0, 3])),
    /// ];
    /// let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(
    ///     Board64::from_str("
    ///         ...XXXXXXX
    ///         ...XXXXXXX
    ///         ...XXXXXXX
    ///         ...XXXXXXX
    ///     ").unwrap(),
    ///     placed_piece_blocks.iter().collect(),
    /// );
    /// assert_eq!(
    ///     placed_piece_blocks_flow.board_all_merged(),
    ///     Board64::from_str("
    ///         XXXXXXXXXX
    ///         XXXXXXXXXX
    ///         XXXXXXXXXX
    ///         XXXXXXXXXX
    ///     ").unwrap(),
    /// );
    /// ```
    pub fn board_all_merged(&self) -> Board64 {
        self.refs.iter()
            .fold(self.initial_board, |mut board: Board64, &blocks| {
                board.set_all(&blocks.locations);
                board
            })
    }

    /// Returns true if all placements have been successfully placed from the initial board.
    ///
    /// Note that it does not depend on Rotation System. It depends only on spaces and landing.
    ///
    /// If you want to make a decision that considers the Rotation System, use `can_stack_all()`.
    /// ```
    /// use std::str::FromStr;
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str("
    ///     ...#######
    ///     ...#######
    ///     ...#######
    ///     ...#######
    /// ").unwrap();
    ///
    /// {
    ///     let placed_piece_blocks: Vec<PlacedPieceBlocks> = vec![
    ///         PlacedPiece::new(piece!(LN), 0, array_vec![0, 1]).into(),
    ///         PlacedPiece::new(piece!(ON), 0, array_vec![1, 2]).into(),
    ///         PlacedPiece::new(piece!(JS), 0, array_vec![2, 3]).into(),
    ///     ];
    ///     let flow = PlacedPieceBlocksFlow::new(
    ///         board,
    ///         placed_piece_blocks.iter().collect(),
    ///     );
    ///     assert!(flow.can_place_all());
    /// }
    ///
    /// {
    ///     let placed_piece_blocks: Vec<PlacedPieceBlocks> = vec![
    ///         PlacedPiece::new(piece!(ON), 0, array_vec![1, 2]).into(),
    ///     ];
    ///     let flow = PlacedPieceBlocksFlow::new(
    ///         board,
    ///         placed_piece_blocks.iter().collect(),
    ///     );
    ///     assert!(!flow.can_place_all());
    /// }
    /// ```
    #[inline]
    pub fn can_place_all(&self) -> bool {
        let mut board = self.initial_board;

        for &placed_piece_blocks in self.refs.iter() {
            if let Some(_) = placed_piece_blocks.place_according_to(board) {
                board.set_all(&placed_piece_blocks.locations);
            } else {
                return false;
            }
        }

        true
    }

    /// Returns true if all placements have been successful from the initial board according to the Rotation System.
    ///
    /// Note that the same form will succeed regardless of the orientation.
    /// If you want to be strict, use `can_stack_all_strictly()`.
    /// ```
    /// use std::str::FromStr;
    /// use tinyvec::array_vec;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// use Shape::*;
    /// use Orientation::*;
    ///
    /// let board = Board64::from_str("
    ///     XXX..XXXXX
    ///     XXX..XXXXX
    ///     XX..XXXXXX
    ///     XX..XXXXXX
    /// ").unwrap();
    ///
    /// {
    ///     let placed_piece_blocks: Vec<PlacedPieceBlocks> = vec![
    ///         PlacedPiece::new(S.with(South), 2, array_vec![1, 2]).into(),
    ///         PlacedPiece::new(S.with(South), 2, array_vec![0, 3]).into(),
    ///     ];
    ///     let flow = PlacedPieceBlocksFlow::new(
    ///         board,
    ///         placed_piece_blocks.iter().collect(),
    ///     );
    ///     assert!(flow.can_stack_all(MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///     assert!(flow.can_stack_all_strictly(MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///     assert!(!flow.can_stack_all(MoveRules::srs(AllowMove::Harddrop), bl(3, 20)));
    /// }
    /// {
    ///     let placed_piece_blocks: Vec<PlacedPieceBlocks> = vec![
    ///         // S-North is unreachable, but S-South is reachable in SRS.
    ///         PlacedPiece::new(S.with(North), 2, array_vec![1, 2]).into(),
    ///         PlacedPiece::new(S.with(North), 2, array_vec![0, 3]).into(),
    ///     ];
    ///     let flow = PlacedPieceBlocksFlow::new(
    ///         board,
    ///         placed_piece_blocks.iter().collect(),
    ///     );
    ///     assert!(flow.can_stack_all(MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///     assert!(!flow.can_stack_all_strictly(MoveRules::srs(AllowMove::Softdrop), bl(3, 20)));
    ///     assert!(!flow.can_stack_all(MoveRules::srs(AllowMove::Harddrop), bl(3, 20)));
    /// }
    /// ```
    #[inline]
    pub fn can_stack_all<T: RotationSystem>(&self, move_rules: MoveRules<T>, spawn: BlPosition) -> bool {
        self.can_stack_all_dyn(move_rules, move |_, _| Some(spawn))
    }

    /// It's similar to `can_stack_all()` except that spawn can be set dynamically.
    pub fn can_stack_all_dyn<T: RotationSystem>(&self, move_rules: MoveRules<T>, spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>) -> bool {
        let mut board = self.initial_board;
        for &placed_piece_blocks in self.refs.iter() {
            if let Some(placement) = placed_piece_blocks.place_according_to(board) {
                let board_to_place = board.after_clearing();

                match spawn_func(placement.piece, &board_to_place) {
                    Some(spawn) => {
                        if !move_rules.can_reach(placement, board_to_place, placement.piece.with(spawn)) {
                            return false;
                        }
                    }
                    None => return false,
                }

                board.set_all(&placed_piece_blocks.locations);
            } else {
                return false;
            }
        }
        true
    }

    /// It's similar to `can_stack_all()` except that the orientation is strictly checked.
    #[inline]
    pub fn can_stack_all_strictly<T: RotationSystem>(&self, move_rules: MoveRules<T>, spawn: BlPosition) -> bool {
        self.can_stack_all_strictly_dyn(move_rules, move |_, _| Some(spawn))
    }

    /// It's similar to `can_stack_all_strictly()` except that spawn can be set dynamically.
    pub fn can_stack_all_strictly_dyn<T: RotationSystem>(&self, move_rules: MoveRules<T>, spawn_func: impl Fn(Piece, &Board64) -> Option<BlPosition>) -> bool {
        let mut board = self.initial_board;
        for &placed_piece_blocks in self.refs.iter() {
            if let Some(placement) = placed_piece_blocks.place_according_to(board) {
                let board_to_place = board.after_clearing();

                match spawn_func(placement.piece, &board_to_place) {
                    Some(spawn) => {
                        if !move_rules.can_reach_strictly(placement, board_to_place, placement.piece.with(spawn)) {
                            return false;
                        }
                    }
                    None => return false,
                }

                board.set_all(&placed_piece_blocks.locations);
            } else {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tinyvec::array_vec;

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn new_case1() {
        let board = Board64::from_str("
            ...#######
            ...#######
            ...#######
            ...#######
        ").unwrap();

        let placement_flow = PlacementFlow::from_slice(
            board,
            &[
                piece!(LN).with(cc(1, 0)),
                piece!(ON).with(cc(0, 0)),
                piece!(JS).with(cc(1, 1)),
            ],
        );
        assert!(placement_flow.can_place_all());

        let placed_piece_blocks: Vec<PlacedPieceBlocks> = placement_flow.to_placed_pieces()
            .unwrap()
            .iter()
            .map(|it| it.into())
            .collect();

        let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());

        assert_eq!(
            PlacementFlow::try_from(placed_piece_blocks_flow),
            Ok(placement_flow),
        );
    }

    #[test]
    fn new_case2() {
        let board = Board64::from_str("
            ...#######
            ...#######
            #..#######
            #..#######
            ...#######
            ...#######
        ").unwrap();

        let placement_flow = PlacementFlow::from_slice(
            board,
            &[
                piece!(JS).with(bl(0, 0)),
                piece!(ON).with(bl(1, 1)),
                piece!(TS).with(bl(0, 0)),
                piece!(LS).with(bl(0, 0)),
            ],
        );
        assert!(placement_flow.can_place_all());

        let placed_piece_blocks: Vec<PlacedPieceBlocks> = placement_flow.to_placed_pieces()
            .unwrap()
            .iter()
            .map(|it| it.into())
            .collect();

        let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());

        assert_eq!(
            PlacementFlow::try_from(placed_piece_blocks_flow),
            Ok(placement_flow),
        );
    }

    #[test]
    fn board_all_merged_case1() {
        let placed_pieces: Vec<PlacedPieceBlocks> = vec![
            PlacedPiece::new(piece!(LN), 0, array_vec![0, 2]).into(),
            PlacedPiece::new(piece!(ON), 0, array_vec![2, 3]).into(),
            PlacedPiece::new(piece!(JS), 0, array_vec![3, 4]).into(),
        ];
        let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(
            Board64::from_str("
                ..........
                ..........
                ##########
                ..........
            ").unwrap(),
            placed_pieces.iter().collect(),
        );
        assert_eq!(placed_piece_blocks_flow.board_all_merged().count_blocks(), 22);
    }

    #[test]
    fn board_all_merged_case2() {
        let placed_piece_blocks: Vec<PlacedPieceBlocks> = vec![
            PlacedPiece::new(piece!(JS), 0, array_vec![0, 3]).into(),
            PlacedPiece::new(piece!(ON), 1, array_vec![1, 2]).into(),
            PlacedPiece::new(piece!(LE), 0, array_vec![0, 1, 2]).into(),
        ];
        let placed_piece_blocks_flow = PlacedPieceBlocksFlow::new(
            Board64::from_str("
                ...#######
                ...#######
                ...#######
                ...#######
            ").unwrap(),
            placed_piece_blocks.iter().collect(),
        );
        assert_eq!(placed_piece_blocks_flow.board_all_merged().count_blocks(), 40);
        assert_eq!(placed_piece_blocks_flow.board_all_merged().after_clearing().count_blocks(), 0);
    }

    #[test]
    fn find_one_placeable() {
        let board = Board64::from_str("
            ...#######
            ...#######
            ...#######
        ").unwrap();
        let placed_piece_blocks = vec![
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(ON), 1, array_vec![0, 3])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(JN), 0, array_vec![2, 3])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(LS), 0, array_vec![0, 1])),
        ];

        let placed_piece_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());
        assert_eq!(placed_piece_flow.len(), 3);
        assert!(!placed_piece_flow.can_place_all());

        let placed_piece_flow = PlacedPieceBlocksFlow::find_one_placeable(board, placed_piece_blocks.iter().collect()).unwrap();
        assert_eq!(placed_piece_flow.len(), 3);
        assert!(placed_piece_flow.can_place_all());
    }

    #[test]
    fn find_one_stackable_with_s_south() {
        let board = Board64::from_str("
            ###..#####
            ###..#####
            ###..#####
            ##..######
            ##..######
            ##..######
        ").unwrap();
        let placed_piece_blocks = vec![
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SS), 2, array_vec![0, 5])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SS), 2, array_vec![1, 4])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SS), 2, array_vec![2, 3])),
        ];

        let placed_piece_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());
        assert_eq!(placed_piece_flow.len(), 3);
        assert!(!placed_piece_flow.can_place_all());

        {
            let move_rules = MoveRules::srs(AllowMove::Softdrop);
            let spawn = bl(4, 20);

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_some());
            if let Some(placed_piece_flow) = placed_piece_flow {
                assert!(placed_piece_flow.can_stack_all(move_rules, spawn));
                assert!(placed_piece_flow.can_stack_all_strictly(move_rules, spawn));
            }

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable_strictly(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_some());
            if let Some(placed_piece_flow) = placed_piece_flow {
                assert!(placed_piece_flow.can_stack_all(move_rules, spawn));
                assert!(placed_piece_flow.can_stack_all_strictly(move_rules, spawn));
            }
        }
        {
            let move_rules = MoveRules::srs(AllowMove::Harddrop);
            let spawn = bl(4, 20);

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_none());

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable_strictly(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_none());
        }
    }

    #[test]
    fn find_one_stackable_with_s_north() {
        let board = Board64::from_str("
            ###..#####
            ###..#####
            ###..#####
            ##..######
            ##..######
            ##..######
        ").unwrap();
        let placed_piece_blocks = vec![
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SN), 2, array_vec![0, 5])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SN), 2, array_vec![1, 4])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SN), 2, array_vec![2, 3])),
        ];

        let placed_piece_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());
        assert_eq!(placed_piece_flow.len(), 3);
        assert!(!placed_piece_flow.can_place_all());

        {
            let move_rules = MoveRules::srs(AllowMove::Softdrop);
            let spawn = bl(4, 20);

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_some());
            if let Some(placed_piece_flow) = placed_piece_flow {
                assert!(placed_piece_flow.can_stack_all(move_rules, spawn));
                assert!(!placed_piece_flow.can_stack_all_strictly(move_rules, spawn));
            }

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable_strictly(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_none());
        }
        {
            let move_rules = MoveRules::srs(AllowMove::Harddrop);
            let spawn = bl(4, 20);

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_none());

            let placed_piece_flow = PlacedPieceBlocksFlow::find_one_stackable_strictly(board, placed_piece_blocks.iter().collect(), move_rules, spawn);
            assert!(placed_piece_flow.is_none());
        }
    }

    #[test]
    fn empty() {
        let placed_piece_flow = PlacedPieceBlocksFlow::new(Board64::blank(), vec![]);
        assert_eq!(placed_piece_flow.len(), 0);
        assert!(placed_piece_flow.can_place_all());
        assert!(placed_piece_flow.can_stack_all(MoveRules::default(), bl(4, 20)));
        assert!(placed_piece_flow.can_stack_all_strictly(MoveRules::default(), bl(4, 20)));
    }

    #[test]
    fn find_one_placeable_no_placeable_case() {
        let board = Board64::from_str("
            ......####
            ......####
            ......####
            ......####
        ").unwrap();
        let placed_piece_blocks = vec![
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(SN), 3, array_vec![0, 1])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(IN), 0, array_vec![1])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(TN), 0, array_vec![2, 3])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(ZN), 2, array_vec![2, 3])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(LW), 4, array_vec![0, 2, 3])),
            PlacedPieceBlocks::make(PlacedPiece::new(piece!(JN), 0, array_vec![0, 3])),
        ];

        let placed_piece_flow = PlacedPieceBlocksFlow::new(board, placed_piece_blocks.iter().collect());
        assert_eq!(placed_piece_flow.len(), 6);
        assert!(!placed_piece_flow.can_place_all());

        assert_eq!(PlacedPieceBlocksFlow::find_one_placeable(board, placed_piece_blocks.iter().collect()), None);
    }
}
