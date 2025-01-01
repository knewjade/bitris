use crate::array_map::zip2_map4;
use crate::boards::{Board, BoardOp};
use crate::coordinates::cc;
use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::h16::loaders::*;
use crate::internal_moves::avx2::h16::minimize::minimize;
use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;
use crate::internal_moves::avx2::h16::rotate::{rotate_ccw, rotate_cw};
use crate::internal_moves::avx2::moves::{Moves1, Moves4};
use crate::pieces::{Orientation, Piece};
use crate::placements::CcPlacement;
use crate::{Rotate, With};

const ORIENTATIONS_ORDER: [Orientation; 4] = [
    Orientation::North,
    Orientation::East,
    Orientation::South,
    Orientation::West,
];

#[inline(always)]
pub fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves4 {
    debug_assert!(board.well_top() <= 11);
    // TODO if board.well_top() <= 11 {でfree_spacesの作り方を変えたい

    let free_spaces = to_free_spaces_lower(board, spawn.piece.shape);

    // スポーン位置を下のボードまでスキップする。
    // ボードで最も高いブロックの位置がy=10以下であるため、
    // `I-East (cy=13)` が左右移動しても引っかからない。
    // そのため、cy=13まで無条件でしても、その後の左右移動・ハードドロップに影響しない。
    let spawn = if spawn.position.cy < 13 {
        spawn
    } else {
        spawn.piece.with(cc(spawn.position.cx, 13))
    };

    let reachables = spawn_and_harddrop_reachables(spawn, &free_spaces);
    let reachables = search_with_rotation::<false>(spawn.piece, reachables, &free_spaces);

    // landed
    let reachables = zip2_map4(reachables, free_spaces, |reachable, free_space| {
        reachable.land(&free_space)
    });

    let reachables = if MINIMIZE {
        minimize(reachables, spawn.piece.shape)
    } else {
        reachables
    };

    Moves4 {
        spawn_piece: spawn.piece,
        reachables: reachables.map(|reachable| reachable.to_bytes_u32()),
    }
}

pub fn search_with_rotation<const BOTTOM_CUT: bool>(
    spawn_piece: Piece,
    mut reachables: [ReachableSimd16; 4],
    free_spaces: &[FreeSpaceSimd16; 4],
) -> [ReachableSimd16; 4] {
    let mut needs_update: u8 = 0b1111;

    let mut current_index: usize = spawn_piece.orientation as usize;
    while needs_update != 0 {
        // if the current index is not updated, skip it.
        if needs_update & (1 << current_index) == 0 {
            current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
            continue;
        }
        needs_update -= 1 << current_index;

        // initialize
        let src_piece = Piece::new(spawn_piece.shape, ORIENTATIONS_ORDER[current_index]);
        let src_index = current_index;

        // move
        loop {
            let reachable = reachables[src_index].clone().move1(&free_spaces[src_index]);

            if reachables[src_index] == reachable {
                break;
            }
            reachables[src_index] = reachable;
        }

        let mask = if BOTTOM_CUT { 0x3FFCu16 } else { 0x3FFFu16 };
        let reachable_for_rotate = reachables[src_index].clone().clip(mask);

        if !reachable_for_rotate.empty() {
            // cw rotate
            {
                let dest_index = src_piece.cw().orientation as usize;

                let found_dest_reachable =
                    rotate_cw(src_piece, &reachable_for_rotate, &free_spaces[dest_index]);

                let dest_reachable = reachables[dest_index].clone().or(&found_dest_reachable);

                if reachables[dest_index] != dest_reachable {
                    reachables[dest_index] = dest_reachable;
                    needs_update |= 1 << dest_index;
                }
            }

            // ccw rotate
            {
                let dest_index = src_piece.ccw().orientation as usize;

                let found_dest_reachable =
                    rotate_ccw(src_piece, &reachable_for_rotate, &free_spaces[dest_index]);

                let dest_reachable = reachables[dest_index].clone().or(&found_dest_reachable);

                if reachables[dest_index] != dest_reachable {
                    reachables[dest_index] = dest_reachable;
                    needs_update |= 1 << dest_index;
                }
            }
        }

        current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
    }

    reachables
}

#[inline(always)]
pub fn moves_softdrop_no_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves1 {
    debug_assert!(board.well_top() <= 11 || spawn.position.cy <= 13);

    let free_space = to_free_space_lower(board, spawn.piece);

    let spawn = if spawn.position.cy < 13 {
        spawn
    } else {
        spawn.piece.with(cc(spawn.position.cx, 13))
    };

    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);
    let reachable = search_no_rotation(reachable, &free_space);

    // landed
    let reachable = reachable.land(&free_space);

    Moves1 {
        spawn_piece: spawn.piece,
        reachable: reachable.to_bytes_u32(),
        minimized: MINIMIZE,
    }
}

