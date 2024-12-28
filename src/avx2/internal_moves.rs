/** This file is for internal */

pub(crate) mod moves {
    use crate::avx2::candidates::reachable_starting_celling;
    use crate::avx2::free_loaders::{free_spaces_each_pieces, load_free_space};
    use crate::avx2::reachable::ReachableSimd16;
    use crate::avx2::rotate_loaders::{rotate_ccw, rotate_cw};
    use crate::boards::{Board, Board16};
    use crate::coordinates::bl;
    use crate::pieces::{Orientation, Piece};
    use crate::placements::BlPlacement;
    use crate::{Rotate, With};

    #[derive(Debug)]
    pub struct Moves<T> {
        pub spawn: BlPlacement,
        pub boards: [Board<T>; 4],
    }

    impl Moves<u16> {
        #[inline]
        pub fn vec(&self) -> Vec<BlPlacement> {
            let mut out = Vec::<BlPlacement>::with_capacity(128);

            for piece in self.spawn.piece.shape.all_pieces_iter() {
                let board = &self.boards[piece.orientation as usize];
                for lx in 0..10 {
                    let mut col = board.cols[lx];
                    while 0 < col {
                        let by = col.trailing_zeros();
                        out.push(piece.with(bl(lx as i32, by as i32)));
                        col -= 1 << by;
                    }
                }
            }

            out
        }
    }

    pub(crate) fn all_moves_softdrop(board: &Board<u64>, spawn: BlPlacement) -> Moves<u16> {
        let free_space_block = load_free_space(board);
        let mut free_spaces = free_spaces_each_pieces(free_space_block, spawn.piece.shape);
        let mut reachables: [ReachableSimd16; 4] = reachable_starting_celling(&free_spaces);

        const ORIENTATIONS_ORDER: [Orientation; 4] = [
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ];

        let mut needs_update: u8 = 0b1111;

        let mut current_index: usize = 0;
        while needs_update != 0 {
            // if the current index is not updated, skip it.
            if needs_update & (1 << current_index) == 0 {
                current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
                continue;
            }
            needs_update -= 1 << current_index;

            // initialize
            let src_piece = Piece::new(spawn.piece.shape, ORIENTATIONS_ORDER[current_index]);
            let src_index = current_index;

            // move
            loop {
                let reachable = ReachableSimd16::new(reachables[src_index].data);
                let reachable = reachable.move1(&free_spaces[src_index]);

                if reachables[src_index] == reachable {
                    break;
                }
                reachables[src_index] = reachable;
            }

            // cw rotate
            {
                let dest_piece = src_piece.cw();
                let dest_index = dest_piece.orientation as usize;

                let found_dest_reachable =
                    rotate_cw(dest_piece, &reachables[src_index], &free_spaces[dest_index]);
                let dest_reachable = ReachableSimd16::new(reachables[dest_index].data);
                let dest_reachable = dest_reachable.or(&found_dest_reachable);

                if reachables[dest_index] != dest_reachable {
                    reachables[dest_index] = dest_reachable;
                    needs_update |= 1 << dest_index;
                }
            }

            // ccw rotate
            {
                let dest_piece = src_piece.cw();
                let dest_index = dest_piece.orientation as usize;

                let found_dest_reachable =
                    rotate_ccw(dest_piece, &reachables[src_index], &free_spaces[dest_index]);
                let dest_reachable = ReachableSimd16::new(reachables[dest_index].data);
                let dest_reachable = dest_reachable.or(&found_dest_reachable);

                if reachables[dest_index] != dest_reachable {
                    reachables[dest_index] = dest_reachable;
                    needs_update |= 1 << dest_index;
                }
            }

            current_index = (current_index + 1) % ORIENTATIONS_ORDER.len();
        }

        // landed
        for index in 0..4 {
            let reachable = ReachableSimd16::new(reachables[index].data);
            reachables[index] = reachable.land(&free_spaces[index]);
        }

        Moves {
            spawn,
            boards: [
                Board16::from(&reachables[0]),
                Board16::from(&reachables[1]),
                Board16::from(&reachables[2]),
                Board16::from(&reachables[3]),
            ],
        }
    }

    pub(crate) fn all_moves_harddrop(board: &Board<u64>, spawn: BlPlacement) -> Moves<u16> {
        let free_spaces = free_spaces_each_pieces(load_free_space(board), spawn.piece.shape);
        let _candidates: [ReachableSimd16; 4] = reachable_starting_celling(&free_spaces);
        Moves {
            spawn,
            boards: [Board16::blank(); 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avx2::internal_moves::moves::all_moves_softdrop;
    use crate::boards::Board64;
    use crate::piece;
    use crate::pieces::{Orientation, Piece, Shape};
    use crate::prelude::{bl, With};
    use std::str::FromStr;

    #[test]
    fn test_no_rotate() {
        let board = Board64::from_str(
            "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ",
        )
        .unwrap();
        let moves = all_moves_softdrop(&board, piece!(TN).with(bl(0, 0)));
        println!("{:?}", moves.vec());
        println!("{:?}", moves.vec().len());
    }
}
