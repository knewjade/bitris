use crate::array_map::map_indexed4;
use crate::boards::Board;
use crate::coordinates::cc;
use crate::internal_moves::avx2::h16;
use crate::internal_moves::avx2::h16::loaders::*;
use crate::internal_moves::avx2::h16::minimize::minimize;
use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;
use crate::internal_moves::avx2::h16::x2::loaders::*;
use crate::internal_moves::avx2::moves::{Moves1, Moves4};
use crate::pieces::Orientation;
use crate::placements::CcPlacement;
use crate::With;

const ORIENTATIONS_ORDER: [Orientation; 4] = [
    Orientation::North,
    Orientation::East,
    Orientation::South,
    Orientation::West,
];

#[inline(always)]
pub(crate) fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves4 {
    todo!()
    //
    // let free_spaces_pair = to_free_spaces_pair(board, spawn);
    //
    // let mut reachables_pair = spawn_and_harddrop_reachables_pair(spawn, &free_spaces_pair);
    //
    // let mut current_index = 1;
    // let mut needs_update: u8 = 0b11;
    // if *&reachables_pair.lower.iter().all(|r| r.empty()) {
    //     needs_update -= 0b01;
    // }
    // if *&reachables_pair.upper.iter().all(|r| r.empty()) {
    //     needs_update -= 0b10;
    // }
    //
    // while needs_update != 0 {
    //     // if the current index is not updated, skip it.
    //     if needs_update & (1 << current_index) == 0 {
    //         current_index = (current_index + 1) % 2;
    //         continue;
    //     }
    //     needs_update -= 1 << current_index;
    //
    //     if current_index == 0 {
    //         // lower
    //         let Pair {
    //             lower: reachables_lower,
    //             upper: reachables_upper,
    //         } = reachables_pair.clone();
    //
    //         reachables_pair.lower = h16::softdrop::search_with_rotation::<false>(
    //             spawn.piece,
    //             reachables_lower,
    //             &free_spaces_pair.lower,
    //         );
    //
    //         let reachables_upper = map_indexed4(reachables_upper, |index, reachable| {
    //             reachable.or_shift::<0, 0, 12, 0>(&reachables_pair.lower[index])
    //         });
    //         if reachables_pair.upper != reachables_upper {
    //             reachables_pair.upper = reachables_upper;
    //             needs_update |= 0b10;
    //         }
    //     } else {
    //         // upper
    //         let Pair {
    //             lower: reachables_lower,
    //             upper: reachables_upper,
    //         } = reachables_pair.clone();
    //
    //         reachables_pair.upper = h16::softdrop::search_with_rotation::<true>(
    //             spawn.piece,
    //             reachables_upper,
    //             &free_spaces_pair.upper,
    //         );
    //
    //         let reachables_lower = map_indexed4(reachables_lower, |index, reachable| {
    //             reachable.or_shift::<0, 0, 0, 12>(&reachables_pair.upper[index])
    //         });
    //         if reachables_pair.lower != reachables_lower {
    //             reachables_pair.lower = reachables_lower;
    //             needs_update |= 0b01;
    //         }
    //     }
    //
    //     current_index = (current_index + 1) % 2;
    // }
    //
    // // landed
    // let reachables_pair = land(&reachables_pair, &free_spaces_pair);
    //
    // let reachables_pair = if MINIMIZE {
    //     Pair::new(
    //         minimize(reachables_pair.lower, spawn.piece.shape),
    //         minimize(reachables_pair.upper, spawn.piece.shape),
    //     )
    // } else {
    //     reachables_pair
    // };
    //
    // Moves4 {
    //     spawn_piece: spawn.piece,
    //     reachables: to_bytes_u32x4(reachables_pair),
    // }
}

#[inline(always)]
pub(crate) fn moves_softdrop_no_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves1 {
    todo!()
    // debug_assert!(13 < spawn.position.cy);
    //
    // let free_space_pair = to_free_space_pair(board, spawn);
    //
    // let spawn = spawn
    //     .piece
    //     .with(cc(spawn.position.cx, spawn.position.cy - 12));
    //
    // // search upper
    // let reachable_upper = spawn_and_harddrop_reachable(spawn, &free_space_pair.upper);
    // let reachable_upper =
    //     h16::softdrop::search_no_rotation(reachable_upper, &free_space_pair.upper);
    //
    // // search lower
    // let reachable_lower = ReachableSimd16::blank().or_shift::<0, 0, 0, 12>(&reachable_upper);
    // let reachable_lower = if !reachable_lower.empty() {
    //     h16::softdrop::search_no_rotation(reachable_lower, &free_space_pair.lower)
    // } else {
    //     reachable_lower
    // };
    //
    // // landed
    // let reachable_lower = reachable_lower.land(&free_space_pair.lower);
    // let reachable_upper = reachable_upper.clip(0xFFFE).land(&free_space_pair.upper);
    //
    // Moves1 {
    //     spawn_piece: spawn.piece,
    //     reachable: to_bytes_u32(&reachable_lower, &reachable_upper),
    //     minimized: MINIMIZE,
    // }
}
