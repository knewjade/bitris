use crate::boards::Board;
use crate::internal_moves::u64::moves::Moves4;
use crate::placements::BlPlacement;
use crate::RotationSystem;

pub fn all_moves_harddrop_rotation(
    rotation_system: &impl RotationSystem,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves4 {
    // let free_board = FreeBoard::from(board);
    // let free_piece_boards = FreePieceBoards::new_according_to(spawn.piece.shape, &free_board);
    //
    // let mut reachable_piece_boards =
    //     gen_reachable_harddrop(&spawn, &free_piece_boards, rotation_system);
    // reachable_piece_boards.extract_landed_positions(&free_piece_boards);
    //
    // Moves {
    //     spawn,
    //     reachables: reachable_piece_boards,
    // }
    todo!()
}

pub fn all_moves_harddrop_no_rotation(
    rotation_system: &impl RotationSystem,
    board: &Board<u64>,
    spawn: BlPlacement,
) -> Moves4 {
    // let free_board = FreeBoard::from(board);
    // let free_piece_boards = FreePieceBoards::new_according_to(spawn.piece.shape, &free_board);
    //
    // let mut reachable_piece_boards =
    //     gen_reachable_harddrop(&spawn, &free_piece_boards, rotation_system);
    // reachable_piece_boards.extract_landed_positions(&free_piece_boards);
    //
    // Moves {
    //     spawn,
    //     reachables: reachable_piece_boards,
    // }
    todo!()
}
