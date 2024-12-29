/** It's auto generated. */
use crate::internal_moves::u64::free_space::FreeSpace64;
use crate::pieces::Shape;

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
