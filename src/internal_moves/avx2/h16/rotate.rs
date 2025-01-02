/*! It's auto generated. */
use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;
use crate::pieces::{Orientation, Piece, Shape};

#[inline(always)]
pub fn rotate_cw(from_piece: Piece, src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
    debug_assert!(from_piece.shape != Shape::O);

    match from_piece.shape {
        Shape::T => {
            match from_piece.orientation {
                Orientation::North => cw::from_t_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_t_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_t_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_t_west(src_reachable, dest_free_space),
            }
        }
        Shape::I => {
            match from_piece.orientation {
                Orientation::North => cw::from_i_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_i_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_i_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_i_west(src_reachable, dest_free_space),
            }
        }
        Shape::O => ReachableSimd16::blank(),
        Shape::L => {
            match from_piece.orientation {
                Orientation::North => cw::from_l_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_l_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_l_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_l_west(src_reachable, dest_free_space),
            }
        }
        Shape::J => {
            match from_piece.orientation {
                Orientation::North => cw::from_j_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_j_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_j_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_j_west(src_reachable, dest_free_space),
            }
        }
        Shape::S => {
            match from_piece.orientation {
                Orientation::North => cw::from_s_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_s_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_s_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_s_west(src_reachable, dest_free_space),
            }
        }
        Shape::Z => {
            match from_piece.orientation {
                Orientation::North => cw::from_z_north(src_reachable, dest_free_space),
                Orientation::East => cw::from_z_east(src_reachable, dest_free_space),
                Orientation::South => cw::from_z_south(src_reachable, dest_free_space),
                Orientation::West => cw::from_z_west(src_reachable, dest_free_space),
            }
        }
    }
}

#[inline(always)]
pub fn rotate_ccw(from_piece: Piece, src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
    debug_assert!(from_piece.shape != Shape::O);

    match from_piece.shape {
        Shape::T => {
            match from_piece.orientation {
                Orientation::North => ccw::from_t_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_t_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_t_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_t_west(src_reachable, dest_free_space),
            }
        }
        Shape::I => {
            match from_piece.orientation {
                Orientation::North => ccw::from_i_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_i_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_i_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_i_west(src_reachable, dest_free_space),
            }
        }
        Shape::O => ReachableSimd16::blank(),
        Shape::L => {
            match from_piece.orientation {
                Orientation::North => ccw::from_l_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_l_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_l_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_l_west(src_reachable, dest_free_space),
            }
        }
        Shape::J => {
            match from_piece.orientation {
                Orientation::North => ccw::from_j_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_j_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_j_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_j_west(src_reachable, dest_free_space),
            }
        }
        Shape::S => {
            match from_piece.orientation {
                Orientation::North => ccw::from_s_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_s_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_s_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_s_west(src_reachable, dest_free_space),
            }
        }
        Shape::Z => {
            match from_piece.orientation {
                Orientation::North => ccw::from_z_north(src_reachable, dest_free_space),
                Orientation::East => ccw::from_z_east(src_reachable, dest_free_space),
                Orientation::South => ccw::from_z_south(src_reachable, dest_free_space),
                Orientation::West => ccw::from_z_west(src_reachable, dest_free_space),
            }
        }
    }
}

pub mod cw {
    use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
    use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;

    #[inline(always)]
    pub fn from_t_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, 0)
        let shift_forward = src_candidates.clone().jump::<0, 2, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, 2)
        let shift_forward = src_candidates.clone().jump::<0, 2, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, -1)
        let shift_forward = src_candidates.clone().jump::<0, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, -1)
        let shift_forward = src_candidates.clone().jump::<0, 2, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, -2)
        let shift_forward = src_candidates.clone().jump::<0, 2, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 0)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, -2)
        let shift_forward = src_candidates.clone().jump::<2, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 1)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 1)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 2)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }
}

pub mod ccw {
    use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
    use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;

    #[inline(always)]
    pub fn from_t_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_t_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, -1)
        let shift_forward = src_candidates.clone().jump::<0, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, -1)
        let shift_forward = src_candidates.clone().jump::<0, 2, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, -2)
        let shift_forward = src_candidates.clone().jump::<0, 2, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 0)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, -2)
        let shift_forward = src_candidates.clone().jump::<2, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 1)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 1)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-2, 2)
        let shift_forward = src_candidates.clone().jump::<2, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_i_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, 0)
        let shift_forward = src_candidates.clone().jump::<0, 2, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (2, 2)
        let shift_forward = src_candidates.clone().jump::<0, 2, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_l_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_j_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_s_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump::<0, 1, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump::<0, 1, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 1>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 2, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }

    #[inline(always)]
    pub fn from_z_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump::<1, 0, 1, 0>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump::<0, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(dest_free_space);
        if src_candidates.empty() {
            return dest_reachable.and(dest_free_space);
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump::<1, 0, 0, 2>();
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable.and(dest_free_space)
    }
}
