use std::cmp;
use std::fmt;
use std::str::FromStr;

use crate::boards::Lines;
use crate::coordinates::{Location, xy};

/// Ceiling of the board.
pub trait Ceiling: Sized {
    /// Returns the board height.
    fn ceiling() -> u32;
}

/// Low level board operations.
pub trait BoardOp: Ceiling {
    /// Returns the board height.
    #[inline(always)]
    fn ceiling(&self) -> u32 { <Self as Ceiling>::ceiling() }

    /// Returns the height to the highest block. If the board is empty, returns 0.
    fn well_top(&self) -> u32;

    /// Set a block at the location.
    fn set_at(&mut self, location: Location);

    /// Unset a block at the location.
    fn unset_at(&mut self, location: Location);

    /// Returns true if a block exists at the location.
    fn is_occupied_at(&self, location: Location) -> bool;

    /// Returns true if a block does not exist at the location.
    fn is_free_at(&self, location: Location) -> bool {
        !self.is_occupied_at(location)
    }

    /// Returns true if there are no blocks on the board.
    fn is_empty(&self) -> bool;

    /// Returns total blocks in the board.
    fn count_blocks(&self) -> u32;

    /// Returns as a key the row in which one or more blocks exist.
    fn used_rows(&self) -> Lines;

    /// Returns as key the rows that are all filled with blocks.
    fn filled_rows(&self) -> Lines;

    /// Remove specified rows only.
    fn clear_lines_partially(&mut self, lines: Lines);

    /// Remove rows that are all filled with blocks.
    fn clear_lines(&mut self) -> Lines;

    /// Swap all blocks and spaces.
    fn invert(&mut self);

    /// Reverse left and right.
    fn mirror(&mut self);

    /// Returns true if there is an overlap.
    fn overlaps(&self, other: &Self) -> bool;

    /// Merge self and other.
    fn merge(&mut self, other: &Self);

    /// Remove all blocks, from self to other
    fn remove_all(&mut self, other: &Self);

    /// Returns true if the location is accessible within the board.
    fn test_access(&self, location: Location) -> bool {
        0 <= location.x && location.x < 10 && 0 <= location.y && location.y < self.ceiling() as i32
    }

    /// Set all blocks at the location on the board. No apply line clear.
    /// If the block already exists, it's nothing happens.
    fn set_all(&mut self, locations: &[Location]) {
        for &location in locations {
            self.set_at(location);
        }
    }

    /// Unset all blocks at the location on the board.
    /// If no block exists, it's nothing happens.
    fn unset_all(&mut self, locations: &[Location]) {
        for &location in locations {
            self.unset_at(location);
        }
    }
}

/// Shrinks and converts to this type from the input type.
pub trait ShrinkFrom<T>: Sized {
    fn shrink_from(value: T) -> Self;
}


/// It represents the position of the blocks.
///
/// Therefore, it must be ensured that the caller does not access outside the board. (Example: MUST NOT call `set_at(xy(-1, -1)`)
/// Instead, validations within this struct are minimal, allowing for faster data handling.
///
/// It has the position of the blocks as a bit array.
/// Bit arrays record positions vertically.
/// Therefore, the size of the array is 10 (=width).
/// Generic type can be unsigned int(u8-u64).
/// This bit width corresponds to the height of the board.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Board<T> {
    pub cols: [T; 10],
}

impl Board<u8> {
    #[inline]
    pub const fn new(cols: [u8; 10]) -> Self {
        Self { cols }
    }

    #[inline]
    pub const fn blank() -> Self {
        Self { cols: [0; 10] }
    }

    /// Returns a board filled up to a specified height.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Board8::filled_up_to(0).count_blocks(), 0);
    /// assert_eq!(Board8::filled_up_to(1).count_blocks(), 10);
    /// assert_eq!(Board8::filled_up_to(5).count_blocks(), 50);
    /// ```
    #[inline]
    pub const fn filled_up_to(height: u8) -> Self {
        Self { cols: [(1 << height) - 1; 10] }
    }

    /// Returns a new board after clearing lines.
    #[inline]
    pub fn after_clearing(&self) -> Self {
        let mut board = self.clone();
        board.clear_lines();
        board
    }
}

impl Board<u16> {
    #[inline]
    pub const fn new(cols: [u16; 10]) -> Self {
        Self { cols }
    }

    #[inline]
    pub const fn blank() -> Self {
        Self { cols: [0; 10] }
    }

