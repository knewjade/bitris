use crate::{Kick, KickTable, Offset, Piece, Rotation, Shape};

macro_rules! k {
    ($dx: expr, $dy: expr) => {
        Kick { offset: Offset { dx: $dx, dy: $dy } }
    }
}

/// Kick table with SRS defined.
#[derive(Copy, Clone, Hash, Debug, Default)]
pub struct SrsKickTable;

impl SrsKickTable {
    const LJSZ_NE: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1, 1), k!(0,-2), k!(-1,-2)];
    const LJSZ_ES: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1,-1), k!(0, 2), k!( 1, 2)];
    const LJSZ_SW: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1, 1), k!(0,-2), k!( 1,-2)];
    const LJSZ_WN: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1,-1), k!(0, 2), k!(-1, 2)];

    const LJSZ_NW: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1, 1), k!(0,-2), k!( 1,-2)];
    const LJSZ_WS: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1,-1), k!(0, 2), k!(-1, 2)];
    const LJSZ_SE: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1, 1), k!(0,-2), k!(-1,-2)];
    const LJSZ_EN: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1,-1), k!(0, 2), k!( 1, 2)];

    const T_NE: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1, 1), k!(0,-2), k!(-1,-2)];
    const T_ES: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1,-1), k!(0, 2), k!( 1, 2)];
    const T_SW: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1, 1), k!(0,-2), k!( 1,-2)];
    const T_WN: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1,-1), k!(0, 2), k!(-1, 2)];

    const T_NW: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1, 1), k!(0,-2), k!( 1,-2)];
    const T_WS: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1,-1), k!(0, 2), k!(-1, 2)];
    const T_SE: [Kick; 5] = [k!(0,0), k!(-1,0), k!(-1, 1), k!(0,-2), k!(-1,-2)];
    const T_EN: [Kick; 5] = [k!(0,0), k!( 1,0), k!( 1,-1), k!(0, 2), k!( 1, 2)];

    const I_NE: [Kick; 5] = [k!( 1, 0), k!(-1, 0), k!( 2, 0), k!(-1,-1), k!( 2, 2)];
    const I_ES: [Kick; 5] = [k!( 0,-1), k!(-1,-1), k!( 2,-1), k!(-1, 1), k!( 2,-2)];
    const I_SW: [Kick; 5] = [k!(-1, 0), k!( 1, 0), k!(-2, 0), k!( 1, 1), k!(-2,-2)];
    const I_WN: [Kick; 5] = [k!( 0, 1), k!( 1, 1), k!(-2, 1), k!( 1,-1), k!(-2, 2)];

    const I_NW: [Kick; 5] = [k!( 0,-1), k!(-1,-1), k!( 2,-1), k!(-1, 1), k!( 2,-2)];
    const I_WS: [Kick; 5] = [k!( 1, 0), k!(-1, 0), k!( 2, 0), k!(-1,-1), k!( 2, 2)];
    const I_SE: [Kick; 5] = [k!( 0, 1), k!( 1, 1), k!(-2, 1), k!( 1,-1), k!(-2, 2)];
    const I_EN: [Kick; 5] = [k!(-1, 0), k!( 1, 0), k!(-2, 0), k!( 1, 1), k!(-2,-2)];

    const O_NE: [Kick; 1] = [k!( 0, 1)];
    const O_ES: [Kick; 1] = [k!( 1, 0)];
    const O_SW: [Kick; 1] = [k!( 0,-1)];
    const O_WN: [Kick; 1] = [k!(-1, 0)];

    const O_NW: [Kick; 1] = [k!( 1, 0)];
    const O_WS: [Kick; 1] = [k!( 0, 1)];
    const O_SE: [Kick; 1] = [k!(-1, 0)];
    const O_EN: [Kick; 1] = [k!( 0,-1)];

    const EMPTY: [Kick; 0] = [];

    const LJSZ_KICKS: [&'static [Kick]; 12] = [
        // from North
        &Self::LJSZ_NE,
        &Self::LJSZ_NW,
        &Self::EMPTY,
        // from East
        &Self::LJSZ_ES,
        &Self::LJSZ_EN,
        &Self::EMPTY,
        // from South
        &Self::LJSZ_SW,
        &Self::LJSZ_SE,
        &Self::EMPTY,
        // from West
        &Self::LJSZ_WN,
        &Self::LJSZ_WS,
        &Self::EMPTY,
    ];
    const T_KICKS: [&'static [Kick]; 12] = [
        // from North
        &Self::T_NE,
        &Self::T_NW,
        &Self::EMPTY,
        // from East
        &Self::T_ES,
        &Self::T_EN,
        &Self::EMPTY,
        // from South
        &Self::T_SW,
        &Self::T_SE,
        &Self::EMPTY,
        // from West
        &Self::T_WN,
        &Self::T_WS,
        &Self::EMPTY,
    ];
    const I_KICKS: [&'static [Kick]; 12] = [
        // from North
        &Self::I_NE,
        &Self::I_NW,
        &Self::EMPTY,
        // from East
        &Self::I_ES,
        &Self::I_EN,
        &Self::EMPTY,
        // from South
        &Self::I_SW,
        &Self::I_SE,
        &Self::EMPTY,
        // from West
        &Self::I_WN,
        &Self::I_WS,
        &Self::EMPTY,
    ];
    const O_KICKS: [&'static [Kick]; 12] = [
        // from North
        &Self::O_NE,
        &Self::O_NW,
        &Self::EMPTY,
        // from East
        &Self::O_ES,
        &Self::O_EN,
        &Self::EMPTY,
        // from South
        &Self::O_SW,
        &Self::O_SE,
        &Self::EMPTY,
        // from West
        &Self::O_WN,
        &Self::O_WS,
        &Self::EMPTY,
    ];
}

impl KickTable<'static> for SrsKickTable {
    fn get_kicks(&self, oriented_shape: Piece, rotation: Rotation) -> &'static [Kick] {
        assert_ne!(rotation, Rotation::R180);
        let index = oriented_shape.orientation as usize * 3 + rotation as usize;
        match oriented_shape.shape {
            Shape::L | Shape::J | Shape::S | Shape::Z => Self::LJSZ_KICKS[index],
            Shape::T => Self::T_KICKS[index],
            Shape::I => Self::I_KICKS[index],
            Shape::O => Self::O_KICKS[index],
        }
    }

    fn is_moving_in_rotation(&self, shape: Shape) -> bool {
        shape != Shape::O
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use crate::*;
    use crate::srs::*;

    #[test]
    fn srs_t_from_north_to_east() {
        let kicks = SrsKickTable.get_kicks(Piece::new(Shape::T, Orientation::North), Rotation::Cw);
        assert_equal(
            kicks.iter().map(|it| it.offset),
            vec![dd(0, 0), dd(-1, 0), dd(-1, 1), dd(0, -2), dd(-1, -2)].into_iter(),
        );
    }

    #[test]
    #[should_panic]
    fn srs_is_not_unsupported_rotate_180() {
        SrsKickTable.get_kicks(Piece::new(Shape::T, Orientation::North), Rotation::R180);
    }
}
