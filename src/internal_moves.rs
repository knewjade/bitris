/** This file is for internal */

pub(crate) mod moves64 {
    use std::ops::{Not, Shl, Shr};

    use crate::{bl, BlPlacement, BlPosition, Board, Offset, Orientation, Piece, PieceBlocksFactory, Rotate, Rotation, RotationSystem, Shape, With};

    // The type of boards used within this module.
    type Type = u64;
    type ThisBoard = Board<Type>;

    const TYPE_MAX: Type = !0;

    // Locations where the block does not exist are represented by 1.
    #[derive(Debug)]
    pub struct FreeBoard {
        cols: [Type; 10],
    }

    impl FreeBoard {
        #[inline]
        pub fn from(board: &ThisBoard) -> FreeBoard {
            Self { cols: board.cols.map(|col| !col) }
        }
    }


    // The position where there is space to place a piece is represented by 1.
    // The flags are aggregated to the position that corresponds to Bottom-Left.
    #[derive(Copy, Clone, Debug)]
    pub struct FreePieceBoard {
        cols: [Type; 10],
    }

    impl FreePieceBoard {
        // Returns a new board, all initialized with non-free.
        #[inline]
        const fn non_free() -> Self {
            Self { cols: [TYPE_MAX; 10] }
        }
    }

    // It holds `FreePieceBoard` for each orientation of a shape.
    #[derive(Copy, Clone, Debug)]
    pub struct FreePieceBoards {
        boards: [FreePieceBoard; 4],
    }

    impl FreePieceBoards {}

    impl FreePieceBoards {
        // Returns new boards, all initialized with non-free.
        #[inline]
        pub const fn non_free() -> Self {
            Self { boards: [FreePieceBoard::non_free(); 4] }
        }

        // Return new boards initialized to fit the piece.
        #[inline]
        pub fn new_to_fit(shape: Shape, free_board: &FreeBoard) -> Self {
            let mut dest = Self::non_free();
            for orientation in Orientation::all_into_iter() {
                let piece = PieceBlocksFactory.get(Piece { shape, orientation });
                for offset in piece.offsets {
                    Self::keep_if_offset_dest_is_free(
                        &mut dest.boards[orientation as usize], offset - piece.bottom_left, &free_board,
                    );
                }
            }
            dest
        }

        // When a block to which the offset destination is free(1), it keeps its bit.
        #[inline(always)]
        fn keep_if_offset_dest_is_free(
            free_piece_board: &mut FreePieceBoard,
            offset: Offset,
            free_board: &FreeBoard,
        ) {
            assert!(0 <= offset.dy);

            for x in 0..10 {
                let offset_x = x as i32 + offset.dx;
                if 0 <= offset_x && offset_x < 10 {
                    // All `free_piece_board.cols` bits are initialized as 1.
                    // Then, if all four block offsets are free, it is determined that there is space to place the piece.
                    free_piece_board.cols[x] &= free_board.cols[offset_x as usize] >> offset.dy;
                } else {
                    // If the offset destination is outside the board, it cannot be placed.
                    free_piece_board.cols[x] = 0;
                }
            }
        }

        #[inline]
        pub fn is_free(&self, orientation: Orientation, position: BlPosition) -> bool {
            0 < (self.boards[orientation as usize].cols[position.lx as usize] & 1 << position.by)
        }
    }


