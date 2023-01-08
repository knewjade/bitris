use crate::{BlPlacement, Board64};
use crate::internal_moves::moves64;
use crate::srs::SrsKickTable;

/// Collect all the places that can be placed in srs.
/// If the placements have the same block positions, but the orientations are different, each will be collected.
pub fn generate_all_moves(board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
    let result = moves64::all_moves_softdrop(&SrsKickTable, &board, spawn);
    result.vec()
}

/// Collect all the places that can be placed in srs.
/// If the placements have the same block positions, but the orientations are different, one of the placements will be collected.
/// It is guaranteed that the placement to be collected is actually in the orientation where it can be placed.
pub fn generate_minimized_moves(board: Board64, spawn: BlPlacement) -> Vec<BlPlacement> {
    let result = moves64::minimized_moves_softdrop(&SrsKickTable, &board, spawn);
    result.vec()
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
}
