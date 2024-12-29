use std::{fmt, ops};

use crate::internal_macros::forward_ref_op;

/// A key that holds a flag for each row. For example, `key: 0b1001` represents that rows 0 and 3 are on-bits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Lines {
    pub key: u64,
}

impl Lines {
    #[inline]
    pub const fn new(key: u64) -> Self {
        Self { key }
    }

    #[inline]
    pub fn from_slice(ys: &[u8]) -> Self {
        Self::from_iter(ys.iter().map(|&y| y))
    }

    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::blank(), Lines::new(0));
    /// ```
    #[inline]
    pub const fn blank() -> Self {
        Lines::new(0)
    }

    /// Returns a line where y is on-bit.
    ///
    /// Panic if y is 64 or greater.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new_at(0), Lines::new(0b00001));
    /// assert_eq!(Lines::new_at(1), Lines::new(0b00010));
    /// assert_eq!(Lines::new_at(4), Lines::new(0b10000));
    /// ```
    #[inline]
    pub const fn new_at(y: u8) -> Self {
        Lines::new(1 << y)
    }

    /// Returns a line filled up to a specified height.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::filled_up_to(0), Lines::new(0b00000));
    /// assert_eq!(Lines::filled_up_to(1), Lines::new(0b00001));
    /// assert_eq!(Lines::filled_up_to(5), Lines::new(0b11111));
    /// ```
    #[inline]
    pub const fn filled_up_to(height: u8) -> Self {
        Self::new((1u64 << height) - 1)
    }

    /// Returns the height to the highest line. If the lines are empty, return 0.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b00000).top(), 0);
    /// assert_eq!(Lines::new(0b00001).top(), 1);
    /// assert_eq!(Lines::new(0b10000).top(), 5);
    /// ```
    #[inline]
    pub fn top(&self) -> u32 {
        64 - self.key.leading_zeros()
    }

    /// Count the lines that are on-bits.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b0000).count(), 0);
    /// assert_eq!(Lines::new(0b0001).count(), 1);
    /// assert_eq!(Lines::new(0b11010).count(), 3);
    /// ```
    #[inline]
    pub fn count(&self) -> u32 {
        self.key.count_ones()
    }

    /// Returns whether it is on-bit or not.
    /// Panics if `y` is over 64.
    /// ```
    /// use bitris::prelude::*;
    /// let lines = Lines::new(0b01001);
    /// assert_eq!(lines.test_at(0), true);
    /// assert_eq!(lines.test_at(1), false);
    /// assert_eq!(lines.test_at(2), false);
    /// assert_eq!(lines.test_at(3), true);
    /// assert_eq!(lines.test_at(63), false);
    /// ```
    #[inline]
    pub fn test_at(&self, y: usize) -> bool {
        0 < (self.key & (1u64 << y))
    }

    /// Returns all y-coordinate of the enabled rows.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b00000).ys(), vec![]);
    /// assert_eq!(Lines::new(0b00001).ys(), vec![0]);
    /// assert_eq!(Lines::new(0b10100).ys(), vec![2, 4]);
    /// assert_eq!(Lines::new(1 << 63).ys(), vec![63]);
    /// ```
    #[inline]
    pub fn ys(&self) -> Vec<u8> {
        self.ys_iter().collect()
    }

    /// Returns all y-coordinate of the enabled rows. See `Self::ys()`.
    #[inline]
    pub fn ys_iter(&self) -> impl Iterator<Item = u8> {
        let mut vec = Vec::with_capacity(self.count() as usize);
        let mut key = self.key;
        while key != 0 {
            let y = key.trailing_zeros();
            key -= 1u64 << y;
            vec.push(y as u8);
        }
        vec.into_iter()
    }

    /// Returns true if the enabled row does not exist.
    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::blank().is_blank(),true);
    /// assert_eq!(Lines::new(0b00001).is_blank(), false);
    /// ```
    #[inline]
    pub fn is_blank(self) -> bool {
        self.key == 0
    }

    /// Inserts new off-bits at the specified mask rows (interception) while keeping the on-bit correlation of the source.
    /// ```
    /// use bitris::prelude::*;
    /// let lines = Lines::new(0b00111111);
    /// let interception = Lines::new(0b00100010);
    /// assert_eq!(lines.intercept(interception), Lines::new(0b11011101));
    /// ```
    #[inline]
    pub fn intercept(self, interception: Lines) -> Lines {
        // TODO This can be replaced in PDEP.
        let mut key: u64 = self.key;
        let mut remaining: u64 = interception.key;
        while 0 < remaining {
            let next_remaining = remaining & (remaining - 1);
            let current_bit = remaining - next_remaining;
            remaining = next_remaining;
            let lower_mask = current_bit - 1;
            let upper_mask = !lower_mask;
            key = ((key & upper_mask) << 1) | (key & lower_mask);
        }
        Lines::new(key)
    }

    /// Returns true if there is an overlap.
    ///
    /// Note that returns false always if the other is blank.
    /// ```
    /// use bitris::prelude::*;
    /// assert!(Lines::new(0b01010101).overlaps(&Lines::new(0b00000001)));
    /// assert!(!Lines::new(0b01010101).overlaps(&Lines::new(0b10101010)));
    /// assert!(!Lines::new(0b01010101).overlaps(&Lines::blank()));
    /// ```
    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        0 < (self.key & other.key)
    }

    /// Returns true when it has all the other's on-bits.
    ///
    /// Note that returns true always if the other is blank.
    /// ```
    /// use bitris::prelude::*;
    /// assert!(Lines::new(0b11110000).contains_all(&Lines::new(0b01100000)));
    /// assert!(Lines::new(0b11110000).contains_all(&Lines::new(0b11110000)));
    /// assert!(!Lines::new(0b11110000).contains_all(&Lines::new(0b00011000)));
    /// assert!(Lines::new(0b11110000).contains_all(&Lines::blank()));
    /// ```
    #[inline]
    pub fn contains_all(&self, other: &Self) -> bool {
        (self.key & other.key) == other.key
    }
}

