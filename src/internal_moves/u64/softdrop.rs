use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::internal_moves::u64::loaders::{rotate, to_free_space, to_free_spaces, spawn_and_harddrop_reachable, spawn_and_harddrop_reachables, minimize, can_reach4, can_reach1};
use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::{Orientation, Piece, ToCcPosition};
use crate::placements::{BlPlacement, CcPlacement};
use crate::{Rotate, Rotation, RotationSystem, With};
use crate::boards::Board;
use crate::internal_moves::u64::moves::{Moves1, Moves4};

const ORIENTATIONS_ORDER: [Orientation; 4] = [
    Orientation::North,
    Orientation::East,
    Orientation::South,
    Orientation::West,
];

pub(crate) fn search_with_rotation(
    rotation_system: &impl RotationSystem,
    spawn: CcPlacement,
    mut reachables: [Reachable64; 4],
    free_spaces: &[FreeSpace64; 4],
) -> [Reachable64; 4] {
    let mut needs_update: u8 = 0b1111;

    let mut left = [true; 4];
    let mut current_index: usize = spawn.orientation() as usize;
    while needs_update != 0 {
        // if the current index is not updated, skip it.
        if needs_update & (1 << current_index) == 0 {
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

            let reachable = reachables[src_index]
                .clone()
                .move_n(&free_spaces[src_index], left[src_index]);

            if reachables[src_index] == reachable {
                break;
            }
            reachables[src_index] = reachable;
        }

        // cw rotate
        {
            let dest_index = src_piece.cw().orientation as usize;

            let found_dest_reachable = rotate(
                    rotation_system,
                    Rotation::Cw,
                    src_piece,
                    &reachables[src_index],
                    &free_spaces[dest_index],
            );

            let dest_reachable = reachables[dest_index]
                .clone()
                .or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
            }
        }

        // ccw rotate
        {
            let dest_index = src_piece.ccw().orientation as usize;

            let found_dest_reachable = rotate(
                rotation_system,
                Rotation::Ccw,
                src_piece,
                &reachables[src_index],
                &free_spaces[dest_index],
            );

            let dest_reachable = reachables[dest_index]
                .clone()
                .or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
            }
        }

        current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
    }

    reachables
}

pub(crate) fn can_reach_with_rotation(
    rotation_system: &impl RotationSystem,
    spawn: CcPlacement,
    mut reachables: [Reachable64; 4],
    free_spaces: &[FreeSpace64; 4],
    goals: &[CcPlacement],
) -> bool {
    let mut needs_update: u8 = 0b1111;

    let mut left = [true; 4];
    let mut current_index: usize = spawn.orientation() as usize;
    while needs_update != 0 {
        // if the current index is not updated, skip it.
        if needs_update & (1 << current_index) == 0 {
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

            let reachable = reachables[src_index]
                .clone()
                .move_n(&free_spaces[src_index], left[src_index]);

            if reachables[src_index] == reachable {
                break;
            }
            reachables[src_index] = reachable;
        }

        // cw rotate
        {
            let dest_index = src_piece.cw().orientation as usize;

            let found_dest_reachable = rotate(
                rotation_system,
                Rotation::Cw,
                src_piece,
                &reachables[src_index],
                &free_spaces[dest_index],
            );

            let dest_reachable = reachables[dest_index]
                .clone()
                .or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
            }
        }

        // ccw rotate
        {
            let dest_index = src_piece.ccw().orientation as usize;

            let found_dest_reachable = rotate(
                rotation_system,
                Rotation::Ccw,
                src_piece,
                &reachables[src_index],
                &free_spaces[dest_index],
            );

            let dest_reachable = reachables[dest_index]
                .clone()
                .or(&found_dest_reachable);

            if reachables[dest_index] != dest_reachable {
                reachables[dest_index] = dest_reachable;
                needs_update |= 1 << dest_index;
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

pub(crate) fn can_reach_no_rotation(
    mut reachable: Reachable64,
    free_space: &FreeSpace64,
    goal: CcPlacement,
) -> bool {
    let mut left = false;
    loop {
        left = !left;

        let new_reachable = reachable.clone();
        let new_reachable = new_reachable.move_n(&free_space, left);

        if reachable == new_reachable {
            break;
        }

        if can_reach1(&reachable, goal) {
            return true;
        }

        reachable = new_reachable;
    }

    false
}

pub fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
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

    let reachables = if MINIMIZE {
        minimize(reachables, spawn.piece.shape)
    } else {
        reachables
    };

    Moves4 { spawn, reachables }
}

pub fn moves_softdrop_no_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves1 {
    let spawn = spawn.canonical_or_self().to_cc_placement();
    let free_space = to_free_space(board, spawn.piece);
    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);

    let reachable = search_no_rotation(reachable, &free_space);
    let reachable = reachable.land(&free_space);

    Moves1 { spawn, reachable, minimized: MINIMIZE }
}

pub(crate) fn can_reach_softdrop_with_rotation(
    rotation_system: &impl RotationSystem,
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    let spawn = spawn.to_cc_placement();
    let goals = goal.piece.orientations_having_same_form()
        .iter()
        .map(|&orientation| goal.piece.shape.with(orientation))
        .map(|piece| piece.with(goal.position.to_cc_position(piece.to_piece_blocks())))
        .collect::<Vec<_>>();

    let free_spaces = to_free_spaces(board, spawn.piece.shape);
    let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);

    if can_reach4(&reachables, &goals) {
        return true;
    }

    can_reach_with_rotation(rotation_system, spawn, reachables, &free_spaces, &goals)
}

pub(crate) fn can_reach_strictly_softdrop_with_rotation(
    rotation_system: &impl RotationSystem,
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    let spawn = spawn.to_cc_placement();
    let goals = vec![goal.to_cc_placement()];

    let free_spaces = to_free_spaces(board, spawn.piece.shape);
    let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);

    if can_reach4(&reachables, &goals) {
        return true;
    }

    can_reach_with_rotation(rotation_system, spawn, reachables, &free_spaces, &goals)
}

pub(crate) fn can_reach_softdrop_no_rotation(
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    let spawn = spawn.canonical_or_self().to_cc_placement();
    let goal = goal.to_cc_placement();

    let free_space = to_free_space(board, spawn.piece);
    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);

    if can_reach1(&reachable, goal) {
        return true;
    }

    can_reach_no_rotation(reachable, &free_space, goal)
}

pub(crate) fn can_reach_strictly_softdrop_no_rotation(
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    can_reach_softdrop_no_rotation(goal, board, spawn)
}
