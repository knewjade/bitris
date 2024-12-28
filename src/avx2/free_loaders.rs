use crate::avx2::aligned::AlignedU8s;
use crate::avx2::free_space::FreeSpaceSimd16;
use crate::avx2::free;
use crate::boards::Board;
use crate::pieces::Shape;

// ブロックと空を反転して、下位16bitを読み込み
pub(crate) fn load_free_space(board: &Board<u64>) -> FreeSpaceSimd16 {
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

    FreeSpaceSimd16::from(AlignedU8s::new(bytes_u8))
}

pub(crate) fn free_spaces_each_pieces(
    free_space_block: FreeSpaceSimd16, shape: Shape,
) -> [FreeSpaceSimd16; 4] {
    match shape {
        Shape::T => [
            free::t_north(free_space_block.clone()),
            free::t_east(free_space_block.clone()),
            free::t_south(free_space_block.clone()),
            free::t_west(free_space_block.clone()),
        ],
        Shape::I => {
            let north = free::i_north(free_space_block.clone());
            let east = free::i_east(free_space_block.clone());
            [north.clone(), east.clone(), north, east]
        }
        Shape::O => {
            let north = free::o_north(free_space_block.clone());
            [north.clone(), north.clone(), north.clone(), north]
        }
        Shape::L => [
            free::l_north(free_space_block.clone()),
            free::l_east(free_space_block.clone()),
            free::l_south(free_space_block.clone()),
            free::l_west(free_space_block.clone()),
        ],
        Shape::J => [
            free::j_north(free_space_block.clone()),
            free::j_east(free_space_block.clone()),
            free::j_south(free_space_block.clone()),
            free::j_west(free_space_block.clone()),
        ],
        Shape::S => {
            let north = free::s_north(free_space_block.clone());
            let east = free::s_east(free_space_block.clone());
            [north.clone(), east.clone(), north, east]
        }
        Shape::Z => {
            let north = free::z_north(free_space_block.clone());
            let east = free::z_east(free_space_block.clone());
            [north.clone(), east.clone(), north, east]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::avx2::free_loaders::{free_spaces_each_pieces, load_free_space};
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
