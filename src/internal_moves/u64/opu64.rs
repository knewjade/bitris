use std::cmp::Ordering::{Equal, Greater, Less};
use crate::coordinates::Offset;

#[inline(always)]
#[allow(clippy::nonminimal_bool)]
pub fn shift<const LEFT: i32, const RIGHT: i32, const DOWN: i32, const UP: i32>(
    data: [u64; 10],
) -> [u64; 10] {
    debug_assert!(0 <= LEFT && LEFT <= 4);
    debug_assert!(0 <= RIGHT && RIGHT <= 4);
    debug_assert!(
        (LEFT == 0 && RIGHT == 0) || (0 < LEFT && RIGHT == 0) || (0 < RIGHT && LEFT == 0)
    );

    debug_assert!(0 <= DOWN && DOWN <= 4);
    debug_assert!(0 <= UP && UP <= 4);
    debug_assert!((DOWN == 0 && UP == 0) || (0 < DOWN && UP == 0) || (0 < UP && DOWN == 0));

    if LEFT == 0 && RIGHT == 0 && DOWN == 0 && UP == 0 {
        return data;
    }

    // down or up
    let data = if 0 < DOWN {
        data.map(|v| v >> DOWN)
    } else if 0 < UP {
        data.map(|v| v << UP)
    } else {
        data
    };

    // left ot right
    if 0 < LEFT {
        let mut dest = [0u64; 10];
        let left = LEFT as usize;
        dest[..(10 - left)].copy_from_slice(&data[left..10]);
        dest
    } else if 0 < RIGHT {
        let mut dest = [0u64; 10];
        let right = RIGHT as usize;
        for index in 0..(10 - right) {
            dest[index + right] = data[index];
        }
        dest
    } else {
        data
    }
}

#[inline(always)]
pub fn shift_by_offset(data: [u64; 10], offset: Offset) -> [u64; 10] {
    if offset == Offset::new(0, 0) {
        return data;
    }

    // down or up
    let data = match offset.dy.cmp(&0) {
        Less => data.map(|v| v >> -offset.dy),
        Greater => data.map(|v| v << offset.dy),
        Equal => data,
    };

    // left ot right
    if offset.dx < 0 {
        let mut dest = [0u64; 10];
        let left = (-offset.dx) as usize;
        for index in 0..(10 - left) {
            dest[index] = data[index + left];
        }
        dest
    } else if 0 < offset.dx {
        let mut dest = [0u64; 10];
        let right = offset.dx as usize;
        for index in 0..(10 - right) {
            dest[index + right] = data[index];
        }
        dest
    } else {
        data
    }
}

// ボードを左右下方向にシフトしてマージ
#[inline(always)]
pub fn move_nr(data: [u64; 10], free_space: [u64; 10]) -> [u64; 10] {
    let mut data = data;

    {
        let index = 0;
        // left
        data[index] |= data[index + 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }
    for index in 1..9 {
        // left
        data[index] |= data[index + 1] & free_space[index];
        // right
        data[index] |= data[index - 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }
    {
        let index = 9;
        // right
        data[index] |= data[index - 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }

    data
}

#[inline(always)]
pub fn move_nl(data: [u64; 10], free_space: [u64; 10]) -> [u64; 10] {
    let mut data = data;

    {
        let index = 9;
        // right
        data[index] |= data[index - 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }
    for index in (1..9).rev() {
        // left
        data[index] |= data[index + 1] & free_space[index];
        // right
        data[index] |= data[index - 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }
    {
        let index = 0;
        // left
        data[index] |= data[index + 1] & free_space[index];
        // down
        loop {
            let d = (data[index] | (data[index] >> 1)) & free_space[index];
            if d == data[index] {
                break;
            }
            data[index] = d;
        }
    }

    data
}

#[inline(always)]
pub fn land(data: [u64; 10], free_space: [u64; 10]) -> [u64; 10] {
    [
        !(free_space[0] << 1) & data[0],
        !(free_space[1] << 1) & data[1],
        !(free_space[2] << 1) & data[2],
        !(free_space[3] << 1) & data[3],
        !(free_space[4] << 1) & data[4],
        !(free_space[5] << 1) & data[5],
        !(free_space[6] << 1) & data[6],
        !(free_space[7] << 1) & data[7],
        !(free_space[8] << 1) & data[8],
        !(free_space[9] << 1) & data[9],
    ]
}

#[inline(always)]
pub fn or(left: [u64; 10], right: [u64; 10]) -> [u64; 10] {
    [
        left[0] | right[0],
        left[1] | right[1],
        left[2] | right[2],
        left[3] | right[3],
        left[4] | right[4],
        left[5] | right[5],
        left[6] | right[6],
        left[7] | right[7],
        left[8] | right[8],
        left[9] | right[9],
    ]
}

#[inline(always)]
pub fn and(left: [u64; 10], right: [u64; 10]) -> [u64; 10] {
    [
        left[0] & right[0],
        left[1] & right[1],
        left[2] & right[2],
        left[3] & right[3],
        left[4] & right[4],
        left[5] & right[5],
        left[6] & right[6],
        left[7] & right[7],
        left[8] & right[8],
        left[9] & right[9],
    ]
}

#[inline(always)]
pub fn and_not(left: [u64; 10], right: [u64; 10]) -> [u64; 10] {
    [
        !left[0] & right[0],
        !left[1] & right[1],
        !left[2] & right[2],
        !left[3] & right[3],
        !left[4] & right[4],
        !left[5] & right[5],
        !left[6] & right[6],
        !left[7] & right[7],
        !left[8] & right[8],
        !left[9] & right[9],
    ]
}
