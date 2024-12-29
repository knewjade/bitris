use crate::boards::Board;
use crate::internal_moves::u64::loaders::{can_reach1, can_reach4, minimize, spawn_and_harddrop_reachable, spawn_and_harddrop_reachables, to_free_space, to_free_spaces};
use crate::internal_moves::u64::moves::{Moves1, Moves4};
use crate::placements::BlPlacement;
use crate::{RotationSystem, With};
use crate::pieces::ToCcPosition;

pub fn moves_harddrop_with_rotation<const MINIMIZE: bool>(
    rotation_system: &impl RotationSystem,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves4 {
    let spawn = spawn.to_cc_placement();
    let free_spaces = to_free_spaces(board, spawn.piece.shape);
    let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);

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

pub fn moves_harddrop_no_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves1 {
    let spawn = spawn.canonical_or_self().to_cc_placement();
    let free_space = to_free_space(board, spawn.piece);
    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);

    let reachable = reachable.land(&free_space);

    Moves1 { spawn, reachable, minimized: MINIMIZE }
}

pub(crate) fn can_reach_harddrop_with_rotation(
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

    can_reach4(&reachables, &goals)
}

pub(crate) fn can_reach_strictly_harddrop_with_rotation(
    rotation_system: &impl RotationSystem,
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    let spawn = spawn.to_cc_placement();
    let goals = vec![goal.to_cc_placement()];

    let free_spaces = to_free_spaces(board, spawn.piece.shape);
    let reachables = spawn_and_harddrop_reachables(rotation_system, spawn, &free_spaces);

    can_reach4(&reachables, &goals)
}

pub(crate) fn can_reach_harddrop_no_rotation(
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    let spawn = spawn.canonical_or_self().to_cc_placement();
    let goal = goal.to_cc_placement();

    let free_space = to_free_space(board, spawn.piece);
    let reachable = spawn_and_harddrop_reachable(spawn, &free_space);

    can_reach1(&reachable, goal)
}

pub(crate) fn can_reach_strictly_harddrop_no_rotation(
    goal: BlPlacement,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> bool {
    crate::internal_moves::u64::softdrop::can_reach_softdrop_no_rotation(goal, board, spawn)
}