pub fn search_no_rotation(
    mut reachable: ReachableSimd16,
    free_space: &FreeSpaceSimd16,
) -> ReachableSimd16 {
    loop {
        let new_reachable = reachable.clone();
        let new_reachable = new_reachable.move1(free_space);

        if reachable == new_reachable {
            break;
        }
        reachable = new_reachable;
    }

    reachable
}

// pub(crate) fn can_reach_with_rotation<const BOTTOM_CUT: bool>(
//     spawn_piece: Piece,
//     mut reachables: [ReachableSimd16; 4],
//     free_spaces: &[FreeSpaceSimd16; 4],
//     goals: &[CcPlacement],
// ) -> bool {
//     let mut needs_update: u8 = 0b1111;
//
//     let mut current_index: usize = spawn_piece.orientation as usize;
//     while needs_update != 0 {
//         // if the current index is not updated, skip it.
//         if needs_update & (1 << current_index) == 0 {
//             current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
//             continue;
//         }
//         needs_update -= 1 << current_index;
//
//         // initialize
//         let src_piece = Piece::new(spawn_piece.shape, ORIENTATIONS_ORDER[current_index]);
//         let src_index = current_index;
//
//         // move
//         loop {
//             let reachable = reachables[src_index].clone().move1(&free_spaces[src_index]);
//
//             if reachables[src_index] == reachable {
//                 break;
//             }
//             reachables[src_index] = reachable;
//         }
//
//         let mask = if BOTTOM_CUT { 0x3FFCu16 } else { 0x3FFFu16 };
//         let reachable_for_rotate = reachables[src_index].clone().clip(mask);
//
//         if !reachable_for_rotate.empty() {
//             // cw rotate
//             {
//                 let dest_index = src_piece.cw().orientation as usize;
//
//                 let found_dest_reachable =
//                     rotate_cw(src_piece, &reachable_for_rotate, &free_spaces[dest_index]);
//
//                 let dest_reachable = reachables[dest_index].clone().or(&found_dest_reachable);
//
//                 if reachables[dest_index] != dest_reachable {
//                     reachables[dest_index] = dest_reachable;
//                     needs_update |= 1 << dest_index;
//                 }
//             }
//
//             // ccw rotate
//             {
//                 let dest_index = src_piece.ccw().orientation as usize;
//
//                 let found_dest_reachable =
//                     rotate_ccw(src_piece, &reachable_for_rotate, &free_spaces[dest_index]);
//
//                 let dest_reachable = reachables[dest_index].clone().or(&found_dest_reachable);
//
//                 if reachables[dest_index] != dest_reachable {
//                     reachables[dest_index] = dest_reachable;
//                     needs_update |= 1 << dest_index;
//                 }
//             }
//         }
//
//         if can_reach4(&reachables, goals) {
//             return true;
//         }
//
//         current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
//     }
//
//     false
// }
//

// pub(crate) fn can_reach_no_rotation(
//     mut reachable: ReachableSimd16,
//     free_space: &FreeSpaceSimd16,
//     goal: CcPlacement,
// ) -> bool {
//     loop {
//         let new_reachable = reachable.clone();
//         let new_reachable = new_reachable.move1(free_space);
//
//         if reachable == new_reachable {
//             break;
//         }
//
//         if can_reach1(&new_reachable, goal) {
//             return true;
//         }
//
//         reachable = new_reachable;
//     }
//
//     false
// }
//