    /// Returns a board filled up to a specified height.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Board16::filled_up_to(0).count_blocks(), 0);
    /// assert_eq!(Board16::filled_up_to(1).count_blocks(), 10);
    /// assert_eq!(Board16::filled_up_to(5).count_blocks(), 50);
    /// ```
    #[inline]
    pub const fn filled_up_to(height: u8) -> Self {
        Self { cols: [(1 << height) - 1; 10] }
    }

    /// Returns a new board after clearing lines.
    #[inline]
    pub fn after_clearing(&self) -> Self {
        let mut board = self.clone();
        board.clear_lines();
        board
    }
}

impl Board<u32> {
    #[inline]
    pub const fn new(cols: [u32; 10]) -> Self {
        Self { cols }
    }

    #[inline]
    pub const fn blank() -> Self {
        Self { cols: [0; 10] }
    }

    /// Returns a board filled up to a specified height.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Board32::filled_up_to(0).count_blocks(), 0);
    /// assert_eq!(Board32::filled_up_to(1).count_blocks(), 10);
    /// assert_eq!(Board32::filled_up_to(5).count_blocks(), 50);
    /// ```
    #[inline]
    pub const fn filled_up_to(height: u8) -> Self {
        Self { cols: [(1 << height) - 1; 10] }
    }

    /// Returns a new board after clearing lines.
    #[inline]
    pub fn after_clearing(&self) -> Self {
        let mut board = self.clone();
        board.clear_lines();
        board
    }
}

impl Board<u64> {
    #[inline]
    pub const fn new(cols: [u64; 10]) -> Self {
        Self { cols }
    }

    #[inline]
    pub const fn blank() -> Self {
        Self { cols: [0; 10] }
    }

    /// Returns a board filled up to a specified height.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Board64::filled_up_to(0).count_blocks(), 0);
    /// assert_eq!(Board64::filled_up_to(1).count_blocks(), 10);
    /// assert_eq!(Board64::filled_up_to(5).count_blocks(), 50);
    /// ```
    #[inline]
    pub const fn filled_up_to(height: u8) -> Self {
        Self { cols: [(1 << height) - 1; 10] }
    }

    /// Returns a new board after clearing lines.
    #[inline]
    #[must_use]
    pub fn after_clearing(&self) -> Self {
        let mut board = self.clone();
        board.clear_lines();
        board
    }
}


impl<T> fmt::Display for Board<T> where Board<T>: BoardOp {
    /// ```
    /// use bitris::prelude::*;
    /// let mut board = Board64::default();
    /// board.set_at(xy(2, 1));
    ///
    /// let expected = "\
    /// (Board64):\n\
    /// ..........\n\
    /// ..#.......\n\
    /// ..........";
    /// assert_eq!(format!("{}", board), expected);
    /// `````
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let height = cmp::min(self.well_top() + 1, self.ceiling()) as u32;
        let mut str = String::with_capacity((height * 11) as usize);
        for y in (0..height).rev() {
            let y = y as i32;
            for x in 0..10 {
                let ch = if self.is_occupied_at(xy(x, y)) { '#' } else { '.' };
                str.push(ch)
            }
            if 0 < y {
                str.push('\n')
            }
        }
        write!(f, "(Board{}):\n{}", self.ceiling(), str.to_string())
    }
}


/// A collection of errors that occur during board creation.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BoardFromStrError {
    /// args: (invalid_char)
    InvalidCharacter(char),
    /// args: (width)
    MismatchedWidth(u32),
    /// args: (ceiling)
    ExceedBoardCeiling(u32),
}

impl fmt::Display for BoardFromStrError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BoardFromStrError::*;
        match *self {
            InvalidCharacter(ch) => write!(fmt, "`{}` is invalid", ch),
            MismatchedWidth(width) => write!(fmt, "total blocks is mismatched width: {}", width),
            ExceedBoardCeiling(ceiling) => write!(fmt, "exceed board ceiling: ceiling={}", ceiling),
        }
    }
}

impl<T> FromStr for Board<T> where Board<T>: BoardOp + Default {
    type Err = BoardFromStrError;

    /// ```
    /// use std::str::FromStr;
    /// use bitris::prelude::*;
    /// let board = Board64::from_str("
    ///     ..........
    ///     ..........
    ///     ..#.......
    ///     ..........
    /// ").unwrap();
    /// assert!(board.is_occupied_at(xy(2, 1)));
    /// ```
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        use BoardFromStrError::*;

