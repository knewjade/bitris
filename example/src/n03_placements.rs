#[cfg(test)]
mod tests {
    use bitris::prelude::*;
    use bitris::macros::piece;
    
    #[test]
    fn placements() {
        let mut board = Board64::blank();
    
        let o_north = piece!(ON);
    
        let lines_cleared = o_north.with(bl(0, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::blank()));
    
        let lines_cleared = o_north.with(bl(2, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::blank()));
    
        let lines_cleared = o_north.with(bl(4, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::blank()));
    
        let lines_cleared = o_north.with(bl(6, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::blank()));
    
        assert_eq!(format!("{}", board), "(Board64):
..........
########..
########..");
    
        let lines_cleared = o_north.with(bl(8, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::new(0b11)));
        if let Some(lines) = lines_cleared {
            assert_eq!(lines.count(), 2);
        }
    
        // Access off board
        let lines_cleared = o_north.with(bl(-1, -1)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, None);
    }
}
