use crate::boards::Board;
use crate::internal_moves::avx2::h24::aligned::AlignedU24s;
use crate::internal_moves::avx2::h24::{free, opsimd};
use crate::internal_moves::avx2::h24::free_space::FreeSpaceSimd24;
use crate::internal_moves::avx2::h24::reachable::ReachableSimd24;
use crate::pieces::{Orientation, Piece, Shape};
use crate::placements::CcPlacement;

#[inline(always)]
pub fn to_free_spaces(free_space_block: &FreeSpaceSimd24, shape: Shape) -> [FreeSpaceSimd24; 4] {
    free::to_free_spaces(free_space_block, shape)
}

#[inline(always)]
pub fn to_free_space(free_space_block_lower: &FreeSpaceSimd24, piece: Piece) -> FreeSpaceSimd24 {
    free::to_free_space(&free_space_block_lower, piece)
}

#[inline(always)]
pub fn to_free_space_block(board: &Board<u64>) -> FreeSpaceSimd24 {
    let bytes_u64 = board.cols.map(|col| !col);

    let bytes_u8: [u8; 32] = [
        0,
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
    ];

    FreeSpaceSimd24::from(AlignedU24s::new(bytes_u8))
}

#[inline(always)]
pub fn spawn_and_harddrop_reachables(
    spawn: CcPlacement,
    free_space_block: &FreeSpaceSimd24,
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

    let orientation_index = spawn.piece.orientation as usize;
    let data = opsimd::spawn(
        spawn,
        free_space_block.data,
        free_spaces[orientation_index].data,
    );

    match spawn.piece.orientation {
        Orientation::North => [
            ReachableSimd24::new(data),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
        ],
        Orientation::East => [
            ReachableSimd24::blank(),
            ReachableSimd24::new(data),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
        ],
        Orientation::South => [
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::new(data),
            ReachableSimd24::blank(),
        ],
        Orientation::West => [
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::blank(),
            ReachableSimd24::new(data),
        ],
    }
}

#[inline(always)]
pub fn spawn_and_harddrop_reachable(
    spawn: CcPlacement,
    free_space_block: &FreeSpaceSimd24,
    free_space: &FreeSpaceSimd24,
) -> ReachableSimd24 {
    if spawn.position.cy < 0 || 24 <= spawn.position.cy {
        return ReachableSimd24::blank();
    }

    let data = opsimd::spawn(
        spawn,
        free_space_block.data,
        free_space.data,
    );

    ReachableSimd24::new(data)
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