        let mut board = Board::<T>::default();
        let ceiling = board.ceiling();
        let mut index = 0;
        for char in str.chars().rev() {
            match char {
                '#' | 'X' => {
                    board.set_at(xy(9 - index % 10, index / 10));
                    index += 1;
                }
                '.' | '_' => {
                    index += 1
                }
                ' ' | '\n' | '\r' => {
                    continue;
                }
                _ => {
                    return Err(InvalidCharacter(char));
                }
            }

            if 10 * ceiling <= index as u32 {
                return Err(ExceedBoardCeiling(ceiling));
            }
        }

        if index % 10 != 0 {
            return Err(MismatchedWidth(index as u32));
        }

        Ok(board)
    }
}


macro_rules! board_from {
    ($t:ty, into $u:ty) => {
        impl From<Board<$t>> for Board<$u> {
            #[inline]
            fn from(board: Board<$t>) -> Self {
                Self { cols: board.cols.map(Into::into) }
            }
        }

        impl From<&Board<$t>> for Board<$u> {
            #[inline]
            fn from(board: &Board<$t>) -> Self {
                Self { cols: board.cols.map(Into::into) }
            }
        }
    };
}

macro_rules! board_shrink_from {
    ($t:ty, into $u:ty) => {
        impl ShrinkFrom<Board<$t>> for Board<$u> {
            #[inline]
            fn shrink_from(board: Board<$t>) -> Self {
                Self { cols: board.cols.map(|it| it as $u) }
            }
        }

        impl ShrinkFrom<&Board<$t>> for Board<$u> {
            #[inline]
            fn shrink_from(board: &Board<$t>) -> Self {
                Self { cols: board.cols.map(|it| it as $u) }
            }
        }
    };
}

board_from!( u8, into u16);
board_from!( u8, into u32);
board_from!( u8, into u64);
board_from!(u16, into u32);
board_from!(u16, into u64);
board_from!(u32, into u64);

board_shrink_from!(u64, into u32);
board_shrink_from!(u64, into u16);
board_shrink_from!(u64, into u8 );
board_shrink_from!(u32, into u16);
board_shrink_from!(u32, into u8 );
board_shrink_from!(u16, into u8 );


macro_rules! set_at {
    ($cols:expr, $loc:expr) => {
        $cols[$loc.x as usize] |= (1 << $loc.y)
    };
}

macro_rules! unset_at {
    ($cols:expr, $location:expr) => {
        $cols[$location.x as usize] &= !(1 << $location.y)
    };
}

macro_rules! is_occupied_at {
    ($cols:expr, $location:expr) => {
        0 < ($cols[$location.x as usize] & (1 << $location.y))
    };
}

macro_rules! is_free_at {
    ($cols:expr, $location:expr) => {
        ($cols[$location.x as usize] & (1 << $location.y)) == 0
    };
}

macro_rules! is_empty {
    ($cols:expr) => {
        $cols.iter().all(|col| *col == 0)
    };
}

macro_rules! count_blocks {
    ($cols:expr) => {
        $cols.iter().map(|col| col.count_ones()).fold(0, |sum, it| sum + it)
    };
}

macro_rules! used_row_key {
    ($cols:expr) => {
        $cols.iter().fold(0, |merged, col| merged | col)
    };
}

macro_rules! filled_row_key {
    ($cols:expr) => {
        $cols.iter().fold(!0, |merged, col| merged & col)
    };
}

macro_rules! clear_lines {
    ($cols:expr, $key:expr) => ({
        let mut key = $key;
        while 0 < key {
            let mask = (key - 1) & !key;
            let inverted_mask = !mask;
            for col in &mut $cols {
                let bottom = *col & mask;
                let slided_upper = (*col >> 1) & inverted_mask;
                *col = slided_upper | bottom;
            }
            key = (key >> 1) & inverted_mask;
        }
    });
}

macro_rules! invert {
    ($cols:expr) => ({
        for col in &mut $cols {
            *col = !(*col);
        }
    });
}

macro_rules! mirror {
    ($cols:expr) => {
        $cols.reverse()
    };
}

macro_rules! overlaps {
    ($cols1:expr, $cols2:expr) => ({
        for x in 0..10 {
            if 0 < ($cols1[x] & $cols2[x]) {
                return true;
            }
        }
        false
    });
}

macro_rules! merge {
    ($cols1:expr, $cols2:expr) => {
        for x in 0..10 {
            $cols1[x] |= $cols2[x];
        }
    };
}

