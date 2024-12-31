use crate::array_map::{map_indexed4, ref_zip2_map4, zip2_map4};
use crate::boards::{Board, Board16, BoardOp};
use crate::coordinates::cc;
use crate::internal_moves::avx2::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::loaders::{
    can_reach1, can_reach4, spawn_and_harddrop_reachable, spawn_and_harddrop_reachables,
    to_bytes_u32, to_bytes_u32x4, to_free_space_lower, to_free_space_upper, to_free_spaces_lower,
    to_free_spaces_upper,
};
use crate::internal_moves::avx2::minimize::minimize;
use crate::internal_moves::avx2::moves::{Moves1, Moves4};
use crate::internal_moves::avx2::reachable::ReachableSimd16;
use crate::internal_moves::avx2::rotate::{rotate_ccw, rotate_cw};
use crate::pieces::{Orientation, Piece};
use crate::placements::{BlPlacement, CcPlacement};
use crate::{Rotate, With};

const ORIENTATIONS_ORDER: [Orientation; 4] = [
    Orientation::North,
    Orientation::East,
    Orientation::South,
    Orientation::West,
];

pub(crate) fn search_with_rotation<const BOTTOM_CUT: bool>(
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

pub(crate) fn can_reach_with_rotation<const BOTTOM_CUT: bool>(
    spawn_piece: Piece,
    mut reachables: [ReachableSimd16; 4],
    free_spaces: &[FreeSpaceSimd16; 4],
    goals: &[CcPlacement],
) -> bool {
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

        if can_reach4(&reachables, goals) {
            return true;
        }

        current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
    }

    false
}

