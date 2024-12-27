use crate::avx2::aligned::AlignedU8s;
use crate::avx2::board::BoardSimd;
use crate::boards::Board;
use crate::pieces::Shape;

// ブロックと空を反転して、下位16bitを読み込み
pub(crate) fn load_free_space(board: &Board<u64>) -> BoardSimd {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_u8: [u8; 32] = [
        bytes_u64[0] as u8,
        (bytes_u64[0] >> 8) as u8,
        bytes_u64[1] as u8,
        (bytes_u64[1] >> 8) as u8,
        bytes_u64[2] as u8,
        (bytes_u64[2] >> 8) as u8,
        bytes_u64[3] as u8,
        (bytes_u64[3] >> 8) as u8,
        bytes_u64[4] as u8,
        (bytes_u64[4] >> 8) as u8,
        bytes_u64[5] as u8,
        (bytes_u64[5] >> 8) as u8,
        bytes_u64[6] as u8,
        (bytes_u64[6] >> 8) as u8,
        bytes_u64[7] as u8,
        (bytes_u64[7] >> 8) as u8,
        bytes_u64[8] as u8,
        (bytes_u64[8] >> 8) as u8,
        bytes_u64[9] as u8,
        (bytes_u64[9] >> 8) as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    BoardSimd::from(AlignedU8s::new(bytes_u8))
}

pub(crate) fn free_spaces_each_pieces(board_simd: BoardSimd, shape: Shape) -> [BoardSimd; 4] {
    match shape {
        Shape::T => [
            free::t_north(board_simd.clone()),
            free::t_east(board_simd.clone()),
            free::t_south(board_simd.clone()),
            free::t_west(board_simd.clone()),
        ],
        Shape::I => {
            let north = free::i_north(board_simd.clone());
            let east = free::i_east(board_simd.clone());
            [north.clone(), east.clone(), north, east]
        }
        Shape::O => {
            let north = free::o_north(board_simd.clone());
            [north.clone(), north.clone(), north.clone(), north]
        }
        Shape::L => [
            free::l_north(board_simd.clone()),
            free::l_east(board_simd.clone()),
            free::l_south(board_simd.clone()),
            free::l_west(board_simd.clone()),
        ],
        Shape::J => [
            free::j_north(board_simd.clone()),
            free::j_east(board_simd.clone()),
            free::j_south(board_simd.clone()),
            free::j_west(board_simd.clone()),
        ],
        Shape::S => {
            let north = free::s_north(board_simd.clone());
            let east = free::s_east(board_simd.clone());
            [north.clone(), east.clone(), north, east]
        }
        Shape::Z => {
            let north = free::z_north(board_simd.clone());
            let east = free::z_east(board_simd.clone());
            [north.clone(), east.clone(), north, east]
        }
    }
}

mod free {
    use crate::avx2::board::BoardSimd;

    pub(crate) fn t_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn t_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn t_south(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn t_west(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn i_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<3>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<2>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn i_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<3>());
        b1.and(b2)
    }

    pub(crate) fn o_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn l_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn l_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<2>());
        b1.and(b2)
    }

    pub(crate) fn l_south(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<2>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn l_west(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn j_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<2>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn j_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<2>());
        b1.and(b2)
    }

    pub(crate) fn j_south(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<2>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn j_west(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<2>()
            .and(board.clone().shift_right::<0>().shift_up::<2>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn s_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<0>().shift_up::<0>());
        b1.and(b2)
    }

    pub(crate) fn s_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<2>());
        b1.and(b2)
    }

    pub(crate) fn z_north(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<2>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<1>().shift_up::<0>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        b1.and(b2)
    }

    pub(crate) fn z_east(board: BoardSimd) -> BoardSimd {
        let b1 = board
            .clone()
            .shift_right::<0>()
            .shift_up::<0>()
            .and(board.clone().shift_right::<0>().shift_up::<1>());
        let b2 = board
            .clone()
            .shift_right::<1>()
            .shift_up::<1>()
            .and(board.clone().shift_right::<1>().shift_up::<2>());
        b1.and(b2)
    }
}

#[cfg(test)]
mod tests {
    use crate::avx2::free::{free_spaces_each_pieces, load_free_space};
    use crate::boards::{Board16, Board64};
    use crate::pieces::Shape;
    use std::str::FromStr;

    #[test]
    fn test_free_spaces_by_pieces() {
        let board = Board64::from_str(
            "\
                ##.....###\
                ##....####\
                ##...#####\
                ##....####\
            ",
        ).unwrap();
        let board_simd = load_free_space(&board);
        let free_spaces = free_spaces_each_pieces(board_simd, Shape::T);
        let board = Board16::from(&free_spaces[0]);
        assert_eq!(board,
            Board16::from_str(
                "\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ..########\
                ....###...\
                ....##....\
                ....#.....\
                ....##....\
                ..........\
            ").unwrap()
        )
    }
}
