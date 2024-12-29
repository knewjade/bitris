use crate::boards::Board64;
use crate::internal_moves::u64::free::to_free_spaces;
use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::{Piece, Shape};
use crate::placements::BlPlacement;
use crate::{Rotation, RotationSystem};

// ブロックと空を反転して読み込み
pub fn free_spaces_each_pieces(board: &Board64, shape: Shape) -> [FreeSpace64; 4] {
    let free_space_block = FreeSpace64::new(board.cols.map(|col| !col));
    to_free_spaces(free_space_block, shape)
}

pub fn spawn_reachable(spawn: BlPlacement) -> [Reachable64; 4] {
    let mut reachables = [
        Reachable64::blank(),
        Reachable64::blank(),
        Reachable64::blank(),
        Reachable64::blank(),
    ];
    let index = spawn.piece.orientation as usize;
    let location = spawn.position.to_location();
    reachables[index] = reachables[index].clone().set_at(location);
    reachables
}

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
