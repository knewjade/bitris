#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitris::macros::piece;
    use bitris::prelude::*;
    
    #[test]
    fn moves() {
        use Shape::*;
        use Orientation::*;

        let board64: Board64 = Board64::from_str("
            ..........
            ......####
            .....#####
            ....######
            ...#######
            ..########
            .#########
        ").expect("Failed to create a board");

        let spawn = piece!(IN).with(cc(4, 20)).bl_placement();

        let all_moves = srs::MoveGenerator.generate_all_moves::<BlPlacement>(board64, spawn);
        assert_eq!(all_moves.len(), 34);

        let minimized_moves = srs::MoveGenerator.generate_minimized_moves::<BlPlacement>(board64, spawn);
        assert_eq!(minimized_moves.len(), 17);

        // The result includes both orientations that have the same form.
        assert!(all_moves.contains(&Piece::new(I, North).with(bl(0, 3))));
        assert!(all_moves.contains(&Piece::new(I, South).with(bl(0, 3))));

        // The result includes one orientation that has the same form.
        assert!(minimized_moves.contains(&Piece::new(I, North).with(bl(0, 3))));
        assert!(!minimized_moves.contains(&Piece::new(I, South).with(bl(0, 3))));
    }
}
