/** It's auto generated. */
use crate::avx2::free_space::FreeSpaceSimd16;

#[inline(always)]
pub fn t_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
pub fn t_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn t_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
pub fn t_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
pub fn i_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 2, 0, 0>())
}

#[inline(always)]
pub fn i_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 0, 2, 0>())
}

#[inline(always)]
pub fn o_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn l_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn l_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
pub fn l_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.shift::<1, 0, 1, 0>())
}

#[inline(always)]
pub fn l_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 1, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<1, 0, 0, 1>())
}

#[inline(always)]
pub fn j_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<1, 0, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn j_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}

#[inline(always)]
pub fn j_south(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 1, 0>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<1, 0, 0, 0>())
}

#[inline(always)]
pub fn j_west(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 1, 0>()
        .and(space.clone().shift::<0, 0, 1, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 0, 1>())
}

#[inline(always)]
pub fn s_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 0>()
        .and(space.clone())
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.shift::<0, 1, 0, 1>())
}

#[inline(always)]
pub fn s_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 0, 0, 1>()
        .and(space.clone())
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.shift::<0, 1, 1, 0>())
}

#[inline(always)]
pub fn z_north(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<1, 0, 0, 1>()
        .and(space.clone().shift::<0, 0, 0, 1>())
        .and(space.clone())
        .and(space.shift::<0, 1, 0, 0>())
}

#[inline(always)]
pub fn z_east(space: FreeSpaceSimd16) -> FreeSpaceSimd16 {
    space.clone().shift::<0, 1, 0, 1>()
        .and(space.clone().shift::<0, 1, 0, 0>())
        .and(space.clone())
        .and(space.shift::<0, 0, 1, 0>())
}
