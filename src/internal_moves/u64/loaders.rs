use crate::boards::Board64;
use crate::internal_moves::u64::free;
use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::{Orientation, Piece, Shape};
use crate::{Rotate, Rotation, RotationSystem, With};
use crate::prelude::CcPlacement;

// ブロックと空を反転して読み込み
#[inline(always)]
pub fn to_free_spaces(board: &Board64, shape: Shape) -> [FreeSpace64; 4] {
    let free_space_block = FreeSpace64::new(board.cols.map(|col| !col));
    free::to_free_spaces(free_space_block, shape)
}

// ブロックと空を反転して読み込み
#[inline(always)]
pub fn to_free_space(board: &Board64, piece: Piece) -> FreeSpace64 {
    let free_space_block = FreeSpace64::new(board.cols.map(|col| !col));
    free::to_free_space(free_space_block, piece)
}

#[inline(always)]
pub fn spawn_and_harddrop_reachables(
    rotation_system: &impl RotationSystem,
    spawn: CcPlacement,
    free_spaces: &[FreeSpace64; 4],
) -> [Reachable64; 4] {
    let mut placements = [None; 4];

    // spawn
    placements[spawn.piece.orientation as usize] = Some(spawn);

    // rotate
    for rotation in [Rotation::Cw, Rotation::Ccw] {
        let mut prev = spawn;
        for _ in 0..3 {
            let current_piece = prev.piece.rotate(rotation);
            let orientation_index = current_piece.orientation as usize;

            if placements[orientation_index].is_some() {
                break
            }

            // use first kick
            let offset = rotation_system.iter_kicks(prev.piece, rotation)
                .next()
                .unwrap()
                .offset;
            let current_position = prev.position + offset;
            if !free_spaces[orientation_index].is_free_space(current_position.to_location()) {
                break
            }

            let current = current_piece.with(current_position);
            placements[orientation_index] = Some(current);
            prev = current;
        }
    }

    let mut index = 0;
    placements.map(|placement| {
        let reachable = placement
            .map(|p| spawn_and_harddrop_reachable(p, &free_spaces[index]))
            .unwrap_or_else(|| Reachable64::blank());
        index += 1;
        reachable
    })
}

#[inline(always)]
pub fn spawn_and_harddrop_reachable(spawn: CcPlacement, free_space: &FreeSpace64) -> Reachable64 {
    // index
    let spawn_location = spawn.position.to_location();
    let spawn_x = spawn_location.x as usize;

    // boards
    let mut spawn_reachable = Reachable64::blank();
    let spawn_free_space = free_space.cols;

    // a spawn bit
    let spawn_bit = 1u64 << spawn_location.y;

    // 1-mask over spawn y
    let mask = u64::MAX - ((1u64 << (spawn_location.y + 1)) - 1);

    // left
    for x in (0..spawn_x).rev() {
        let free_space = spawn_free_space[x];
        if (spawn_bit & free_space) == 0 {
            break;
        }

        // harddrop
        let harddrop_dest_y = 64 - (!(free_space | mask)).leading_zeros();
        if harddrop_dest_y < spawn_location.y as u32 {
            let reachable = (spawn_bit - 1) - ((1 << harddrop_dest_y) - 1);
            spawn_reachable.cols[x] = spawn_bit | reachable;
        }
    }

    // right
    for x in spawn_x..10 {
        let free_space = spawn_free_space[x];
        if (spawn_bit & free_space) == 0 {
            break;
        }

        // harddrop
        let harddrop_dest_y = 64 - (!(free_space | mask)).leading_zeros();
        if harddrop_dest_y < spawn_location.y as u32 {
            let reachable = (spawn_bit - 1) - ((1 << harddrop_dest_y) - 1);
            spawn_reachable.cols[x] = spawn_bit | reachable;
        }
    }
    spawn_reachable
}

#[inline(always)]
pub fn rotate(
    rotation_system: &impl RotationSystem,
    rotation: Rotation,
    from_piece: Piece,
    src_reachable: &Reachable64,
    dest_free_space: &FreeSpace64,
) -> Reachable64 {
    debug_assert!(!src_reachable.empty());

    let mut src_candidates = src_reachable.clone();
    let mut dest_reachable = Reachable64::blank();

    let kicks = rotation_system
        .iter_kicks(from_piece, rotation)
        .enumerate()
        .collect::<Vec<_>>();

    for (index, &kick) in &kicks {
        let shift_forward = src_candidates
            .clone()
            .jump_and(dest_free_space, kick.offset);
        dest_reachable = dest_reachable.or(&shift_forward);

        let last = *index == kicks.len() - 1;
        if !last {
            src_candidates = src_candidates.jump_rev(shift_forward, -kick.offset);
            if src_candidates.empty() {
                break;
            }
        }
    }

    dest_reachable
}

// Extract canonical positions from the currently free positions.
#[inline(always)]
pub fn minimize(reachables: [Reachable64; 4], shape: Shape) -> [Reachable64; 4] {
    let mut reachables = reachables;
    for piece in shape.all_pieces_iter() {
        match piece.canonical() {
            None => {
                continue
            }
            Some(dest) => {
                let src_bl = piece.to_piece_blocks().bottom_left;
                let dest_bl = dest.to_piece_blocks().bottom_left;
                let offset = src_bl - dest_bl;
                reachables[dest.orientation as usize] = reachables[dest.orientation as usize]
                    .clone()
                    .or_shift(&reachables[piece.orientation as usize], offset);
                reachables[piece.orientation as usize] = Reachable64::blank();
            }
        }
    }
    reachables
}

pub fn can_reach4(reachables: &[Reachable64; 4], goals: &[CcPlacement]) -> bool {
    goals
        .iter()
        .any(|&goal_placement| {
            let orientation_index = goal_placement.piece.orientation as usize;
            let location = goal_placement.position.to_location();
            reachables[orientation_index].is_visited(location)
        })
}

pub fn can_reach1(reachable: &Reachable64, goal: CcPlacement) -> bool {
    let location = goal.position.to_location();
    reachable.is_visited(location)
}
