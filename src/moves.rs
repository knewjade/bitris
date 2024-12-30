use crate::boards::Board64;
use crate::internal_macros::enum_display;
use crate::internal_moves::u64::{harddrop, softdrop};
use crate::placements::BlPlacement;
use crate::srs::SrsKickTable;
use crate::RotationSystem;

/// A collection of piece drop types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum AllowMove {
    #[default]
    Softdrop,
    Harddrop,
}

/// Rules to be applied during move generation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MoveRules<'a, T>
where
    T: RotationSystem,
{
    pub rotation_system: &'a T,
    pub allow_move: AllowMove,
}

impl MoveRules<'_, SrsKickTable> {
    #[inline]
    pub fn default() -> Self {
        Self::srs(AllowMove::Softdrop)
    }

    #[inline]
    pub fn srs(allow_move: AllowMove) -> Self {
        Self {
            rotation_system: &SrsKickTable,
            allow_move,
        }
    }
}

impl<'a, T> MoveRules<'a, T>
where
    T: RotationSystem,
{
    #[inline]
    pub fn new(rotation_system: &'a T, allow_move: AllowMove) -> Self {
        Self {
            rotation_system,
            allow_move,
        }
    }

    /// Collect all the placements that can be placed in the rotation system.
    /// If the placements have the same block positions, but the orientations are different, each will be collected.
    ///
    /// Panics if the spawn is not placeable position.
    #[inline]
    pub fn generate_all_moves(&self, board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
        let is_moving_in_rotation = self
            .rotation_system
            .is_moving_in_rotation(spawn.piece.shape);
        match self.allow_move {
            AllowMove::Softdrop => {
                if is_moving_in_rotation {
                    softdrop::moves_softdrop_with_rotation::<false>(
                        self.rotation_system,
                        &board,
                        spawn,
                    )
                    .vec()
                } else {
                    softdrop::moves_softdrop_no_rotation::<false>(&board, spawn).vec()
                }
            }
            AllowMove::Harddrop => {
                if is_moving_in_rotation {
                    harddrop::moves_harddrop_with_rotation::<false>(
                        self.rotation_system,
                        &board,
                        spawn,
                    )
                    .vec()
                } else {
                    harddrop::moves_harddrop_no_rotation::<false>(&board, spawn).vec()
                }
            }
        }
    }

    /// Collect all the placements that can be placed in the rotation system.
    /// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
    /// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
    ///
    /// Panics if the spawn is not placeable position.
    #[inline]
    pub fn generate_minimized_moves(&self, board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
        let is_moving_in_rotation = self
            .rotation_system
            .is_moving_in_rotation(spawn.piece.shape);
        match self.allow_move {
            AllowMove::Softdrop => {
                if is_moving_in_rotation {
                    softdrop::moves_softdrop_with_rotation::<true>(
                        self.rotation_system,
                        &board,
                        spawn,
                    )
                    .vec()
                } else {
                    softdrop::moves_softdrop_no_rotation::<true>(&board, spawn).vec()
                }
            }
            AllowMove::Harddrop => {
                if is_moving_in_rotation {
                    harddrop::moves_harddrop_with_rotation::<true>(
                        self.rotation_system,
                        &board,
                        spawn,
                    )
                    .vec()
                } else {
                    harddrop::moves_harddrop_no_rotation::<true>(&board, spawn).vec()
                }
            }
        }
    }

    /// Return true when the piece can be carried to the placement.
    ///
    /// Note that the same form will succeed regardless of the orientation.
    /// If you want to be strict, use `can_reach_strictly()`.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::piece;
    /// use bitris::prelude::*;
    ///
    /// use Shape::*;
    /// use Orientation::*;
    ///
    /// let board = Board64::from_str("
    ///     .##..#####
    ///     .#..######
    /// ").unwrap();
    ///
    /// let srs_softdrop = MoveRules::srs(AllowMove::Softdrop);
    /// let srs_harddrop = MoveRules::srs(AllowMove::Harddrop);
    /// let spawn = S.with(North).with(bl(4, 20));
    ///
    /// // S-South is reachable using softdrop.
    /// assert!(srs_softdrop.can_reach(S.with(South).with(bl(2, 0)), board, spawn));
    /// assert!(srs_softdrop.can_reach_strictly(S.with(South).with(bl(2, 0)), board, spawn));
    /// assert!(!srs_harddrop.can_reach(S.with(South).with(bl(2, 0)), board, spawn));
    ///
    /// // S-North is unreachable in srs, but success because the south is reachable.
    /// assert!(srs_softdrop.can_reach(S.with(North).with(bl(2, 0)), board, spawn));
    /// assert!(!srs_softdrop.can_reach_strictly(S.with(North).with(bl(2, 0)), board, spawn));
    /// assert!(!srs_harddrop.can_reach(S.with(North).with(bl(2, 0)), board, spawn));
    /// ```
    pub fn can_reach(&self, goal: BlPlacement, board: Board64, spawn: BlPlacement) -> bool {
        assert_eq!(goal.piece.shape, spawn.piece.shape);

        if !goal.can_place_on(&board) {
            return false;
        }

        let is_moving_in_rotation = self
            .rotation_system
            .is_moving_in_rotation(spawn.piece.shape);
        match self.allow_move {
            AllowMove::Softdrop => {
                if is_moving_in_rotation {
                    softdrop::can_reach_softdrop_with_rotation(
                        self.rotation_system,
                        goal,
                        &board,
                        spawn,
                    )
                } else {
                    softdrop::can_reach_softdrop_no_rotation(goal, &board, spawn)
                }
            }
            AllowMove::Harddrop => {
                if is_moving_in_rotation {
                    harddrop::can_reach_harddrop_with_rotation(
                        self.rotation_system,
                        goal,
                        &board,
                        spawn,
                    )
                } else {
                    harddrop::can_reach_harddrop_no_rotation(goal, &board, spawn)
                }
            }
        }
    }

    /// It's similar to `can_reach()` except that the orientation is strictly checked.
    pub fn can_reach_strictly(
        &self,
        goal: BlPlacement,
        board: Board64,
        spawn: BlPlacement,
    ) -> bool {
        assert_eq!(goal.piece.shape, spawn.piece.shape);

        if !goal.can_place_on(&board) {
            return false;
        }

        let is_moving_in_rotation = self
            .rotation_system
            .is_moving_in_rotation(spawn.piece.shape);
        match self.allow_move {
            AllowMove::Softdrop => {
                if is_moving_in_rotation {
                    softdrop::can_reach_strictly_softdrop_with_rotation(
                        self.rotation_system,
                        goal,
                        &board,
                        spawn,
                    )
                } else {
                    softdrop::can_reach_strictly_softdrop_no_rotation(goal, &board, spawn)
                }
            }
            AllowMove::Harddrop => {
                if is_moving_in_rotation {
                    harddrop::can_reach_strictly_harddrop_with_rotation(
                        self.rotation_system,
                        goal,
                        &board,
                        spawn,
                    )
                } else {
                    harddrop::can_reach_strictly_harddrop_no_rotation(goal, &board, spawn)
                }
            }
        }
    }
}