macro_rules! remove_all {
    ($cols1:expr, $cols2:expr) => {
        for x in 0..10 {
            $cols1[x] &= !($cols2[x]);
        }
    };
}

impl Ceiling for Board<u8> {
    #[inline(always)]
    fn ceiling() -> u32 {
        8
    }
}

impl Ceiling for Board<u16> {
    #[inline(always)]
    fn ceiling() -> u32 {
        16
    }
}

impl Ceiling for Board<u32> {
    #[inline(always)]
    fn ceiling() -> u32 {
        32
    }
}

impl Ceiling for Board<u64> {
    #[inline(always)]
    fn ceiling() -> u32 {
        64
    }
}

impl BoardOp for Board<u8> {
    #[inline]
    fn well_top(&self) -> u32 {
        self.ceiling() - used_row_key!(self.cols).leading_zeros()
    }

    #[inline]
    fn set_at(&mut self, location: Location) {
        set_at!(self.cols, location)
    }

    #[inline]
    fn unset_at(&mut self, location: Location) {
        unset_at!(self.cols, location)
    }

    #[inline]
    fn is_occupied_at(&self, location: Location) -> bool {
        is_occupied_at!(self.cols, location)
    }

    #[inline]
    fn is_free_at(&self, location: Location) -> bool {
        is_free_at!(self.cols, location)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        is_empty!(self.cols)
    }

    #[inline]
    fn count_blocks(&self) -> u32 {
        count_blocks!(self.cols)
    }

    #[inline]
    fn used_rows(&self) -> Lines {
        Lines::new(used_row_key!(self.cols) as u64)
    }

    #[inline]
    fn filled_rows(&self) -> Lines {
        Lines::new(filled_row_key!(self.cols) as u64)
    }

    #[inline]
    fn clear_lines_partially(&mut self, lines: Lines) {
        clear_lines!(self.cols, lines.key as u8)
    }

    #[inline]
    fn clear_lines(&mut self) -> Lines {
        let key = filled_row_key!(self.cols);
        clear_lines!(self.cols, key);
        Lines::new(key as u64)
    }

    #[inline]
    fn invert(&mut self) {
        invert!(self.cols)
    }

    #[inline]
    fn mirror(&mut self) {
        mirror!(self.cols)
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self.cols, other.cols)
    }

    #[inline]
    fn merge(&mut self, other: &Self) {
        merge!(self.cols, other.cols)
    }

    #[inline]
    fn remove_all(&mut self, other: &Self) {
        remove_all!(self.cols, other.cols)
    }
}

impl BoardOp for Board<u16> {
    #[inline]
    fn well_top(&self) -> u32 {
        self.ceiling() - used_row_key!(self.cols).leading_zeros()
    }

    #[inline]
    fn set_at(&mut self, location: Location) {
        set_at!(self.cols, location)
    }

    #[inline]
    fn unset_at(&mut self, location: Location) {
        unset_at!(self.cols, location)
    }

    #[inline]
    fn is_occupied_at(&self, location: Location) -> bool {
        is_occupied_at!(self.cols, location)
    }

    #[inline]
    fn is_free_at(&self, location: Location) -> bool {
        is_free_at!(self.cols, location)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        is_empty!(self.cols)
    }

    #[inline]
    fn count_blocks(&self) -> u32 {
        count_blocks!(self.cols)
    }

    #[inline]
    fn used_rows(&self) -> Lines {
        Lines::new(used_row_key!(self.cols) as u64)
    }

    #[inline]
    fn filled_rows(&self) -> Lines {
        Lines::new(filled_row_key!(self.cols) as u64)
    }

    #[inline]
    fn clear_lines_partially(&mut self, lines: Lines) {
        clear_lines!(self.cols, lines.key as u16)
    }

    #[inline]
    fn clear_lines(&mut self) -> Lines {
        let key = filled_row_key!(self.cols);
        clear_lines!(self.cols, key);
        Lines::new(key as u64)
    }

    #[inline]
    fn invert(&mut self) {
        invert!(self.cols)
    }

    #[inline]
    fn mirror(&mut self) {
        mirror!(self.cols)
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self.cols, other.cols)
    }

    #[inline]
    fn merge(&mut self, other: &Self) {
        merge!(self.cols, other.cols)
    }

    #[inline]
    fn remove_all(&mut self, other: &Self) {
        remove_all!(self.cols, other.cols)
    }
}

impl BoardOp for Board<u32> {
    #[inline]
    fn well_top(&self) -> u32 {
        self.ceiling() - used_row_key!(self.cols).leading_zeros()
    }

