use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::internal_moves::u64::loaders::{rotate, to_free_space, to_free_spaces, spawn_and_harddrop_reachable, spawn_and_harddrop_reachables};
use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::{Orientation, Piece, Shape};
use crate::placements::{BlPlacement, CcPlacement};
use crate::{Rotate, Rotation, RotationSystem};
use crate::boards::Board;
use crate::internal_moves::u64::moves::{Moves1, Moves4};

pub(crate) fn search_with_rotation(
    rotation_system: &impl RotationSystem,
    spawn: CcPlacement,
    mut reachables: [Reachable64; 4],
    free_spaces: &[FreeSpace64; 4],
) -> [Reachable64; 4] {
    const ORIENTATIONS_ORDER: [Orientation; 4] = [
        Orientation::North,
        Orientation::East,
        Orientation::South,
        Orientation::West,
    ];

    let mut needs_update: u8 = 0b1111;

    let mut left = [true; 4];
    let mut current_index: usize = spawn.orientation() as usize;
    while needs_update != 0 {
        // println!("current_index: {}", current_index);
        // if the current index is not updated, skip it.
        if needs_update & (1 << current_index) == 0 {
            // println!("  skip");
            current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
            continue;
        }
        needs_update -= 1 << current_index;

        // initialize
        let src_piece = Piece::new(spawn.piece.shape, ORIENTATIONS_ORDER[current_index]);
        let src_index = current_index;

        // move
        loop {
            left[src_index] = !left[src_index];

            // println!("  move, left {}", left[src_index]);
            let reachable = reachables[src_index].clone();
            let reachable = reachable.move_n(&free_spaces[src_index], left[src_index]);

            if reachables[src_index] == reachable {
                break;
            }
            reachables[src_index] = reachable;
        }

        // cw rotate
        {
            let dest_piece = src_piece.cw();
            let dest_index = dest_piece.orientation as usize;

            let found_dest_reachable =
                rotate(rotation_system, Rotation::Cw, src_piece, &reachables[src_index], &free_spaces[dest_index]);

            let dest_reachable = reachables[dest_index].clone();
            let dest_reachable = dest_reachable.or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
            }
        }

        // ccw rotate
        {
            let dest_piece = src_piece.ccw();
            let dest_index = dest_piece.orientation as usize;

            let found_dest_reachable =
                rotate(rotation_system, Rotation::Ccw, src_piece, &reachables[src_index], &free_spaces[dest_index]);
            let dest_reachable = reachables[dest_index].clone();
            let dest_reachable = dest_reachable.or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
            }
        }

        current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
    }

    reachables
}

pub(crate) fn search_no_rotation(
    mut reachable: Reachable64,
    free_space: &FreeSpace64,
) -> Reachable64 {
    let mut left = false;
    loop {
        left = !left;

        let new_reachable = reachable.clone();
        let new_reachable = new_reachable.move_n(&free_space, left);

        if reachable == new_reachable {
            break;
        }
        reachable = new_reachable;
    }

    reachable
}

pub fn all_moves_softdrop_with_rotation(
    rotation_system: &impl RotationSystem,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves4 {
    let spawn = spawn.to_cc_placement();
    let free_spaces = to_free_spaces(board, spawn.piece.shape);
    let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);
    let reachables = search_with_rotation(rotation_system, spawn, reachables, &free_spaces);

    // landed
    let mut index = 0;
    let reachables = reachables.map(|reachable| {
        let candidate = reachable.land(&free_spaces[index]);
        index += 1;
        candidate
    });

    Moves4 { spawn, reachables }
}

pub fn all_moves_softdrop_no_rotation(
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves1 {
    let spawn = spawn.canonical_or_self().to_cc_placement();
    let free_space = to_free_space(board, spawn.piece);
    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);

    let reachable = search_no_rotation(reachable, &free_space);
    let reachable = reachable.land(&free_space);

    Moves1 { spawn, reachable, all_moves: true }
}