// pub(crate) fn can_reach_softdrop_with_rotation(
//     goal: BlPlacement,
//     board: &Board<u64>,
//     spawn: BlPlacement,
// ) -> bool {
//     let spawn = spawn.to_cc_placement();
//     let goals = goal
//         .piece
//         .orientations_having_same_form()
//         .iter()
//         .map(|&orientation| goal.piece.shape.with(orientation))
//         .map(|piece| piece.with(goal.position.to_cc_position(piece.to_piece_blocks())))
//         .collect::<Vec<_>>();
//     can_reach_softdrop_with_rotation_2(board, spawn, goals)
// }
//
// pub(crate) fn can_reach_strictly_softdrop_with_rotation(
//     goal: BlPlacement,
//     board: &Board<u64>,
//     spawn: BlPlacement,
// ) -> bool {
//     let spawn = spawn.to_cc_placement();
//     let goals = vec![goal.to_cc_placement()];
//     can_reach_softdrop_with_rotation_2(board, spawn, goals)
// }
//
// fn can_reach_softdrop_with_rotation_2(
//     board: &Board<u64>,
//     spawn: CcPlacement,
//     goals: Vec<CcPlacement>,
// ) -> bool {
//     // let free_spaces = to_free_spaces(board, spawn.piece.shape);
//     // let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);
//     //
//     // if can_reach4(&reachables, &goals) {
//     //     return true;
//     // }
//     //
//     // can_reach_with_rotation(rotation_system, spawn, reachables, &free_spaces, &goals)
//
//     // 地面がy<=11であれば、その上に置くI-East(cy=14)もfree_spaceで集約できるため問題が起きない。
//     // y=12だと、I-Eastが横移動で通れなくなる。
//     if board.well_top() <= 11 {
//         can_reach_softdrop_with_rotation_lower_only(board, spawn, goals)
//     } else {
//         can_reach_softdrop_with_rotation_2boards(board, spawn, goals)
//     }
// }
//
// #[inline(always)]
// fn can_reach_softdrop_with_rotation_lower_only(
//     board: &Board<u64>,
//     spawn: CcPlacement,
//     goals: Vec<CcPlacement>,
// ) -> bool {
//     debug_assert!(board.well_top() <= 11);
//
//     let free_spaces = to_free_spaces_lower(board, spawn.piece.shape);
//
//     // スポーン位置を下のボードまでスキップする。
//     // ボードで最も高いブロックの位置がy=10以下であるため、
//     // `I-East (cy=13)` が左右移動しても引っかからない。
//     // そのため、cy=13までは無条件で到達可能。
//     if 13 <= spawn.position.cy {
//         return true
//     }
//
//     let reachables = spawn_and_harddrop_reachables(spawn, &free_spaces);
//
//     if can_reach4(&reachables, &goals) {
//         return true;
//     }
//
//     todo!()
//     // let reachables = search_with_rotation::<false>(spawn.piece, reachables, &free_spaces);
//     //
//     // Moves4 {
//     //     spawn_piece: spawn.piece,
//     //     reachables: reachables.map(|reachable| reachable.to_bytes_u32()),
//     // }
// }
//
// #[inline(always)]
// fn can_reach_softdrop_with_rotation_2boards(
//     board: &Board<u64>,
//     spawn: CcPlacement,
//     goals: Vec<CcPlacement>,
// ) -> bool {
//     let free_spaces_pair = to_free_spaces_pair(board, spawn);
//
//     let mut reachables_pair = spawn_and_harddrop_reachables_pair(spawn, &free_spaces_pair);
//
//     if can_reach4_pair(&reachables_pair, &goals) {
//         return true;
//     }
//
//     let mut current_index = 1;
//     let mut needs_update: u8 = 0b11;
//     if *&reachables_pair.lower.iter().all(|r| r.empty()) {
//         needs_update -= 0b01;
//     }
//     if *&reachables_pair.upper.iter().all(|r| r.empty()) {
//         needs_update -= 0b10;
//     }
//
//     while needs_update != 0 {
//         // if the current index is not updated, skip it.
//         if needs_update & (1 << current_index) == 0 {
//             current_index = (current_index + 1) % 2;
//             continue;
//         }
//         needs_update -= 1 << current_index;
//
//         if current_index == 0 {
//             // lower
//             let Pair {
//                 lower: reachables_lower,
//                 upper: reachables_upper,
//             } = reachables_pair.clone();
//
//             reachables_pair.lower = search_with_rotation::<false>(
//                 spawn.piece,
//                 reachables_lower,
//                 &free_spaces_pair.lower,
//             );
//
//             let reachables_upper = map_indexed4(reachables_upper, |index, reachable| {
//                 reachable.or_shift::<0, 0, 12, 0>(&reachables_pair.lower[index])
//             });
//             if reachables_pair.upper != reachables_upper {
//                 reachables_pair.upper = reachables_upper;
//                 needs_update |= 0b10;
//             }
//         } else {
//             // upper
//             let Pair {
//                 lower: reachables_lower,
//                 upper: reachables_upper,
//             } = reachables_pair.clone();
//
//             reachables_pair.upper = search_with_rotation::<true>(
//                 spawn.piece,
//                 reachables_upper,
//                 &free_spaces_pair.upper,
//             );
//
//             let reachables_lower = map_indexed4(reachables_lower, |index, reachable| {
//                 reachable.or_shift::<0, 0, 0, 12>(&reachables_pair.upper[index])
//             });
//             if reachables_pair.lower != reachables_lower {
//                 reachables_pair.lower = reachables_lower;
//                 needs_update |= 0b01;
//             }
//         }
//
//         if can_reach4_pair(&reachables_pair, &goals) {
//             return true;
//         }
//
//         current_index = (current_index + 1) % 2;
//     }
//
//     todo!()
// }
//
//
// // pub(crate) fn can_reach_softdrop_no_rotation(
// //     goal: BlPlacement,
// //     board: &Board<u64>,
// //     spawn: BlPlacement,
// // ) -> bool {
// //     let spawn = spawn.canonical_or_self().to_cc_placement();
// //     let goal = goal.to_cc_placement();
// //
// //     let free_space = to_free_space(board, spawn.piece);
// //     let reachable = spawn_and_harddrop_reachable(spawn, &free_space);
// //
// //     if can_reach1(&reachable, goal) {
// //         return true;
// //     }
// //
// //     can_reach_no_rotation(reachable, &free_space, goal)
// // }
// //
// // pub(crate) fn can_reach_strictly_softdrop_no_rotation(
// //     goal: BlPlacement,
// //     board: &Board<u64>,
// //     spawn: BlPlacement,
// // ) -> bool {
// //     can_reach_softdrop_no_rotation(goal, board, spawn)
// // }