    #[inline]
    fn set_at(&mut self, location: Location) {
        set_at!(self.cols, location)
    }

    #[inline]
    fn unset_at(&mut self, location: Location) {
        unset_at!(self.cols, location)
    }

    #[inline]
    fn is_occupied_at(&self, location: Location) -> bool {
        is_occupied_at!(self.cols, location)
    }

    #[inline]
    fn is_free_at(&self, location: Location) -> bool {
        is_free_at!(self.cols, location)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        is_empty!(self.cols)
    }

    #[inline]
    fn count_blocks(&self) -> u32 {
        count_blocks!(self.cols)
    }

    #[inline]
    fn used_rows(&self) -> Lines {
        Lines::new(used_row_key!(self.cols) as u64)
    }

    #[inline]
    fn filled_rows(&self) -> Lines {
        Lines::new(filled_row_key!(self.cols) as u64)
    }

    #[inline]
    fn clear_lines_partially(&mut self, lines: Lines) {
        clear_lines!(self.cols, lines.key as u32)
    }

    #[inline]
    fn clear_lines(&mut self) -> Lines {
        let key = filled_row_key!(self.cols);
        clear_lines!(self.cols, key);
        Lines::new(key as u64)
    }

    #[inline]
    fn invert(&mut self) {
        invert!(self.cols)
    }

    #[inline]
    fn mirror(&mut self) {
        mirror!(self.cols)
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self.cols, other.cols)
    }

    #[inline]
    fn merge(&mut self, other: &Self) {
        merge!(self.cols, other.cols)
    }

    #[inline]
    fn remove_all(&mut self, other: &Self) {
        remove_all!(self.cols, other.cols)
    }
}

impl BoardOp for Board<u64> {
    fn well_top(&self) -> u32 {
        self.ceiling() - used_row_key!(self.cols).leading_zeros()
    }

    #[inline]
    fn set_at(&mut self, location: Location) {
        set_at!(self.cols, location)
    }

    #[inline]
    fn unset_at(&mut self, location: Location) {
        unset_at!(self.cols, location)
    }

    #[inline]
    fn is_occupied_at(&self, location: Location) -> bool {
        is_occupied_at!(self.cols, location)
    }

    #[inline]
    fn is_free_at(&self, location: Location) -> bool {
        is_free_at!(self.cols, location)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        is_empty!(self.cols)
    }

    #[inline]
    fn count_blocks(&self) -> u32 {
        count_blocks!(self.cols)
    }

    #[inline]
    fn used_rows(&self) -> Lines {
        Lines::new(used_row_key!(self.cols))
    }

    #[inline]
    fn filled_rows(&self) -> Lines {
        Lines::new(filled_row_key!(self.cols))
    }

    #[inline]
    fn clear_lines_partially(&mut self, lines: Lines) {
        clear_lines!(self.cols, lines.key)
    }

    #[inline]
    fn clear_lines(&mut self) -> Lines {
        let key = filled_row_key!(self.cols);
        clear_lines!(self.cols, key);
        Lines::new(key)
    }

    #[inline]
    fn invert(&mut self) {
        invert!(self.cols)
    }

    #[inline]
    fn mirror(&mut self) {
        mirror!(self.cols)
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self.cols, other.cols)
    }

    #[inline]
    fn merge(&mut self, other: &Self) {
        merge!(self.cols, other.cols)
    }

    #[inline]
    fn remove_all(&mut self, other: &Self) {
        remove_all!(self.cols, other.cols)
    }
}

/// An alias for `BoardT<u8>`
pub type Board8 = Board<u8>;

/// An alias for `BoardT<u16>`
pub type Board16 = Board<u16>;

/// An alias for `BoardT<u32>`
pub type Board32 = Board<u32>;

/// An alias for `BoardT<u64>`
pub type Board64 = Board<u64>;


#[cfg(test)]
mod tests {
    use std::fmt;
    use std::mem::size_of;
    use std::str::FromStr;

    use rstest::*;
    use rstest_reuse::*;

    use crate::prelude::*;

    #[fixture]
    pub fn board8() -> Board8 { Board8::blank() }

    #[fixture]
    pub fn board16() -> Board16 { Board16::blank() }

    #[fixture]
    pub fn board32() -> Board32 { Board32::blank() }

    #[fixture]
    pub fn board64() -> Board64 { Board64::blank() }