pub(crate) fn search_no_rotation(
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

pub(crate) fn can_reach_no_rotation(
    mut reachable: ReachableSimd16,
    free_space: &FreeSpaceSimd16,
    goal: CcPlacement,
) -> bool {
    loop {
        let new_reachable = reachable.clone();
        let new_reachable = new_reachable.move1(free_space);

        if reachable == new_reachable {
            break;
        }

        if can_reach1(&new_reachable, goal) {
            return true;
        }

        reachable = new_reachable;
    }

    false
}

pub fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves4 {
    let spawn = spawn.to_cc_placement();

    // board.well_top() == ~12~ -> 11
    //  > free_spaceの作成も16bitで可能
    //  > y==11の地面に立つI-East(cy=14)もfree_spaceで集約できるため
    //  > I-Westでcy=13は集約可能、c=14は集約不可なので、上2行は無条件でreachableとする必要がある
    //  > かつspawn.cyは14<=なら問題ない。I-Eastでも地面に引っかからず横移動可能
    //   （spawnの方向は関係なく14が必要。まず全方向に回転して移動することを考えるため。ミノの種類ごとに整理しても良いかも？）
    //  > cy=13ならどうする？kickの移動で上部にはみ出ることはない？ Iの回転でy+2する回転があるため

    // 結論: well_top<=11なら1つだけ利用する
    //      12<=well_topなら2回チェックしてORをとれば境界もカバーできそう。overlapは4でカバーできそう（厳密には3かもだけど4の方が雰囲気良さそう）
    //           y >> 12してチェックする
    // 　　　つまりサポートは高さ28まで

    // 実験メモ: spawnだけ記録してあとはすべてまかすパターンを実装する。その後、spawnをうまくできないか考える
    //          well_top<=11で、14<=spawn.cyならspawnの記録は不可なので別途考える（画面上部はfreeが欠けている可能性があるため盤内でも注意が必要）
    //               13<=spawn.cyなら13<=cyをすべてreachableにしてOK

    if board.well_top() <= 11 {
        let free_spaces = to_free_spaces_lower(board, spawn.piece.shape);

        let spawn1 = if spawn.position.cy < 13 {
            spawn
        } else {
            spawn.piece.with(cc(spawn.position.cx, 13))
        };

        let reachables = spawn_and_harddrop_reachables(spawn1, &free_spaces);
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
            spawn,
            reachables: reachables.map(|reachable| reachable.to_bytes_u32()),
        }
    } else {
        let mut free_spaces_x2 = [
            to_free_spaces_lower(board, spawn.piece.shape),
            to_free_spaces_upper(board, spawn.piece.shape),
        ];
        free_spaces_x2[0] = map_indexed4(free_spaces_x2[0].clone(), |index, reachable| {
            reachable.or_shift::<0, 0, 0, 12>(&free_spaces_x2[1][index])
        });
        free_spaces_x2[1] = map_indexed4(free_spaces_x2[1].clone(), |index, reachable| {
            reachable.or_shift::<0, 0, 12, 0>(&free_spaces_x2[0][index])
        });

        let mut reachables_x2 = [
            spawn_and_harddrop_reachables(spawn, &free_spaces_x2[0]),
            spawn_and_harddrop_reachables(
                spawn
                    .piece
                    .with(cc(spawn.position.cx, spawn.position.cy - 12)),
                &free_spaces_x2[1],
            ),
        ];

        // println!("free_spaces_upper: {}", Board16::from(&free_spaces_x2[1][0]));
        // println!("free_spaces_lower: {}", Board16::from(&free_spaces_x2[0][0]));
        // println!("free_spaces_upper: {}", Board16::from(&free_spaces_x2[1][1]));
        // println!("free_spaces_lower: {}", Board16::from(&free_spaces_x2[0][1]));
        // println!("free_spaces_upper: {}", Board16::from(&free_spaces_x2[1][2]));
        // println!("free_spaces_lower: {}", Board16::from(&free_spaces_x2[0][2]));
        // println!("free_spaces_upper: {}", Board16::from(&free_spaces_x2[1][3]));
        // println!("free_spaces_lower: {}", Board16::from(&free_spaces_x2[0][3]));
        //
        // println!("spawn = {}", spawn);
        // println!("reachable_upper: {}", Board16::from(&reachables_x2[1][0]));
        // println!("reachable_lower: {}", Board16::from(&reachables_x2[0][0]));
        // println!("reachable_upper: {}", Board16::from(&reachables_x2[1][1]));
        // println!("reachable_lower: {}", Board16::from(&reachables_x2[0][1]));
        // println!("reachable_upper: {}", Board16::from(&reachables_x2[1][2]));
        // println!("reachable_lower: {}", Board16::from(&reachables_x2[0][2]));
        // println!("reachable_upper: {}", Board16::from(&reachables_x2[1][3]));
        // println!("reachable_lower: {}", Board16::from(&reachables_x2[0][3]));

        let mut index = 1;
        let mut needs = [
            *&reachables_x2[0].iter().any(|r| !r.empty()),
            *&reachables_x2[1].iter().any(|r| !r.empty()),
        ];
        while needs[0] || needs[1] {
            if !needs[index] {
                index = (index + 1) % 2;
                continue;
            }
            needs[index] = false;

            if index == 0 {
                // lower
                // println!("lower");
                let mut reachables0 = [
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                ];
                std::mem::swap(&mut reachables_x2[0], &mut reachables0);

                reachables_x2[0] =
                    search_with_rotation::<false>(spawn.piece, reachables0, &free_spaces_x2[0]);

                let reachables1 = reachables_x2[1].clone();
                let indexed4 = map_indexed4(reachables1, |index, reachable| {
                    reachable.or_shift::<0, 0, 12, 0>(&reachables_x2[0][index])
                });
                if reachables_x2[1] != indexed4 {
                    reachables_x2[1] = indexed4;
                    needs[1] = true;
                }
            } else {
                // upper
                // println!("upper");
                let mut reachables1 = [
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                    ReachableSimd16::blank(),
                ];
                std::mem::swap(&mut reachables_x2[1], &mut reachables1);

                reachables_x2[1] =
                    search_with_rotation::<true>(spawn.piece, reachables1, &free_spaces_x2[1]);

                let reachables0 = reachables_x2[0].clone();
                let indexed4 = map_indexed4(reachables0, |index, reachable| {
                    reachable.or_shift::<0, 0, 0, 12>(&reachables_x2[1][index])
                });
                if reachables_x2[0] != indexed4 {
                    reachables_x2[0] = indexed4;
                    needs[0] = true;
                }
            }

            index = (index + 1) % 2;
        }

        // landed
        let reachables_x2 = [
            ref_zip2_map4(
                &reachables_x2[0],
                &free_spaces_x2[0],
                |reachable, free_space| reachable.clone().land(&free_space),
            ),
            ref_zip2_map4(
                &reachables_x2[1],
                &free_spaces_x2[1],
                |reachable, free_space| reachable.clone().clip(0xFFFE).land(&free_space),
            ),
        ];

        let reachables_x2 = if MINIMIZE {
            [
                minimize(reachables_x2[0].clone(), spawn.piece.shape),
                minimize(reachables_x2[1].clone(), spawn.piece.shape),
            ]
        } else {
            reachables_x2
        };

        Moves4 {
            spawn,
            reachables: to_bytes_u32x4(&reachables_x2[0], &reachables_x2[1]),
        }
    }
}

