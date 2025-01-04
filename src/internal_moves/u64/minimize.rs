use crate::internal_moves::u64::reachable::Reachable64;
use crate::pieces::Shape;
use std::mem;

// Extract canonical positions from the currently free positions.
#[inline(always)]
pub fn minimize(mut reachables: [Reachable64; 4], shape: Shape) -> [Reachable64; 4] {
    for piece in shape.all_pieces_iter() {
        match piece.canonical() {
            None => continue,
            Some(dest) => {
                let src_bl = piece.to_piece_blocks().bottom_left;
                let dest_bl = dest.to_piece_blocks().bottom_left;
                let offset = src_bl - dest_bl;
                let dest_reachable = mem::replace(
                    &mut reachables[dest.orientation as usize],
                    Reachable64::blank(),
                );
                let src_reachable = mem::replace(
                    &mut reachables[piece.orientation as usize],
                    Reachable64::blank(),
                );
                reachables[dest.orientation as usize] = dest_reachable
                    .or_shift(src_reachable, offset);
            }
        }
    }
    reachables
}
