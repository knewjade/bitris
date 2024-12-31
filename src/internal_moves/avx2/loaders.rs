use crate::boards::Board;
use crate::internal_moves::avx2::aligned::AlignedU8s;
use crate::internal_moves::avx2::free_space::FreeSpaceSimd16;
use crate::internal_moves::avx2::reachable::ReachableSimd16;
use crate::internal_moves::avx2::free;
use crate::pieces::{Piece, Shape};
use crate::placements::CcPlacement;

// ブロックと空を反転して、下位16bitを読み込み
pub fn to_free_spaces_lower(board: &Board<u64>, shape: Shape) -> [FreeSpaceSimd16; 4] {
    let free_space_block_lower = to_free_space_block_lower(board);
    free::to_free_spaces(free_space_block_lower, shape)
}

pub fn to_free_space_lower(board: &Board<u64>, piece: Piece) -> FreeSpaceSimd16 {
    let free_space_block_lower = to_free_space_block_lower(board);
    free::to_free_space(free_space_block_lower, piece)
}

fn to_free_space_block_lower(board: &Board<u64>) -> FreeSpaceSimd16 {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_u8: [u8; 32] = [
        bytes_u64[0] as u8,
        (bytes_u64[0] >> 8) as u8,
        bytes_u64[1] as u8,
        (bytes_u64[1] >> 8) as u8,
        bytes_u64[2] as u8,
        (bytes_u64[2] >> 8) as u8,
        bytes_u64[3] as u8,
        (bytes_u64[3] >> 8) as u8,
        bytes_u64[4] as u8,
        (bytes_u64[4] >> 8) as u8,
        bytes_u64[5] as u8,
        (bytes_u64[5] >> 8) as u8,
        bytes_u64[6] as u8,
        (bytes_u64[6] >> 8) as u8,
        bytes_u64[7] as u8,
        (bytes_u64[7] >> 8) as u8,
        bytes_u64[8] as u8,
        (bytes_u64[8] >> 8) as u8,
        bytes_u64[9] as u8,
        (bytes_u64[9] >> 8) as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    let free_space_block = FreeSpaceSimd16::from(AlignedU8s::new(bytes_u8));
    free_space_block
}

// ブロックと空を反転して、下位12bitをスキップして、16bitを読み込み
pub fn to_free_spaces_upper(board: &Board<u64>, shape: Shape) -> [FreeSpaceSimd16; 4] {
    let free_space_block = free_space_block_upper(board);
    free::to_free_spaces(free_space_block, shape)
}

pub fn to_free_space_upper(board: &Board<u64>, piece: Piece) -> FreeSpaceSimd16 {
    let free_space_block_upper = free_space_block_upper(board);
    free::to_free_space(free_space_block_upper, piece)
}

fn free_space_block_upper(board: &Board<u64>) -> FreeSpaceSimd16 {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_u8: [u8; 32] = [
        (bytes_u64[0] >> 12) as u8,
        (bytes_u64[0] >> 20) as u8,
        (bytes_u64[1] >> 12) as u8,
        (bytes_u64[1] >> 20) as u8,
        (bytes_u64[2] >> 12) as u8,
        (bytes_u64[2] >> 20) as u8,
        (bytes_u64[3] >> 12) as u8,
        (bytes_u64[3] >> 20) as u8,
        (bytes_u64[4] >> 12) as u8,
        (bytes_u64[4] >> 20) as u8,
        (bytes_u64[5] >> 12) as u8,
        (bytes_u64[5] >> 20) as u8,
        (bytes_u64[6] >> 12) as u8,
        (bytes_u64[6] >> 20) as u8,
        (bytes_u64[7] >> 12) as u8,
        (bytes_u64[7] >> 20) as u8,
        (bytes_u64[8] >> 12) as u8,
        (bytes_u64[8] >> 20) as u8,
        (bytes_u64[9] >> 12) as u8,
        (bytes_u64[9] >> 20) as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];

    let free_space_block = FreeSpaceSimd16::from(AlignedU8s::new(bytes_u8));
    free_space_block
}

// #[inline(always)]
// pub fn spawn_reachables(spawn: CcPlacement) -> [ReachableSimd16; 4] {
//     if spawn.position.cy < 0 || 16 <= spawn.position.cy {
//         return [
//             ReachableSimd16::blank(),
//             ReachableSimd16::blank(),
//             ReachableSimd16::blank(),
//             ReachableSimd16::blank(),
//         ];
//     }
//
//     let mut aligneds = [
//         AlignedU8s::blank(),
//         AlignedU8s::blank(),
//         AlignedU8s::blank(),
//         AlignedU8s::blank(),
//     ];
//
//     aligneds[spawn.piece.orientation as usize] =
//         AlignedU8s::blank().set_at(spawn.position.to_location());
//
//     aligneds.map(|aligned| ReachableSimd16::from(aligned))
// }

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
        AlignedU8s::blank(),
        AlignedU8s::blank(),
        AlignedU8s::blank(),
        AlignedU8s::blank(),
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
pub fn spawn_and_harddrop_aligned(spawn: CcPlacement, free_space: &FreeSpaceSimd16) -> AlignedU8s {
    // index
    let spawn_location = spawn.position.to_location();
    let spawn_x = spawn_location.x as usize;

    // boards
    let mut spawn_reachable = AlignedU8s::blank();

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
pub fn to_bytes_u32x4(
    reachables_lower: &[ReachableSimd16; 4],
    reachables_upper: &[ReachableSimd16; 4],
) -> [[u32; 10]; 4] {
    [
        to_bytes_u32(&reachables_lower[0], &reachables_upper[0]),
        to_bytes_u32(&reachables_lower[1], &reachables_upper[1]),
        to_bytes_u32(&reachables_lower[2], &reachables_upper[2]),
        to_bytes_u32(&reachables_lower[3], &reachables_upper[3]),
    ]
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