pub fn moves_softdrop_no_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves1 {
    let spawn = spawn.canonical_or_self().to_cc_placement();

    if board.well_top() <= 11 || spawn.position.cy <= 13 {
        let free_space = to_free_space_lower(board, spawn.piece);

        let spawn1 = if spawn.position.cy < 13 {
            spawn
        } else {
            spawn.piece.with(cc(spawn.position.cx, 13))
        };

        let reachable = spawn_and_harddrop_reachable(spawn1, &free_space);
        let reachable = search_no_rotation(reachable, &free_space);

        // landed
        let reachable = reachable.land(&free_space);

        Moves1 {
            spawn,
            reachable: reachable.to_bytes_u32(),
            minimized: MINIMIZE,
        }
    } else {
        let mut free_space_upper = to_free_space_upper(board, spawn.piece);
        let mut free_space_lower = to_free_space_lower(board, spawn.piece);

        free_space_upper = free_space_upper.or_shift::<0, 0, 12, 0>(&free_space_lower);
        free_space_lower = free_space_lower.or_shift::<0, 0, 0, 12>(&free_space_upper);

        let reachable_upper = spawn_and_harddrop_reachable(
            spawn
                .piece
                .with(cc(spawn.position.cx, spawn.position.cy - 12)),
            &free_space_upper,
        );
        let reachable_upper = search_no_rotation(reachable_upper, &free_space_upper);

        let reachable_lower = ReachableSimd16::blank().or_shift::<0, 0, 0, 12>(&reachable_upper);

        let reachable_lower = if !reachable_lower.empty() {
            search_no_rotation(reachable_lower, &free_space_lower)
        } else {
            reachable_lower
        };

        // landed
        let reachable_lower = reachable_lower.land(&free_space_lower);
        let reachable_upper = reachable_upper.clip(0xFFFE).land(&free_space_upper);

        Moves1 {
            spawn,
            reachable: to_bytes_u32(&reachable_lower, &reachable_upper),
            minimized: MINIMIZE,
        }
    }
}

// pub(crate) fn can_reach_softdrop_with_rotation(
//     rotation_system: &impl RotationSystem,
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
//
//     let free_spaces = to_free_spaces(board, spawn.piece.shape);
//     let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);
//
//     if can_reach4(&reachables, &goals) {
//         return true;
//     }
//
//     can_reach_with_rotation(rotation_system, spawn, reachables, &free_spaces, &goals)
// }
//
// pub(crate) fn can_reach_strictly_softdrop_with_rotation(
//     rotation_system: &impl RotationSystem,
//     goal: BlPlacement,
//     board: &Board<u64>,
//     spawn: BlPlacement,
// ) -> bool {
//     let spawn = spawn.to_cc_placement();
//     let goals = vec![goal.to_cc_placement()];
//
//     let free_spaces = to_free_spaces(board, spawn.piece.shape);
//     let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);
//
//     if can_reach4(&reachables, &goals) {
//         return true;
//     }
//
//     can_reach_with_rotation(rotation_system, spawn, reachables, &free_spaces, &goals)
// }
//
// pub(crate) fn can_reach_softdrop_no_rotation(
//     goal: BlPlacement,
//     board: &Board<u64>,
//     spawn: BlPlacement,
// ) -> bool {
//     let spawn = spawn.canonical_or_self().to_cc_placement();
//     let goal = goal.to_cc_placement();
//
//     let free_space = to_free_space(board, spawn.piece);
//     let reachable = spawn_and_harddrop_reachable(spawn, &free_space);
//
//     if can_reach1(&reachable, goal) {
//         return true;
//     }
//
//     can_reach_no_rotation(reachable, &free_space, goal)
// }
//
// pub(crate) fn can_reach_strictly_softdrop_no_rotation(
//     goal: BlPlacement,
//     board: &Board<u64>,
//     spawn: BlPlacement,
// ) -> bool {
//     can_reach_softdrop_no_rotation(goal, board, spawn)
// }
