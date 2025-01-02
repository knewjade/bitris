use crate::boards::Board;
use crate::internal_moves::avx2::h24::aligned::AlignedU24s;
use crate::internal_moves::avx2::h24::free;
use crate::internal_moves::avx2::h24::free_space::FreeSpaceSimd24;
use crate::internal_moves::avx2::h24::reachable::ReachableSimd24;
use crate::pieces::{Piece, Shape};
use crate::placements::CcPlacement;

#[inline(always)]
pub fn to_free_spaces(board: &Board<u64>, shape: Shape) -> [FreeSpaceSimd24; 4] {
    let free_space_block = to_free_space_block(board);
    free::to_free_spaces(free_space_block, shape)
}

#[inline(always)]
pub fn to_free_space(board: &Board<u64>, piece: Piece) -> FreeSpaceSimd24 {
    let free_space_block_lower = to_free_space_block(board);
    free::to_free_space(free_space_block_lower, piece)
}

#[inline(always)]
fn to_free_space_block(board: &Board<u64>) -> FreeSpaceSimd24 {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_u8: [u8; 32] = [
        bytes_u64[0] as u8,
        (bytes_u64[0] >> 8) as u8,
        (bytes_u64[0] >> 16) as u8,
        bytes_u64[1] as u8,
        (bytes_u64[1] >> 8) as u8,
        (bytes_u64[1] >> 16) as u8,
        bytes_u64[2] as u8,
        (bytes_u64[2] >> 8) as u8,
        (bytes_u64[2] >> 16) as u8,
        bytes_u64[3] as u8,
        (bytes_u64[3] >> 8) as u8,
        (bytes_u64[3] >> 16) as u8,
        bytes_u64[4] as u8,
        (bytes_u64[4] >> 8) as u8,
        (bytes_u64[4] >> 16) as u8,
        bytes_u64[5] as u8,
        (bytes_u64[5] >> 8) as u8,
        (bytes_u64[5] >> 16) as u8,
        bytes_u64[6] as u8,
        (bytes_u64[6] >> 8) as u8,
        (bytes_u64[6] >> 16) as u8,
        bytes_u64[7] as u8,
        (bytes_u64[7] >> 8) as u8,
        (bytes_u64[7] >> 16) as u8,
        bytes_u64[8] as u8,
        (bytes_u64[8] >> 8) as u8,
        (bytes_u64[8] >> 16) as u8,
        bytes_u64[9] as u8,
        (bytes_u64[9] >> 8) as u8,
        (bytes_u64[9] >> 16) as u8,
        0,
        0,
    ];

    FreeSpaceSimd24::from(AlignedU24s::new(bytes_u8))
}

#[inline(always)]
pub fn spawn_and_harddrop_reachables(
    spawn: CcPlacement,
    free_spaces: &[FreeSpaceSimd24; 4],
) -> [ReachableSimd24; 4] {
    if spawn.position.cy < 0 || 24 <= spawn.position.cy {
        return [
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
        ];
    }

    let mut aligneds = [
        AlignedU24s::blank(),
        AlignedU24s::blank(),
        AlignedU24s::blank(),
        AlignedU24s::blank(),
    ];

    // spawn
    let mut spawn_aligned =
        spawn_and_harddrop_aligned(spawn, &free_spaces[spawn.piece.orientation as usize]);

    std::mem::swap(
        &mut aligneds[spawn.piece.orientation as usize],
        &mut spawn_aligned,
    );

    aligneds.map(|aligned| ReachableSimd24::from(aligned))
}

#[inline(always)]
pub fn spawn_and_harddrop_reachable(
    spawn: CcPlacement,
    free_space: &FreeSpaceSimd24,
) -> ReachableSimd24 {
    if spawn.position.cy < 0 || 24 <= spawn.position.cy {
        return ReachableSimd24::blank();
    }
    ReachableSimd24::from(spawn_and_harddrop_aligned(spawn, free_space))
}

#[inline(always)]
pub fn spawn_and_harddrop_aligned(spawn: CcPlacement, free_space: &FreeSpaceSimd24) -> AlignedU24s {
    // index
    let spawn_location = spawn.position.to_location();
    let spawn_x = spawn_location.x as usize;

    // boards
    let mut spawn_reachable = AlignedU24s::blank();

    // a spawn bit
    let spawn_bit = 1u32 << spawn_location.y;

    // 1-mask over spawn y
    let mask = u32::MAX - ((1u32 << (spawn_location.y + 1)) - 1);

    // harddrop
    {
        let x = spawn_x;
        let free_space = free_space.col(x);

        // harddrop
        let harddrop_dest_y = 32 - (!(free_space | mask)).leading_zeros();
        if harddrop_dest_y <= spawn_location.y as u32 {
            let reachable = (spawn_bit - 1) - ((1 << harddrop_dest_y) - 1);
            spawn_reachable.set_u32(x, spawn_bit | reachable);
        }
    }

    spawn_reachable
}

pub fn can_reach4(reachables: &[ReachableSimd24; 4], goals: &[CcPlacement]) -> bool {
    goals.iter().any(|&goal_placement| {
        let orientation_index = goal_placement.piece.orientation as usize;
        let location = goal_placement.position.to_location();
        reachables[orientation_index].is_visited(location)
    })
}

pub fn can_reach1(reachable: &ReachableSimd24, goal: CcPlacement) -> bool {
    let location = goal.position.to_location();
    reachable.is_visited(location)
}
