/*! It's auto generated. */
use crate::internal_moves::avx2::reachable::ReachableSimd16;
use crate::pieces::Shape;

#[inline(always)]
pub fn minimize(mut reachables: [ReachableSimd16; 4], shape: Shape) -> [ReachableSimd16; 4] {
    match shape {
        Shape::T => reachables,
        Shape::I => [
            reachables[0].clone().or_shift::<1, 0, 0, 0>(&reachables[2]),
            reachables[1].clone().or_shift::<0, 0, 0, 1>(&reachables[3]),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
        ],
        Shape::O => [
            reachables[0]
                .clone()
                .or_shift::<0, 0, 1, 0>(&reachables[1])
                .or_shift::<1, 0, 1, 0>(&reachables[2])
                .or_shift::<1, 0, 0, 0>(&reachables[3]),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
        ],
        Shape::L => reachables,
        Shape::J => reachables,
        Shape::S => [
            reachables[0].clone().or_shift::<0, 0, 1, 0>(&reachables[2]),
            reachables[1].clone().or_shift::<1, 0, 0, 0>(&reachables[3]),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
        ],
        Shape::Z => [
            reachables[0].clone().or_shift::<0, 0, 1, 0>(&reachables[2]),
            reachables[1].clone().or_shift::<1, 0, 0, 0>(&reachables[3]),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
        ],
    }
}
