/** This file is for internal */

pub(crate) mod moves {
    use crate::avx2::aligned::AlignedU8s;
    use crate::avx2::board::BoardSimd;
    use crate::avx2::candidates::reachable_starting_celling;
    use crate::avx2::free::{load_free_space, free_spaces_each_pieces};
    use crate::boards::Board;
    use crate::pieces::Orientation;
    use crate::placements::BlPlacement;
    use crate::RotationSystem;

    #[derive(Debug)]
    pub struct Moves {
        pub spawn: BlPlacement,
    }

    impl Moves {
        #[inline]
        pub fn len(&self) -> usize {
            0
        }

        #[inline]
        pub fn vec(&self) -> Vec<BlPlacement> {
            self.vec_with_capacity(self.len())
        }

        /// `capacity` is a hint and does not affect the result.
        pub fn vec_with_capacity(&self, capacity: usize) -> Vec<BlPlacement> {
            let mut out = Vec::<BlPlacement>::with_capacity(capacity);
            out
        }
    }

    pub(crate) fn all_moves_softdrop(
        board: &Board<u64>,
        spawn: BlPlacement,
    ) -> Moves {
        let mut free_spaces = free_spaces_each_pieces(load_free_space(board), spawn.piece.shape);
        let mut reachables: [BoardSimd; 4] = reachable_starting_celling(&free_spaces);

        // move
        for orientation in Orientation::all_iter() {
            loop {
                let index = orientation as usize;

                let reachable = BoardSimd::new(reachables[index].data);
                let reachable = reachable.move1(&free_spaces[index]);

                if reachables[index] == reachable {
                    break;
                }
                reachables[index] = reachable;
            }
        }

        // lockable
        for index in 0..4 {
            let reachable = BoardSimd::new(reachables[index].data);
            reachables[index] = reachable.lock();
        }

        use crate::boards::Board16;

        let board = Board16::from(&reachables[0]);
        println!("{}", board);

        Moves { spawn }
    }

    pub(crate) fn all_moves_harddrop(
        board: &Board<u64>,
        spawn: BlPlacement,
    ) -> Moves {
        let free_spaces = free_spaces_each_pieces(load_free_space(board), spawn.piece.shape);
        let candidates: [BoardSimd; 4] = reachable_starting_celling(&free_spaces);
        Moves { spawn }
    }
}

#[cfg(test)]
mod tests {
    use crate::avx2::free::{free_spaces_each_pieces, load_free_space};
    use crate::avx2::internal_moves::moves::{all_moves_softdrop};
    use crate::boards::{Board16, Board64};
    use crate::pieces::{Piece, Shape, Orientation};
    use std::str::FromStr;
    use crate::piece;
    use crate::prelude::{bl, With};

    #[test]
    fn test_no_rotate() {
        let board = Board64::from_str(
            "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ",
        ).unwrap();
        all_moves_softdrop(&board, piece!(TN).with(bl(0, 0)));

        // let board_simd = load_free_space(&board);
        // let free_spaces = free_spaces_each_pieces(board_simd, Shape::T);
        // let board = Board16::from(&free_spaces[0]);
        // assert_eq!(board,
        //            Board16::from_str(
        //                "\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ..########\
        //         ....###...\
        //         ....##....\
        //         ....#.....\
        //         ....##....\
        //         ..........\
        //     ").unwrap()
        // )
    }
}
