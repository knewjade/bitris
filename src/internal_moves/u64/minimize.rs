use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::Shape;

// Extract canonical positions from the currently free positions.
#[inline(always)]
pub fn minimize(reachables: [Reachable64; 4], shape: Shape) -> [Reachable64; 4] {
    let mut reachables = reachables;
    for piece in shape.all_pieces_iter() {
        match piece.canonical() {
            None => continue,
            Some(dest) => {
                let src_bl = piece.to_piece_blocks().bottom_left;
                let dest_bl = dest.to_piece_blocks().bottom_left;
                let offset = src_bl - dest_bl;
                reachables[dest.orientation as usize] = reachables[dest.orientation as usize]
                    .clone()
                    .or_shift(&reachables[piece.orientation as usize], offset);
                reachables[piece.orientation as usize] = Reachable64::blank();
            }
        }
    }
    reachables
}
