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

pub fn spawn_and_harddrop_reachable(
    spawn: BlPlacement,
    free_spaces: &[FreeSpace64; 4],
) -> [Reachable64; 4] {
    // index
    let spawn_location = spawn.position.to_location();
    let orientation_index = spawn.piece.orientation as usize;
    let spawn_x = spawn_location.x as usize;

    // boards
    let mut spawn_reachable = Reachable64::blank();
    let spawn_free_space = free_spaces[orientation_index].cols;

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

    let mut reachables = [
        Reachable64::blank(),
        Reachable64::blank(),
        Reachable64::blank(),
        Reachable64::blank(),
    ];
    reachables[orientation_index] = spawn_reachable;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Orientation, Shape};
    use crate::srs::SrsKickTable;
    use crate::{
        boards::Board64,
        internal_moves::u64::reachable::Reachable64, pieces::Piece, Rotate, Rotation,
    };
    use std::str::FromStr;

    #[test]
    fn test_rotate() {
        let rotation_system = SrsKickTable;
        let from_piece = Piece {
            shape: Shape::I,
            orientation: Orientation::North,
        };

        let board = Board64::from_str(
            " \
            ......####\
            .........#\
            ########.#\
            #........#\
            #.######.#\
            #.######.#\
            #........#\
            #.########\
            #........#\
            #.######.#\
            #.######.#\
            #........#\
            ########.#\
            ########.#\
            #........#\
            #.######.#\
            #.######.#\
            #........#\
            .#########\
        ",
        )
        .unwrap();

        let free_spaces = free_spaces_each_pieces(&board, from_piece.shape);
        println!("{}", Board64::from(&free_spaces[0]));

        let src_reachable = Reachable64 {
            cols: [0, 0, 1 << 12, 0, 0, 0, 0, 0, 0, 0],
        };

        let result = rotate(
            &rotation_system,
            Rotation::Cw,
            from_piece,
            &src_reachable,
            &free_spaces[from_piece.cw().orientation as usize],
        );

        let expected_result = Reachable64 {
            cols: [0, 1 << 12, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        assert_eq!(result, expected_result);
    }
}