    #[test]
    fn size_of_boards() {
        assert_eq!(size_of::<Board8>(), 10);
        assert_eq!(size_of::<Board16>(), 20);
        assert_eq!(size_of::<Board32>(), 40);
        assert_eq!(size_of::<Board64>(), 80);
    }

    #[test]
    #[should_panic]
    fn board_from_invalid_str() {
        Board8::from_str("...").unwrap();
    }

    #[test]
    fn ceiling() {
        assert_eq!(board8().ceiling(), 8);
        assert_eq!(board16().ceiling(), 16);
        assert_eq!(board32().ceiling(), 32);
        assert_eq!(board64().ceiling(), 64);
    }

    #[test]
    fn from_board_shrink() {
        let mut board = Board64::blank();
        board.set_at(xy(0, 0));
        board.set_at(xy(0, 8));
        board.set_at(xy(0, 16));
        board.set_at(xy(0, 32));
        assert_eq!(board.count_blocks(), 4);
        assert_eq!(board.ceiling(), 64);

        let board = Board32::shrink_from(board);
        assert_eq!(board.count_blocks(), 3);
        assert_eq!(board.ceiling(), 32);

        let board = Board16::shrink_from(board);
        assert_eq!(board.count_blocks(), 2);
        assert_eq!(board.ceiling(), 16);

        let board = Board8::shrink_from(board);
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.ceiling(), 8);
    }

    #[test]
    fn from_board_expand() {
        let mut board = Board8::blank();
        board.set_at(xy(0, 0));
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.ceiling(), 8);

        let board = Board16::from(board);
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.ceiling(), 16);

        let board = Board32::from(board);
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.ceiling(), 32);

        let board = Board64::from(board);
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.ceiling(), 64);
    }

    #[template]
    #[rstest]
    #[case::board8(board8())]
    #[case::board16(board16())]
    #[case::board32(board32())]
    #[case::board64(board64())]
    fn all_boards(#[case] mut board: impl BoardOp) {}

    #[apply(all_boards)]
    fn works(mut board: impl BoardOp) {
        assert!(board.is_empty());
        assert_eq!(board.count_blocks(), 0);
        assert_eq!(board.well_top(), 0);

        {
            let location = xy(1, 2);
            assert!(board.is_free_at(location));
            board.set_at(location);
            assert!(!board.is_free_at(location));
        }

        assert!(!board.is_empty());
        assert_eq!(board.count_blocks(), 1);
        assert_eq!(board.well_top(), 3);

        {
            let location = xy(1, 2);
            assert!(board.is_occupied_at(location));
            board.unset_at(location);
            assert!(!board.is_occupied_at(location));
        }

        assert!(board.is_empty());
        assert_eq!(board.count_blocks(), 0);
        assert_eq!(board.well_top(), 0);
    }

    #[apply(all_boards)]
    fn clear_lines(mut board: impl BoardOp) {
        for x in 0..10 {
            board.set_at(xy(x, 0));
        }
        let cleared = board.clear_lines();
        assert_eq!(cleared.count(), 1);
        assert_eq!(board.count_blocks(), 0);

        board.invert();
        let row_key = board.filled_rows();
        assert_eq!(row_key.count(), board.ceiling());

        board.clear_lines();
        let row_key = board.filled_rows();
        assert_eq!(row_key.count(), 0);
    }

    #[apply(all_boards)]
    fn mirror(mut board: impl BoardOp) {
        board.set_at(xy(0, 0));
        board.mirror();
        assert!(board.is_occupied_at(xy(9, 0)));
    }

    #[apply(all_boards)]
    fn merge(board: impl BoardOp + Clone + PartialEq + fmt::Debug) {
        let mut left = board.clone();
        left.set_at(xy(0, 0));

        let mut right = board.clone();
        right.set_at(xy(9, 0));

        left.merge(&right);
        assert_eq!(left.count_blocks(), 2);
        assert!(left.is_occupied_at(xy(0, 0)));
        assert!(left.is_occupied_at(xy(9, 0)));
    }

    #[apply(all_boards)]
    fn clear_lines_partially(mut board: impl BoardOp + Clone + PartialEq + fmt::Debug) {
        for y in 4..board.ceiling() as i32 {
            board.set_at(xy(0, y));
        }
        assert_eq!(board.well_top(), board.ceiling());
        assert_eq!(board.count_blocks(), board.ceiling() - 4);

        board.clear_lines_partially(Lines::new(0b11110));
        assert_eq!(board.well_top(), board.ceiling() - 4);
        assert_eq!(board.count_blocks(), board.ceiling() - 5);
    }
}
