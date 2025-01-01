use crate::array_map::zip2_map4;
use crate::boards::Board;
use crate::coordinates::cc;
use crate::internal_moves::avx2::loaders::*;
use crate::internal_moves::avx2::minimize::minimize;
use crate::internal_moves::avx2::moves::Moves4;
use crate::internal_moves::avx2::softdrop16;
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
pub fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves4 {
    // TODO board.well_top() <= 20

    let free_spaces = to_free_spaces_lower(board, spawn.piece.shape);

    // スポーン位置を下のボードまでスキップする。
    // ボードで最も高いブロックの位置がy=10以下であるため、
    // `I-East (cy=13)` が左右移動しても引っかからない。
    // そのため、cy=13まで無条件でしても、その後の左右移動・ハードドロップに影響しない。
    let spawn = if spawn.position.cy < 13 {
        spawn
    } else {
        spawn.piece.with(cc(spawn.position.cx, 13))
    };

    let reachables = spawn_and_harddrop_reachables(spawn, &free_spaces);
    let reachables =
        softdrop16::search_with_rotation::<false>(spawn.piece, reachables, &free_spaces);

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