impl fmt::Display for Lines {
    /// ```
    /// use std::fmt::format;
    /// use bitris::prelude::*;
    /// assert_eq!(format!("{}", Lines::new(0b00000)), "Lines (key=0, ys=[])");
    /// assert_eq!(format!("{}", Lines::new(0b00001)), "Lines (key=1, ys=[0])");
    /// assert_eq!(format!("{}", Lines::new(0b10100)), "Lines (key=20, ys=[2, 4])");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lines (key={}, ys=[", self.key)?;

        let ys = self.ys();
        if !ys.is_empty() {
            write!(f, "{}", ys[0])?;
            for y in &ys[1..] {
                write!(f, ", {}", y)?
            }
        }

        write!(f, "])")
    }
}

impl FromIterator<u8> for Lines {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let key = iter.into_iter().fold(0u64, |key, y| key | (1u64 << y));
        Lines::new(key)
    }
}

impl ops::BitAnd<Lines> for Lines {
    type Output = Lines;

    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b1001) & Lines::new(0b1010), Lines::new(0b1000));
    /// ```
    fn bitand(self, rhs: Lines) -> Self::Output {
        Lines::new(self.key & rhs.key)
    }
}

impl ops::BitAndAssign<Lines> for Lines {
    /// ```
    /// use bitris::prelude::*;
    /// let mut lines = Lines::new(0b1001);
    /// lines &= Lines::new(0b1010);
    /// assert_eq!(lines, Lines::new(0b1000));
    /// ```
    fn bitand_assign(&mut self, rhs: Lines) {
        self.key &= rhs.key;
    }
}

impl ops::BitOr<Lines> for Lines {
    type Output = Lines;

    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b1001) | Lines::new(0b1010), Lines::new(0b1011));
    /// ```
    fn bitor(self, rhs: Lines) -> Self::Output {
        Lines::new(self.key | rhs.key)
    }
}

impl ops::BitOrAssign<Lines> for Lines {
    /// ```
    /// use bitris::prelude::*;
    /// let mut lines = Lines::new(0b1001);
    /// lines |= Lines::new(0b1010);
    /// assert_eq!(lines, Lines::new(0b1011));
    /// ```
    fn bitor_assign(&mut self, rhs: Lines) {
        self.key |= rhs.key;
    }
}

impl ops::BitXor<Lines> for Lines {
    type Output = Lines;

    /// ```
    /// use bitris::prelude::*;
    /// assert_eq!(Lines::new(0b1001) ^ Lines::new(0b1010), Lines::new(0b0011));
    /// ```
    fn bitxor(self, rhs: Lines) -> Self::Output {
        Lines::new(self.key ^ rhs.key)
    }
}

impl ops::BitXorAssign<Lines> for Lines {
    /// ```
    /// use bitris::prelude::*;
    /// let mut lines = Lines::new(0b1001);
    /// lines ^= Lines::new(0b1010);
    /// assert_eq!(lines, Lines::new(0b0011));
    /// ```
    fn bitxor_assign(&mut self, rhs: Lines) {
        self.key ^= rhs.key;
    }
}

forward_ref_op! { Lines, & Lines, = Lines }
forward_ref_op! { Lines, &= Lines }
forward_ref_op! { Lines, | Lines, = Lines }
forward_ref_op! { Lines, |= Lines }
forward_ref_op! { Lines, ^ Lines, = Lines }
forward_ref_op! { Lines, ^= Lines }

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn intercept() {
        assert_eq!(
            Lines::new(0b00111111).intercept(Lines::new(0)),
            Lines::new(0b00111111)
        );
        assert_eq!(
            Lines::new(0b00111111).intercept(Lines::filled_up_to(1)),
            Lines::new(0b00111111 << 1)
        );
        assert_eq!(
            Lines::new(0b00111111).intercept(Lines::filled_up_to(5)),
            Lines::new(0b00111111 << 5)
        );
        assert_eq!(
            Lines::new(1).intercept(Lines::filled_up_to(63)),
            Lines::new(1 << 63)
        );
        assert_eq!(
            Lines::new(0).intercept(Lines::filled_up_to(63)),
            Lines::new(0)
        );
        assert_eq!(
            Lines::new(0b00111111).intercept(Lines::new_at(63)),
            Lines::new(0b00111111)
        );
    }
}
