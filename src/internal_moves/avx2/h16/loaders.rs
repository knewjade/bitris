use crate::boards::Board;
use crate::internal_moves::avx2::h16::aligned::AlignedU16s;
use crate::internal_moves::avx2::h16::free;
use crate::internal_moves::avx2::h16::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::h16::reachable::ReachableSimd16;
use crate::pieces::{Piece, Shape};
use crate::placements::CcPlacement;

#[inline(always)]
pub fn to_free_spaces_lower(board: &Board<u64>, shape: Shape) -> [FreeSpaceSimd16; 4] {
    let free_space_block_lower = to_free_space_block_lower(board);
    free::to_free_spaces(free_space_block_lower, shape)
}

#[inline(always)]
pub fn to_free_space_lower(board: &Board<u64>, piece: Piece) -> FreeSpaceSimd16 {
    let free_space_block_lower = to_free_space_block_lower(board);
    free::to_free_space(free_space_block_lower, piece)
}

#[inline(always)]
fn to_free_space_block_lower(board: &Board<u64>) -> FreeSpaceSimd16 {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_16: [u16; 16] = [
        bytes_u64[0] as u16,
        bytes_u64[1] as u16,
        bytes_u64[2] as u16,
        bytes_u64[3] as u16,
        bytes_u64[4] as u16,
        bytes_u64[5] as u16,
        bytes_u64[6] as u16,
        bytes_u64[7] as u16,
        bytes_u64[8] as u16,
        bytes_u64[9] as u16,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    FreeSpaceSimd16::from(AlignedU16s::new(bytes_16))
}

#[inline(always)]
pub fn spawn_and_harddrop_reachables(
    spawn: CcPlacement,
    free_spaces: &[FreeSpaceSimd16; 4],
) -> [ReachableSimd16; 4] {
    if spawn.position.cy < 0 || 16 <= spawn.position.cy {
        return [
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
            ReachableSimd16::blank(),
        ];
    }

    let mut aligneds = [
        AlignedU16s::blank(),
        AlignedU16s::blank(),
        AlignedU16s::blank(),
        AlignedU16s::blank(),
    ];

    // spawn
    let mut spawn_aligned =
        spawn_and_harddrop_aligned(spawn, &free_spaces[spawn.piece.orientation as usize]);

    std::mem::swap(
        &mut aligneds[spawn.piece.orientation as usize],
        &mut spawn_aligned,
    );

    aligneds.map(|aligned| ReachableSimd16::from(aligned))
}

#[inline(always)]
pub fn spawn_and_harddrop_reachable(
    spawn: CcPlacement,
    free_space: &FreeSpaceSimd16,
) -> ReachableSimd16 {
    if spawn.position.cy < 0 || 16 <= spawn.position.cy {
        return ReachableSimd16::blank();
    }
    ReachableSimd16::from(spawn_and_harddrop_aligned(spawn, free_space))
}

#[inline(always)]
pub fn spawn_and_harddrop_aligned(spawn: CcPlacement, free_space: &FreeSpaceSimd16) -> AlignedU16s {
    // index
    let spawn_location = spawn.position.to_location();
    let spawn_x = spawn_location.x as usize;

    // boards
    let mut spawn_reachable = AlignedU16s::blank();

    // a spawn bit
    let spawn_bit = 1u16 << spawn_location.y;

    // 1-mask over spawn y
    let mask = u16::MAX - ((1u16 << (spawn_location.y + 1)) - 1);

    // harddrop
    {
        let x = spawn_x;
        let free_space = free_space.col(x);

        // harddrop
        let harddrop_dest_y = 16 - (!(free_space | mask)).leading_zeros();
        if harddrop_dest_y <= spawn_location.y as u32 {
            let reachable = (spawn_bit - 1) - ((1 << harddrop_dest_y) - 1);
            spawn_reachable.set_u16(x, spawn_bit | reachable);
        }
    }

    spawn_reachable
}

#[inline(always)]
pub fn to_bytes_u32(
    reachable_lower: &ReachableSimd16,
    reachable_upper: &ReachableSimd16,
) -> [u32; 10] {
    let lower = reachable_lower.to_bytes_u32();
    let upper = reachable_upper.to_bytes_u32();
    [
        lower[0] | (upper[0] << 12),
        lower[1] | (upper[1] << 12),
        lower[2] | (upper[2] << 12),
        lower[3] | (upper[3] << 12),
        lower[4] | (upper[4] << 12),
        lower[5] | (upper[5] << 12),
        lower[6] | (upper[6] << 12),
        lower[7] | (upper[7] << 12),
        lower[8] | (upper[8] << 12),
        lower[9] | (upper[9] << 12),
    ]
}

pub fn can_reach4(reachables: &[ReachableSimd16; 4], goals: &[CcPlacement]) -> bool {
    goals.iter().any(|&goal_placement| {
        let orientation_index = goal_placement.piece.orientation as usize;
        let location = goal_placement.position.to_location();
        reachables[orientation_index].is_visited(location)
    })
}

pub fn can_reach1(reachable: &ReachableSimd16, goal: CcPlacement) -> bool {
    let location = goal.position.to_location();
    reachable.is_visited(location)
}