    // The position that the piece can reach is represented by 1.
    // The flags are aggregated to the position that corresponds to Bottom-Left.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct ReachablePieceBoard {
        cols: [Type; 10],
    }

    impl ReachablePieceBoard {
        // Remove positions where `other` is on.
        #[inline]
        pub fn remove(&mut self, other: &ReachablePieceBoard) {
            for x in 0..10 {
                self.cols[x] &= !other.cols[x];
            }
        }

        // Merge positions where `other` is on.
        #[inline]
        pub fn merge(&mut self, other: &ReachablePieceBoard) {
            for x in 0..10 {
                self.cols[x] |= other.cols[x];
            }
        }
    }

    impl ReachablePieceBoard {
        // Returns a new board, all initialized with non-reach.
        #[inline]
        pub const fn non_reach() -> Self {
            Self { cols: [0; 10] }
        }

        #[inline]
        pub fn mark_with_reached(&mut self, position: BlPosition) {
            self.cols[position.lx as usize] |= 1 << position.by
        }

        #[inline]
        pub fn is_blank(&self) -> bool {
            self.cols.iter().all(|it| *it == 0)
        }

        #[inline]
        pub fn count_ones(&self) -> u32 {
            self.cols.iter().map(|col| col.count_ones()).fold(0, |sum, it| sum + it)
        }
    }


    // It holds `ReachablePieceBoard` for each orientation of a shape.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct ReachablePieceBoards {
        boards: [ReachablePieceBoard; 4],
    }

    impl ReachablePieceBoards {
        // Returns new boards, all initialized with non-reach.
        #[inline]
        pub fn non_reach() -> Self {
            Self { boards: [ReachablePieceBoard::non_reach(); 4] }
        }

        // Mark the spawn position and the position rotated N times from the spawn position.
        #[inline]
        pub fn mark_spawn_and_its_post_rotations(
            &mut self, spawn: &BlPlacement, rotation_times: u32,
            free_piece_boards: &FreePieceBoards, rotation_system: &impl RotationSystem,
        ) {
            self.mark_with_reached(spawn.piece.orientation, spawn.position);

            for rotation in [Rotation::Cw, Rotation::Ccw] {
                self.mark_spawn_and_its_post_rotations_with_rotation(
                    spawn, rotation, rotation_times, &free_piece_boards, rotation_system,
                );
            }
        }

        // (private common process for `mark_spawn_and_its_post_rotations`)
        fn mark_spawn_and_its_post_rotations_with_rotation(
            &mut self, spawn: &BlPlacement, rotation: Rotation, rotation_times: u32,
            free_piece_boards: &FreePieceBoards, rotation_system: &impl RotationSystem,
        ) {
            let mut from = *spawn;
            for _ in 0..rotation_times {
                let to = from.rotate(rotation);

                let mut next_from: Option<BlPlacement> = None;
                for kick in rotation_system.iter_kicks(from.piece, rotation) {
                    let moved = to + kick.offset;
                    if free_piece_boards.is_free(to.piece.orientation, moved.position) {
                        next_from = Some(moved);
                        break;
                    }
                }

                if let Some(placement) = next_from {
                    self.mark_with_reached(placement.piece.orientation, placement.position);
                    from = placement;
                } else {
                    return;
                }
            }
        }

        // Mark a position that can be reached.
        #[inline(always)]
        fn mark_with_reached(&mut self, orientation: Orientation, position: BlPosition) {
            self.boards[orientation as usize].mark_with_reached(position);
        }

        // From left to right, update the reachable position.
        // The number of updates is always fixed at 9.
        #[inline]
        fn update_by_moving_right(&mut self, free: &FreePieceBoards) {
            for orientation_index in 0..4 {
                for x in 0..9 {
                    self.boards[orientation_index].cols[x + 1] |= self.boards[orientation_index].cols[x] & free.boards[orientation_index].cols[x + 1];
                }
            }
        }

        // From right to left, update the reachable position.
        // The number of updates is always fixed at 9.
        #[inline]
        fn update_by_moving_left(&mut self, free: &FreePieceBoards) {
            for orientation_index in 0..4 {
                for x in (1..10).rev() {
                    self.boards[orientation_index].cols[x - 1] |= self.boards[orientation_index].cols[x] & free.boards[orientation_index].cols[x - 1];
                }
            }
        }

        // From up to down, update the reachable position.
        // It will be updated for each column and repeats the moving down one row until it stops changing.
        #[inline]
        fn update_by_moving_down(&mut self, free_piece_boards: &FreePieceBoards) {
            for orientation_index in 0..4 {
                for x in 0..10 {
                    let free = free_piece_boards.boards[orientation_index].cols[x];

                    // If the position one row down is free, mark it as reachable.
                    let mut updating = self.boards[orientation_index].cols[x];
                    updating |= (updating >> 1) & free;

                    // Repeat that until it stops changing.
                    while self.boards[orientation_index].cols[x] != updating {
                        self.boards[orientation_index].cols[x] = updating;
                        updating |= (updating >> 1) & free;
                    }
                }
            }
        }

        // By rotating, update the reachable position.
        fn update_by_rotating(
            &mut self, shape: Shape, rotation: Rotation,
            free_piece_boards: &FreePieceBoards,
            previous_pre_rotation: &ReachablePieceBoards,
            rotation_system: &impl RotationSystem,
        ) {
            for orientation in Orientation::all_into_iter() {
                let from = PieceBlocksFactory.get(Piece { shape, orientation });
                let to = from.rotate(rotation);
                let bl_offset = to.bottom_left - from.bottom_left;

                // Exclude previously checked positions from reachable positions.
                let mut candidates_board = self.boards[from.piece.orientation as usize].clone();
                candidates_board.remove(&previous_pre_rotation.boards[from.piece.orientation as usize]);

                // For each kick, check to see if it can rotate from the candidate position.
                // If it can rotate, remove it from the candidates.
                for kick in rotation_system.iter_kicks(from.piece(), rotation) {
                    // Ends when the candidate positions are nothing.
                    if candidates_board.is_blank() {
                        break;
                    }

                    let offset = kick.offset + bl_offset;

                    let to = to.piece.orientation as usize;
                    let dx = offset.dx;
                    let dy = offset.dy.abs() as usize;

                    let (start, end) = if 0 <= dx {
                        (0, 10 - dx as usize)
                    } else {
                        (-dx as usize, 10)
                    };

                    let free_piece_board = &free_piece_boards.boards[to];
                    if 0 <= offset.dy {
                        let forward_op = u64::shl;
                        let backward_op = u64::shr;
                        self.update_by_rotating_for_a_kick(
                            &mut candidates_board, free_piece_board,
                            to, (dx, dy), (start, end), (forward_op, backward_op),
                        );
                    } else {
                        let forward_op = u64::shr;
                        let backward_op = u64::shl;
                        self.update_by_rotating_for_a_kick(
                            &mut candidates_board, free_piece_board,
                            to, (dx, dy), (start, end), (forward_op, backward_op),
                        );
                    }
                }
            }
        }

        // (private common process for `update_by_rotating`. Inline is strongly recommended.)
        #[inline(always)]
        fn update_by_rotating_for_a_kick(
            &mut self,
            candidates_board: &mut ReachablePieceBoard,
            free_piece_board: &FreePieceBoard,
            to: usize,
            (dx, dy): (i32, usize),
            (start, end): (usize, usize),
            (forward_op, backward_op): (fn(u64, usize) -> u64, fn(u64, usize) -> u64),
        ) {
            for x in start..end {
                let tx = (x as i32 + dx) as usize;
                let from = candidates_board.cols[x];
                let free = free_piece_board.cols[tx];
                let fixed = forward_op(from, dy) & free;
                self.boards[to].cols[tx] |= fixed;
                candidates_board.cols[x] &= !backward_op(fixed, dy);
            }
        }

        // Extract lockable positions from the currently free positions.
        #[inline]
        pub fn extract_landed_positions(&mut self, free_piece_boards: &FreePieceBoards) {
            for orientation_index in 0..4 {
                for x in 0..10 {
                    self.boards[orientation_index].cols[x] &= !(free_piece_boards.boards[orientation_index].cols[x] << 1);
                }
            }
        }

        // Extract canonical positions from the currently free positions.
        #[inline]
        pub fn minimize(&mut self, shape: Shape) {
            let mut visited = ReachablePieceBoards::non_reach();
            for orientation in Orientation::all_iter() {
                let canonical = Piece::new(shape, *orientation).canonical_or_self();
                self.boards[*orientation as usize].remove(&visited.boards[canonical.orientation as usize]);
                visited.boards[canonical.orientation as usize].merge(&self.boards[*orientation as usize]);
            }
        }

        #[inline]
        pub fn count_ones(&self) -> u32 {
            self.boards.iter()
                .map(|board| board.count_ones())
                .fold(0, |sum, count| sum + count)
        }
    }

    // Generate boards with reachable locations.
    pub fn gen_reachable_softdrop(
        spawn: &BlPlacement,
        free_piece_boards: &FreePieceBoards,
        rotation_system: &impl RotationSystem,
    ) -> ReachablePieceBoards {
        // ==========================================================================================================
        // [NOTE] The characteristics of each operation are as follows. The order of operations is determined accordingly.
        //
        // # moving right/left
        // The number of internal processes is always fixed at 4*9 times, so it is lightweight and stable
        //
        // # moving down
        // Internal processing repeats until there are no more changes,
        // so use it when current positions are " very limited or almost completed."
        //
        // # rotating cw/ccw
        // For SRS testing, more processing is required than others, so it is used as little as possible.
        // ==========================================================================================================

        assert!(free_piece_boards.is_free(spawn.piece.orientation, spawn.position));

        let mut reachable_piece_boards = ReachablePieceBoards::non_reach();
        reachable_piece_boards.mark_spawn_and_its_post_rotations(spawn, 2, &free_piece_boards, rotation_system);

        // Preparation: At least cover the positions reachable by harddrop.
        // In the beginning, changes will almost certainly occur.
        // Thus, predefined operations are applied before the loop.
        {
            // First Down: most columns have no flags, so propagation is skipped.
            reachable_piece_boards.update_by_moving_down(&free_piece_boards);

            // Left and Right: Wide space coverage with faster operations.
            reachable_piece_boards.update_by_moving_right(&free_piece_boards);
            reachable_piece_boards.update_by_moving_left(&free_piece_boards);

            // Second Down: Required for harddrop.
            // If the block is sparse, it is likely to be covered by the side moving operations.
            // In that case, the changes will be few.
            // Therefore, Propagation will be reduced.
            reachable_piece_boards.update_by_moving_down(&free_piece_boards);
        }

        // Expand the reachable area without using rotation.
        // Rotating process is heavy and should be done as few times as possible.
        loop {
            let freeze = reachable_piece_boards.clone();

            reachable_piece_boards.update_by_moving_right(&free_piece_boards);
            reachable_piece_boards.update_by_moving_left(&free_piece_boards);
            reachable_piece_boards.update_by_moving_down(&free_piece_boards);

            if freeze == reachable_piece_boards {
                break;
            }
        }

        // If the rotation does not change the position, then it's complete.
        if rotation_system.is_moving_in_rotation(spawn.piece.shape).not() {
            return reachable_piece_boards;
        }

        // Expand the reachable area using rotation.
        // If no change occurs after applying all operations, then it's complete.
        let mut freeze = ReachablePieceBoards::non_reach();

        loop {
            // These boards have positions in the previous pre-rotation.
            // These boards is used to cut positions that have already been checked rotation.
            let previous_pre_rotation = freeze.clone();

            // Save boards before operations.
            freeze = reachable_piece_boards.clone();

            // First, it starts with rotation operations, as it does not change except for rotation already.
            // Positions that have already been checked are skipped in the calculation.
            //
            // Precisely, the previous positions of each rotation should be recorded.
            // However, copying also takes time, so it approximates by the board at the start of the last loop (before the last rotation).
            reachable_piece_boards.update_by_rotating(
                spawn.piece.shape, Rotation::Cw, &free_piece_boards, &previous_pre_rotation, rotation_system,
            );
            reachable_piece_boards.update_by_rotating(
                spawn.piece.shape, Rotation::Ccw, &free_piece_boards, &previous_pre_rotation, rotation_system,
            );

            // Apply from down because it's faster when there are fewer changes.
            reachable_piece_boards.update_by_moving_down(&free_piece_boards);

            // The side moving operations is needed to assure that no changes occur.
            reachable_piece_boards.update_by_moving_right(&free_piece_boards);
            reachable_piece_boards.update_by_moving_left(&free_piece_boards);

            if freeze == reachable_piece_boards {
                break;
            }
        }

        reachable_piece_boards
    }

    // Generate boards with reachable locations without down move.
    // Targets move that can be moved by rotating at the spawn position, then moving left/right, then hard drop.
    pub fn gen_reachable_harddrop(
        spawn: &BlPlacement,
        free_piece_boards: &FreePieceBoards,
        rotation_system: &impl RotationSystem,
    ) -> ReachablePieceBoards {
        assert!(free_piece_boards.is_free(spawn.piece.orientation, spawn.position));

        let mut reachable_piece_boards = ReachablePieceBoards::non_reach();
        reachable_piece_boards.mark_spawn_and_its_post_rotations(spawn, 2, &free_piece_boards, rotation_system);

        // Left and Right
        reachable_piece_boards.update_by_moving_right(&free_piece_boards);
        reachable_piece_boards.update_by_moving_left(&free_piece_boards);

        // Harddrop
        reachable_piece_boards.update_by_moving_down(&free_piece_boards);

        reachable_piece_boards
    }

    #[derive(Debug)]
    pub struct Moves {
        pub spawn: BlPlacement,
        pub reachable_piece_boards: ReachablePieceBoards,
    }

    impl Moves {
        #[inline]
        pub fn vec(&self) -> Vec<BlPlacement> {
            let capacity = self.reachable_piece_boards.count_ones() as usize;
            self.vec_with_capacity(capacity)
        }

        /// `capacity` is a hint and does not affect the result.
        pub fn vec_with_capacity(&self, capacity: usize) -> Vec<BlPlacement> {
            let mut out = Vec::<BlPlacement>::with_capacity(capacity);

            for orientation in Orientation::all_into_iter() {
                let piece = Piece::new(self.spawn.piece.shape, orientation);
                for lx in 0..10 {
                    let mut col = self.reachable_piece_boards.boards[orientation as usize].cols[lx];
                    while 0 < col {
                        let next = col & (col - 1);
                        let bit = col - next;

                        let by = bit.trailing_zeros();
                        out.push(piece.with(bl(lx as i32, by as i32)));
                        col = next;
                    }
                }
            }

            out
        }
    }

    pub(crate) fn all_moves_softdrop(rotation_system: &impl RotationSystem, board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable_softdrop(&spawn, &free_piece_boards, rotation_system);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);

        Moves { spawn, reachable_piece_boards }
    }

    pub(crate) fn minimized_moves_softdrop(rotation_system: &impl RotationSystem, board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable_softdrop(&spawn, &free_piece_boards, rotation_system);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);
        reachable_piece_boards.minimize(spawn.piece.shape);

        Moves { spawn, reachable_piece_boards }
    }

    pub(crate) fn all_moves_harddrop<'a>(rotation_system: &impl RotationSystem, board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable_harddrop(&spawn, &free_piece_boards, rotation_system);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);

        Moves { spawn, reachable_piece_boards }
    }

    pub(crate) fn minimized_moves_harddrop<'a>(rotation_system: &impl RotationSystem, board: &Board<u64>, spawn: BlPlacement) -> Moves {
        let free_board = FreeBoard::from(&board);
        let free_piece_boards = FreePieceBoards::new_to_fit(spawn.piece.shape, &free_board);

        let mut reachable_piece_boards = gen_reachable_harddrop(&spawn, &free_piece_boards, rotation_system);
        reachable_piece_boards.extract_landed_positions(&free_piece_boards);
        reachable_piece_boards.minimize(spawn.piece.shape);

        Moves { spawn, reachable_piece_boards }
    }
}
