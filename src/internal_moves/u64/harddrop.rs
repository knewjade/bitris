use crate::boards::Board;
use crate::internal_moves::u64::loaders::{minimize, spawn_and_harddrop_reachable, spawn_and_harddrop_reachables, to_free_space, to_free_spaces};
use crate::internal_moves::u64::moves::{Moves1, Moves4};
use crate::placements::BlPlacement;
use crate::RotationSystem;

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
