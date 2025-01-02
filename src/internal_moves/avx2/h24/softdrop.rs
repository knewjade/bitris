use crate::array_map::zip2_map4;
use crate::boards::{Board, Board32, BoardOp};
use crate::coordinates::cc;
use crate::internal_moves::avx2::h24::free_space::FreeSpaceSimd24;
use crate::internal_moves::avx2::h24::loaders::*;
use crate::internal_moves::avx2::h24::minimize::minimize;
use crate::internal_moves::avx2::h24::reachable::ReachableSimd24;
use crate::internal_moves::avx2::h24::rotate::{rotate_ccw, rotate_cw};
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
    debug_assert!(board.well_top() <= 20);
    // TODO if board.well_top() <= 19 {でfree_spacesの作り方を変えたい

    let free_space_block = to_free_space_block(board);
    let free_spaces = to_free_spaces(&free_space_block, spawn.piece.shape);

    // スポーン位置を下のボードまでスキップする。
    // ボードで最も高いブロックの位置がy=19以下であるため、
    // `I-East (cy=21)` が左右移動しても引っかからない。
    // そのため、cy=21まで無条件でしても、その後の左右移動・ハードドロップに影響しない。
    let spawn = if spawn.position.cy < 21 {
        spawn
    } else {
        spawn.piece.with(cc(spawn.position.cx, 21))
    };

    let reachables = spawn_and_harddrop_reachables(spawn, &free_space_block, &free_spaces);
    let reachables = search_with_rotation(spawn.piece, reachables, &free_spaces);

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

pub fn search_with_rotation(
    spawn_piece: Piece,
    mut reachables: [ReachableSimd24; 4],
    free_spaces: &[FreeSpaceSimd24; 4],
) -> [ReachableSimd24; 4] {
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

        let mask = 0x3FFFFFu32;
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
    debug_assert!(board.well_top() <= 20);
    // TODO if board.well_top() <= 19 {でfree_spacesの作り方を変えたい

    let free_space_block = to_free_space_block(board);
    let free_space = to_free_space(&free_space_block, spawn.piece);

    let spawn = if spawn.position.cy < 21 {
        spawn
    } else {
        spawn.piece.with(cc(spawn.position.cx, 21))
    };

    let reachable = spawn_and_harddrop_reachable(spawn, &free_space_block, &free_space);
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
    mut reachable: ReachableSimd24,
    free_space: &FreeSpaceSimd24,
) -> ReachableSimd24 {
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
