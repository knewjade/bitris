/** It's auto generated. */
use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::pieces::{Shape, Orientation, Piece};

#[inline(always)]
pub fn to_free_spaces(free_space_block: FreeSpace64, shape: Shape) -> [FreeSpace64; 4] {
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
pub fn to_free_space(free_space_block: FreeSpace64, piece: Piece) -> FreeSpace64 {
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
fn t_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
fn t_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
fn t_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
fn t_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
fn i_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<2, 0, 0, 0>())
}

#[inline(always)]
fn i_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 0, 0, 2>())
}

#[inline(always)]
fn i_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 2, 0, 0>())
}

#[inline(always)]
fn i_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 0, 2, 0>())
}

#[inline(always)]
fn o_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
fn o_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
fn o_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
fn o_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
fn l_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
fn l_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
fn l_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
fn l_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
fn j_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 1, 0>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
fn j_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 1, 0>()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
fn j_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
fn j_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
fn s_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
fn s_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
fn s_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
fn s_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
fn z_north(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 1, 0>()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
fn z_east(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 1, 0>()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
fn z_south(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
fn z_west(space: FreeSpace64) -> FreeSpace64 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}
