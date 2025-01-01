use crate::boards::Board;
use crate::internal_moves::avx2::moves::Moves4;
use crate::pieces::Orientation;
use crate::placements::CcPlacement;

#[inline(always)]
pub fn moves_softdrop_with_rotation<const MINIMIZE: bool>(
    board: &Board<u64>,
    spawn: CcPlacement,
) -> Moves4 {
    todo!()
}
