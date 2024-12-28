/** It's auto generated. */

pub mod cw {
    use crate::avx2::free_space::FreeSpaceSimd16;
    use crate::avx2::reachable::ReachableSimd16;

    #[inline(always)]
    pub fn from_t_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 0)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, -2)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 1)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 2)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }
}

pub mod ccw {
    use crate::avx2::free_space::FreeSpaceSimd16;
    use crate::avx2::reachable::ReachableSimd16;

    #[inline(always)]
    pub fn from_t_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_t_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 0)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, -2)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 1)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 2, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-2, 2)
        let shift_forward = src_candidates.clone().jump_and::<2, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_i_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<2, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (2, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 2, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_l_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_j_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_s_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_north(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_east(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, -1)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<1, 0, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (1, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 1, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_south(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 1>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 1, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, -2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 2>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 2, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }

    #[inline(always)]
    pub fn from_z_west(src_reachable: &ReachableSimd16, dest_free_space: &FreeSpaceSimd16) -> ReachableSimd16 {
        debug_assert!(!src_reachable.empty());

        let src_candidates = src_reachable.clone();
        let dest_reachable = ReachableSimd16::blank();

        // Kick (0, 0)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 0)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, -1)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 1, 0>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 1, 0, 1>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (0, 2)
        let shift_forward = src_candidates.clone().jump_and::<0, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        let src_candidates = src_candidates.jump_rev::<0, 0, 2, 0>(shift_forward);
        if src_candidates.empty() {
            return dest_reachable;
        }
        
        // Kick (-1, 2)
        let shift_forward = src_candidates.clone().jump_and::<1, 0, 0, 2>(dest_free_space);
        let dest_reachable = dest_reachable.or(&shift_forward);
        dest_reachable
    }
}
