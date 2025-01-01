/*! It's auto generated. */
use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
use crate::pieces::{Orientation, Piece, Shape};

#[inline(always)]
pub fn to_free_spaces(free_space_block: FreeSpaceSimd16, shape: Shape) -> [FreeSpaceSimd16; 4] {
    match shape {
        Shape::T => [
            t_north(free_space_block.clone()),
            t_east(free_space_block.clone()),
            t_south(free_space_block.clone()),
            t_west(free_space_block),
        ],
        Shape::I => [
            i_north(free_space_block.clone()),
            i_east(free_space_block.clone()),
            i_south(free_space_block.clone()),
            i_west(free_space_block),
        ],
        Shape::O => [
            o_north(free_space_block.clone()),
            o_east(free_space_block.clone()),
            o_south(free_space_block.clone()),
            o_west(free_space_block),
        ],
        Shape::L => [
            l_north(free_space_block.clone()),
            l_east(free_space_block.clone()),
            l_south(free_space_block.clone()),
            l_west(free_space_block),
        ],
        Shape::J => [
            j_north(free_space_block.clone()),
            j_east(free_space_block.clone()),
            j_south(free_space_block.clone()),
            j_west(free_space_block),
        ],
        Shape::S => [
            s_north(free_space_block.clone()),
            s_east(free_space_block.clone()),
            s_south(free_space_block.clone()),
            s_west(free_space_block),
        ],
        Shape::Z => [
            z_north(free_space_block.clone()),
            z_east(free_space_block.clone()),
            z_south(free_space_block.clone()),
            z_west(free_space_block),
        ],
    }
}

#[inline(always)]
pub fn to_free_space(free_space_block: FreeSpaceSimd16, piece: Piece) -> FreeSpaceSimd16 {
    match piece.shape {
        Shape::T => {
            match piece.orientation {
                Orientation::North => t_north(free_space_block.clone()),
                Orientation::East => t_east(free_space_block.clone()),
                Orientation::South => t_south(free_space_block.clone()),
                Orientation::West => t_west(free_space_block.clone()),
            }
        },
        Shape::I => {
            match piece.orientation {
                Orientation::North => i_north(free_space_block.clone()),
                Orientation::East => i_east(free_space_block.clone()),
                Orientation::South => i_south(free_space_block.clone()),
                Orientation::West => i_west(free_space_block.clone()),
            }
        },
        Shape::O => {
            match piece.orientation {
                Orientation::North => o_north(free_space_block.clone()),
                Orientation::East => o_east(free_space_block.clone()),
                Orientation::South => o_south(free_space_block.clone()),
                Orientation::West => o_west(free_space_block.clone()),
            }
        },
        Shape::L => {
            match piece.orientation {
                Orientation::North => l_north(free_space_block.clone()),
                Orientation::East => l_east(free_space_block.clone()),
                Orientation::South => l_south(free_space_block.clone()),
                Orientation::West => l_west(free_space_block.clone()),
            }
        },
        Shape::J => {
            match piece.orientation {
                Orientation::North => j_north(free_space_block.clone()),
                Orientation::East => j_east(free_space_block.clone()),
                Orientation::South => j_south(free_space_block.clone()),
                Orientation::West => j_west(free_space_block.clone()),
            }
        },
        Shape::S => {
            match piece.orientation {
                Orientation::North => s_north(free_space_block.clone()),
                Orientation::East => s_east(free_space_block.clone()),
                Orientation::South => s_south(free_space_block.clone()),
                Orientation::West => s_west(free_space_block.clone()),
            }
        },
        Shape::Z => {
            match piece.orientation {
                Orientation::North => z_north(free_space_block.clone()),
                Orientation::East => z_east(free_space_block.clone()),
                Orientation::South => z_south(free_space_block.clone()),
                Orientation::West => z_west(free_space_block.clone()),
            }
        },
    }
}

#[inline(always)]
pub fn t_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
pub fn t_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
pub fn t_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
pub fn t_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn i_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<2, 0, 0, 0>())
}

#[inline(always)]
pub fn i_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 0, 0, 2>())
}

#[inline(always)]
pub fn i_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 2, 0, 0>())
}

#[inline(always)]
pub fn i_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 0, 2, 0>())
}

#[inline(always)]
pub fn o_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
pub fn o_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
pub fn o_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn o_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
pub fn l_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
pub fn l_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
pub fn l_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn l_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
pub fn j_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 1, 0>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
pub fn j_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 1, 0>()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
pub fn j_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn j_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
pub fn s_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
pub fn s_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
pub fn s_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn s_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
pub fn z_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 1, 0>()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
pub fn z_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 1, 0>()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
pub fn z_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn z_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}