enum_display! { AllowMove, has Softdrop,Harddrop }

pub mod srs {
    use std::slice::Iter;

    use crate::boards::Board64;
    use crate::coordinates::Offset;
    use crate::pieces::{Piece, Shape};
    use crate::placements::BlPlacement;
    use crate::{AllowMove, Kick, MoveRules, Rotation, RotationSystem};

    macro_rules! k {
        ($dx: expr, $dy: expr) => {
            Kick::new(Offset::new($dx, $dy))
        };
    }

    /// Kick table with SRS defined.
    #[derive(Copy, Clone, Hash, Debug, Default)]
    pub struct SrsKickTable;

    impl SrsKickTable {
        const LJSZ_NE: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, 1), k!(0, -2), k!(-1, -2)];
        const LJSZ_ES: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, -1), k!(0, 2), k!(1, 2)];
        const LJSZ_SW: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, 1), k!(0, -2), k!(1, -2)];
        const LJSZ_WN: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, -1), k!(0, 2), k!(-1, 2)];

        const LJSZ_NW: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, 1), k!(0, -2), k!(1, -2)];
        const LJSZ_WS: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, -1), k!(0, 2), k!(-1, 2)];
        const LJSZ_SE: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, 1), k!(0, -2), k!(-1, -2)];
        const LJSZ_EN: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, -1), k!(0, 2), k!(1, 2)];

        const T_NE: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, 1), k!(0, -2), k!(-1, -2)];
        const T_ES: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, -1), k!(0, 2), k!(1, 2)];
        const T_SW: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, 1), k!(0, -2), k!(1, -2)];
        const T_WN: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, -1), k!(0, 2), k!(-1, 2)];

        const T_NW: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, 1), k!(0, -2), k!(1, -2)];
        const T_WS: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, -1), k!(0, 2), k!(-1, 2)];
        const T_SE: [Kick; 5] = [k!(0, 0), k!(-1, 0), k!(-1, 1), k!(0, -2), k!(-1, -2)];
        const T_EN: [Kick; 5] = [k!(0, 0), k!(1, 0), k!(1, -1), k!(0, 2), k!(1, 2)];

        const I_NE: [Kick; 5] = [k!(1, 0), k!(-1, 0), k!(2, 0), k!(-1, -1), k!(2, 2)];
        const I_ES: [Kick; 5] = [k!(0, -1), k!(-1, -1), k!(2, -1), k!(-1, 1), k!(2, -2)];
        const I_SW: [Kick; 5] = [k!(-1, 0), k!(1, 0), k!(-2, 0), k!(1, 1), k!(-2, -2)];
        const I_WN: [Kick; 5] = [k!(0, 1), k!(1, 1), k!(-2, 1), k!(1, -1), k!(-2, 2)];

        const I_NW: [Kick; 5] = [k!(0, -1), k!(-1, -1), k!(2, -1), k!(-1, 1), k!(2, -2)];
        const I_WS: [Kick; 5] = [k!(1, 0), k!(-1, 0), k!(2, 0), k!(-1, -1), k!(2, 2)];
        const I_SE: [Kick; 5] = [k!(0, 1), k!(1, 1), k!(-2, 1), k!(1, -1), k!(-2, 2)];
        const I_EN: [Kick; 5] = [k!(-1, 0), k!(1, 0), k!(-2, 0), k!(1, 1), k!(-2, -2)];

        const O_NE: [Kick; 1] = [k!(0, 1)];
        const O_ES: [Kick; 1] = [k!(1, 0)];
        const O_SW: [Kick; 1] = [k!(0, -1)];
        const O_WN: [Kick; 1] = [k!(-1, 0)];

        const O_NW: [Kick; 1] = [k!(1, 0)];
        const O_WS: [Kick; 1] = [k!(0, 1)];
        const O_SE: [Kick; 1] = [k!(-1, 0)];
        const O_EN: [Kick; 1] = [k!(0, -1)];

        const EMPTY: [Kick; 0] = [];

        const LJSZ_KICKS: [&'static [Kick]; 12] = [
            // from North
            &Self::LJSZ_NE,
            &Self::LJSZ_NW,
            &Self::EMPTY,
            // from East
            &Self::LJSZ_ES,
            &Self::LJSZ_EN,
            &Self::EMPTY,
            // from South
            &Self::LJSZ_SW,
            &Self::LJSZ_SE,
            &Self::EMPTY,
            // from West
            &Self::LJSZ_WN,
            &Self::LJSZ_WS,
            &Self::EMPTY,
        ];
        const T_KICKS: [&'static [Kick]; 12] = [
            // from North
            &Self::T_NE,
            &Self::T_NW,
            &Self::EMPTY,
            // from East
            &Self::T_ES,
            &Self::T_EN,
            &Self::EMPTY,
            // from South
            &Self::T_SW,
            &Self::T_SE,
            &Self::EMPTY,
            // from West
            &Self::T_WN,
            &Self::T_WS,
            &Self::EMPTY,
        ];
        const I_KICKS: [&'static [Kick]; 12] = [
            // from North
            &Self::I_NE,
            &Self::I_NW,
            &Self::EMPTY,
            // from East
            &Self::I_ES,
            &Self::I_EN,
            &Self::EMPTY,
            // from South
            &Self::I_SW,
            &Self::I_SE,
            &Self::EMPTY,
            // from West
            &Self::I_WN,
            &Self::I_WS,
            &Self::EMPTY,
        ];
        const O_KICKS: [&'static [Kick]; 12] = [
            // from North
            &Self::O_NE,
            &Self::O_NW,
            &Self::EMPTY,
            // from East
            &Self::O_ES,
            &Self::O_EN,
            &Self::EMPTY,
            // from South
            &Self::O_SW,
            &Self::O_SE,
            &Self::EMPTY,
            // from West
            &Self::O_WN,
            &Self::O_WS,
            &Self::EMPTY,
        ];
    }

    impl RotationSystem for SrsKickTable {
        fn iter_kicks(&self, piece: Piece, rotation: Rotation) -> Iter<'_, Kick> {
            assert_ne!(
                rotation,
                Rotation::R180,
                "This kick table does not support 180 rotation."
            );
            let index = piece.orientation as usize * 3 + rotation as usize;
            match piece.shape {
                Shape::L | Shape::J | Shape::S | Shape::Z => Self::LJSZ_KICKS[index].iter(),
                Shape::T => Self::T_KICKS[index].iter(),
                Shape::I => Self::I_KICKS[index].iter(),
                Shape::O => Self::O_KICKS[index].iter(),
            }
        }

        fn is_moving_in_rotation(&self, shape: Shape) -> bool {
            shape != Shape::O
        }
    }

    /// Collect all the places that can be placed in srs.
    /// If the placements have the same block positions, but the orientations are different, each will be collected.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str(" \
    ///     ..........\
    ///     ..........\
    ///     ..........\
    ///     ..........\
    /// ").unwrap();
    /// let placement = piece!(TN).with(bl(4, 20));
    /// let moves = srs::generate_all_moves(AllowMove::Softdrop, board, placement);
    /// assert_eq!(moves.len(), 34);
    /// ```
    pub fn generate_all_moves(
        allow_move: AllowMove,
        board: Board64,
        spawn: BlPlacement,
    ) -> Vec<BlPlacement> {
        let move_rules = MoveRules::srs(allow_move);
        move_rules.generate_all_moves(board, spawn)
    }

    /// Collect all the places that can be placed in srs.
    /// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
    /// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
    /// ```
    /// use std::str::FromStr;
    /// use bitris::macros::piece;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str(" \
    ///             ..........\
    ///             ..........\
    ///             ..........\
    ///             ..........\
    ///         ").unwrap();
    /// let placement = piece!(ON).with(bl(4, 20));
    /// let moves = srs::generate_minimized_moves(AllowMove::Softdrop, board, placement);
    /// assert_eq!(moves.len(), 9);
    /// ```
    pub fn generate_minimized_moves(
        allow_move: AllowMove,
        board: Board64,
        spawn: BlPlacement,
    ) -> Vec<BlPlacement> {
        let move_rules = MoveRules::srs(allow_move);
        move_rules.generate_minimized_moves(board, spawn)
    }

    /// Return true when the piece can be carried to the placement in srs.
    ///
    /// Note that the same form will succeed regardless of the orientation.
    /// If you want to be strict, use `can_reach_strictly()`.
    #[inline]
    pub fn can_reach(
        allow_move: AllowMove,
        goal: BlPlacement,
        board: Board64,
        spawn: BlPlacement,
    ) -> bool {
        let move_rules = MoveRules::srs(allow_move);
        move_rules.can_reach(goal, board, spawn)
    }

    /// It's similar to `can_reach()` except that the orientation is strictly checked.
    #[inline]
    pub fn can_reach_strictly(
        allow_move: AllowMove,
        goal: BlPlacement,
        board: Board64,
        spawn: BlPlacement,
    ) -> bool {
        let move_rules = MoveRules::srs(allow_move);
        move_rules.can_reach_strictly(goal, board, spawn)
    }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;

        use itertools::assert_equal;

        use crate::piece;
        use crate::prelude::*;
        use crate::srs::*;

        #[test]
        fn srs_t_from_north_to_east() {
            let kicks =
                SrsKickTable.iter_kicks(Piece::new(Shape::T, Orientation::North), Rotation::Cw);
            assert_equal(
                kicks.map(|it| it.offset),
                vec![dd(0, 0), dd(-1, 0), dd(-1, 1), dd(0, -2), dd(-1, -2)],
            );
        }

        #[test]
        #[should_panic]
        fn srs_is_not_unsupported_rotate_180() {
            let _ =
                SrsKickTable.iter_kicks(Piece::new(Shape::T, Orientation::North), Rotation::R180);
        }

        #[test]
        fn test_kick() {
            let board = Board64::from_str(
                "\
            XXX.......\
            XX........\
            XX.XXXXXXX\
            XX..XXXXXX\
            XX.XXXXXXX\
        ",
            )
            .unwrap();
            let placement = piece!(TN).with(cc(3, 3));
            let rotation = Rotation::Cw;
            assert_eq!(
                SrsKickTable.test_kick(&board, placement, rotation),
                Some(TestKickResult {
                    test_index: 4,
                    kick: *SrsKickTable
                        .iter_kicks(placement.piece, rotation)
                        .nth(4)
                        .unwrap(),
                    destination: CcPlacement {
                        piece: piece!(TE),
                        position: cc(2, 1)
                    },
                })
            );

            let board = Board64::from_str(
                "\
            XX..XXXXXX\
            XXX..XXXXX\
        ",
            )
            .unwrap();
            let placement = piece!(ZW).with(cc(4, 2));
            let rotation = Rotation::Ccw;
            assert_eq!(
                SrsKickTable.test_kick(&board, placement, rotation),
                Some(TestKickResult {
                    test_index: 2,
                    kick: *SrsKickTable
                        .iter_kicks(placement.piece, rotation)
                        .nth(2)
                        .unwrap(),
                    destination: CcPlacement {
                        piece: piece!(ZS),
                        position: cc(3, 1)
                    },
                })
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::piece;
    use crate::prelude::*;

    #[test]
    fn generate_all_moves() {
        let board = Board64::from_str(
            " \
            ..XXXXXX..\
            ..........\
            ..........\
            ..........\
        ",
        )
        .unwrap();
        let rules = MoveRules::srs(AllowMove::Harddrop);
        let placement = piece!(SN).with(bl(4, 20));
        let moves = rules.generate_all_moves(board, placement);
        assert_eq!(moves.len(), 34);
        assert_eq!(moves.iter().filter(|it| it.position.by == 0).count(), 4);
    }

    #[test]
    fn generate_minimized_moves() {
        let board = Board64::from_str(
            " \
            ..XXXXXX..\
            ..........\
            ..........\
            ..........\
        ",
        )
        .unwrap();
        let rules = MoveRules::srs(AllowMove::Harddrop);
        let placement = piece!(SN).with(bl(4, 20));
        let moves = rules.generate_minimized_moves(board, placement);
        assert_eq!(moves.len(), 17);
        assert_eq!(moves.iter().filter(|it| it.position.by == 0).count(), 2);
    }
}
