use bitris::prelude::*;

fn main() {
    for shape in Shape::all_iter() {
        for orientation in Orientation::all_iter() {
            let piece = Piece::new(shape, orientation);
            if piece.canonical().is_some() {
                continue;
            }

            let piece_blocks = piece.to_piece_blocks();
            let top_right = piece_blocks.top_right;
            let function_name = format!("{:?}_{:?}", shape, orientation);
            println!("pub(crate) fn {}(board: BoardSimd) -> BoardSimd {{", function_name.to_lowercase());

            let d = top_right - piece_blocks.offsets[0];
            println!(
                "    let b1 = board.clone().shift_right::<{:?}>().shift_up::<{:?}>().and(",
                d.dx, d.dy
            );

            let d = top_right - piece_blocks.offsets[1];
            println!(
                "        board.clone().shift_right::<{:?}>().shift_up::<{:?}>()",
                d.dx, d.dy
            );

            println!("    );");

            let d = top_right - piece_blocks.offsets[2];
            println!(
                "    let b2 = board.clone().shift_right::<{:?}>().shift_up::<{:?}>().and(",
                d.dx, d.dy
            );

            let d = top_right - piece_blocks.offsets[3];
            println!(
                "        board.clone().shift_right::<{:?}>().shift_up::<{:?}>()",
                d.dx, d.dy
            );

            println!("    );");

            println!("    b1.and(b2)");

            println!("}}");

            println!();
        }
    }
}
