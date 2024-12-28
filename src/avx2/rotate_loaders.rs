use crate::avx2::free_space::FreeSpaceSimd16;
use crate::avx2::reachable::ReachableSimd16;
use crate::avx2::rotate::{ccw, cw};
use crate::pieces::{Orientation, Shape};
use crate::prelude::Piece;

pub fn rotate_cw(
    from_piece: Piece,
    src_reachable: &ReachableSimd16,
    dest_free_space: &FreeSpaceSimd16,
) -> ReachableSimd16 {
    match from_piece.shape {
        Shape::T => match from_piece.orientation {
            Orientation::North => cw::from_t_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_t_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_t_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_t_west(src_reachable, dest_free_space),
        },
        Shape::I => match from_piece.orientation {
            Orientation::North => cw::from_i_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_i_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_i_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_i_west(src_reachable, dest_free_space),
        },
        Shape::O => ReachableSimd16::blank(),
        Shape::L => match from_piece.orientation {
            Orientation::North => cw::from_l_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_l_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_l_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_l_west(src_reachable, dest_free_space),
        },
        Shape::J => match from_piece.orientation {
            Orientation::North => cw::from_j_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_j_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_j_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_j_west(src_reachable, dest_free_space),
        },
        Shape::S => match from_piece.orientation {
            Orientation::North => cw::from_s_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_s_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_s_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_s_west(src_reachable, dest_free_space),
        },
        Shape::Z => match from_piece.orientation {
            Orientation::North => cw::from_z_north(src_reachable, dest_free_space),
            Orientation::East => cw::from_z_east(src_reachable, dest_free_space),
            Orientation::South => cw::from_z_south(src_reachable, dest_free_space),
            Orientation::West => cw::from_z_west(src_reachable, dest_free_space),
        },
    }
}

pub fn rotate_ccw(
    from_piece: Piece,
    src_reachable: &ReachableSimd16,
    dest_free_space: &FreeSpaceSimd16,
) -> ReachableSimd16 {
    match from_piece.shape {
        Shape::T => match from_piece.orientation {
            Orientation::North => ccw::from_t_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_t_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_t_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_t_south(src_reachable, dest_free_space),
        },
        Shape::I => match from_piece.orientation {
            Orientation::North => ccw::from_i_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_i_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_i_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_i_south(src_reachable, dest_free_space),
        },
        Shape::O => ReachableSimd16::blank(),
        Shape::L => match from_piece.orientation {
            Orientation::North => ccw::from_l_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_l_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_l_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_l_south(src_reachable, dest_free_space),
        },
        Shape::J => match from_piece.orientation {
            Orientation::North => ccw::from_j_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_j_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_j_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_j_south(src_reachable, dest_free_space),
        },
        Shape::S => match from_piece.orientation {
            Orientation::North => ccw::from_s_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_s_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_s_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_s_south(src_reachable, dest_free_space),
        },
        Shape::Z => match from_piece.orientation {
            Orientation::North => ccw::from_z_west(src_reachable, dest_free_space),
            Orientation::East => ccw::from_z_north(src_reachable, dest_free_space),
            Orientation::South => ccw::from_z_east(src_reachable, dest_free_space),
            Orientation::West => ccw::from_z_south(src_reachable, dest_free_space),
        },
    }
}
