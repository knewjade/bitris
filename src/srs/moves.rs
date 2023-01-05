use crate::{BlPlacement, Board64};

mod srs64 {
    use crate::{BlPlacement, Board};
    use crate::internal_moves::moves64::*;
    use crate::srs::SrsKickTable;

    pub(crate) fn all_moves(board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let kick_table = SrsKickTable;

        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable(&spawn, &free_piece_boards, &kick_table);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);

        Moves { spawn, reachable_piece_boards }
    }

    pub(crate) fn minimized_moves(board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let kick_table = SrsKickTable;

        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable(&spawn, &free_piece_boards, &kick_table);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);
        reachable_piece_boards.minimize(spawn.piece.shape);

        Moves { spawn, reachable_piece_boards }
    }
}

/// Collect all the places that can be placed in srs.
/// If the placements have the same block positions, but the orientations are different, each will be collected.
pub fn generate_all_moves(board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
    let result = srs64::all_moves(&board, spawn);
    result.vec::<BlPlacement>()
}

/// Collect all the places that can be placed in srs.
/// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
/// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
pub fn generate_minimized_moves(board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
    let result = srs64::minimized_moves(&board, spawn);
    result.vec::<BlPlacement>()
}


/// Facade for the generation of moves.
pub struct MoveGenerator;

impl MoveGenerator {
    /// Collect all the places that can be placed in srs.
    /// If the placements have the same block positions, but the orientations are different, each will be collected.
    pub fn generate_all_moves<P>(self, board: impl Into<Board64>, spawn: impl Into<BlPlacement>) -> Vec<P> where P: From<BlPlacement> {
        let result = srs64::all_moves(&board.into(), spawn.into());
        result.vec::<P>()
    }

    /// Collect all the places that can be placed in srs.
    /// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
    /// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
    pub fn generate_minimized_moves<P>(self, board: impl Into<Board64>, spawn: impl Into<BlPlacement>) -> Vec<P> where P: From<BlPlacement> {
        let result = srs64::minimized_moves(&board.into(), spawn.into());
        result.vec::<P>()
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;

    #[test]
    fn generate_all_moves() {
        let board = Board64::from_str(" \
            ..........\
            ..........\
            ..........\
            ..........\
        ").unwrap();
        let placement = piece!(TN).with(bl(4, 20));
        let moves = srs::generate_all_moves(board, placement);
        assert_eq!(moves.len(), 34);
    }

    #[test]
    fn generate_minimized_moves() {
        let board = Board64::from_str(" \
            ..........\
            ..........\
            ..........\
            ..........\
        ").unwrap();
        let placement = piece!(ON).with(bl(4, 20));
        let moves = srs::generate_minimized_moves(board, placement);
        assert_eq!(moves.len(), 9);
    }

    #[test]
    fn by_facade() {
        let board = Board32::from_str(" \
            ..........\
            ..........\
            ..........\
            ..........\
        ").unwrap();
        let placement = piece!(ON).with(cc(4, 20));
        let moves = srs::MoveGenerator.generate_minimized_moves::<CcPlacement>(board, placement);
        assert_eq!(moves.len(), 9);
    }
}
