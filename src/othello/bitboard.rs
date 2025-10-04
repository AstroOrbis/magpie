use crate::othello::{
    Position,
    constants::{CCW_ROTATION_TABLE, CW_ROTATION_TABLE, POSITIONS},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a 8x8 board as a `u64`.
///
/// There are no restrictions placed on the bits represented, unlike the
/// similar [`Position`] where only a single bit may be set.
///
/// [`Position`]: crate::othello::Position
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    /// Retrieves the underlying u64.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert_eq!(b.raw(), 0);
    /// ```
    #[must_use]
    pub fn raw(self) -> u64 {
        self.0
    }

    /// Returns true if and only if no bits are set.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert!(b.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Counts the number of set bits.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::MAX.into();
    /// assert_eq!(b.count_set(), 64);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn count_set(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Counts the number of bits that are set to zero.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::MAX.into();
    /// assert_eq!(b.count_empty(), 0);
    /// ```
    #[must_use]
    pub fn count_empty(self) -> u8 {
        self.0.count_zeros().try_into().unwrap()
    }

    /// Extracts each bit as its own bitboard.
    ///
    /// For example, given the following (tiny) bitboard:
    /// ```text
    /// 111
    /// 000
    /// 111
    /// ```
    ///
    /// The iterator will break up that bitboard and yield the following
    /// bitboards:
    /// ```text
    /// 100    010    001    000    000    000    000    000    000
    /// 000 => 000 => 000 => 000 => 000 => 000 => 000 => 000 => 000
    /// 000    000    000    000    000    000    100    010    001
    /// ```
    /// The iterator always return 64 bitboards.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = 0.into();
    /// assert_eq!(b.bits().len(), 64);
    ///  ```
    #[must_use]
    pub fn bits(self) -> impl ExactSizeIterator<Item = Bitboard> {
        POSITIONS.iter().map(move |m| self & *m)
    }

    /// Extracts each bit set to one as its own bitboard.
    ///
    /// For example, given the following (tiny) bitboard:
    /// ```text
    /// 100
    /// 000
    /// 001
    /// ``` e
    ///
    /// The iterator will break up that bitboard and yield the following
    /// bitboards:
    /// ```text
    /// 100    000
    /// 000 => 000
    /// 000    001
    /// ```
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Bitboard;
    ///
    /// let b: Bitboard = u64::from(u32::MAX).into();
    /// assert_eq!(b.hot_bits().len(), 32);
    ///  ```
    #[must_use]
    pub fn hot_bits(self) -> impl ExactSizeIterator<Item = Position> {
        let positions = HotBits {
            remaining: self.count_set(),
            bitboard: self,
        };
        positions.into_iter()
    }

    // Bitboard format is:
    // a1, b1, c1, d1, e1, f1, g1, h1
    // a2, b2, c2, d2, e2, f2, g2, h2
    // etc

    // fn rotate_map(&self, map: &[u8; 64]) -> Bitboard {
    //     let mut out = 0u64;
    //
    //     for from in 0..64 {
    //         if (self.0 >> (63 - from)) & 1 != 0 {
    //             out |= 1u64 << (63 - map[from]);
    //         }
    //     }
    //
    //     Bitboard(out)
    // }

    pub fn ccw(&self) -> Self {
        let mut out = 0u64;
        for row in 0..8 {
            let byte = ((self.0 >> (row * 8)) & 0xFF) as usize;
            out |= CCW_ROTATION_TABLE[row][byte];
        }
        Bitboard(out)
    }
    pub fn cw(&self) -> Self {
        let mut out = 0u64;
        for row in 0..8 {
            let byte = ((self.0 >> (row * 8)) & 0xFF) as usize;
            out |= CW_ROTATION_TABLE[row][byte];
        }
        Bitboard(out)
    }

    pub fn flip180(&self) -> Self {
        Bitboard(self.0.reverse_bits())
    }

    /// Returns (ccw, d180, cw)
    pub fn rotations(&self) -> (Self, Self, Self) {
        (self.ccw(), self.flip180(), self.cw())
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let pos = rank * 8 + file;
                let mask = 1u64 << (63 - pos);
                let ch = if self.0 & mask != 0 { '1' } else { '.' };
                write!(f, "{} ", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_bitboard_rotations() {
    //  corner
    let b: Bitboard =
        0b00000011_00000001_00000100_00000000_00000000_00000000_00000000_00000000u64.into();
    println!("b:\n{b}");
    println!("ccw:\n{}", b.ccw());
    println!("180:\n{}", b.flip180());
    println!("cw:\n{}", b.cw());
}

#[cfg(kani)]
mod kani_bitboard_rotations {
    use super::Bitboard;

    #[kani::proof]
    fn rotations_preserve_count_set() {
        let b: Bitboard = kani::any();
        let (ccw, d180, cw) = b.rotations();
        let count = b.count_set();
        assert_eq!(count, ccw.count_set());
        assert_eq!(count, d180.count_set());
        assert_eq!(count, cw.count_set());
    }

    #[kani::proof]
    fn rotate_ccw_cw_is_identity() {
        let b: Bitboard = kani::any();
        let ccw = b.ccw();
        let cw = ccw.cw();
        assert_eq!(b.raw(), cw.raw());
    }

    #[kani::proof]
    fn double_cw_is_double_ccw_is_180() {
        let b: Bitboard = kani::any();
        let double_cw = b.cw().cw();
        let double_ccw = b.ccw().ccw();
        let d180 = b.flip180();
        assert_eq!(double_cw.raw(), double_ccw.raw());
        assert_eq!(double_cw.raw(), d180.raw());
    }
}

#[derive(Clone, Debug)]
struct HotBits {
    remaining: u8,
    bitboard: Bitboard,
}

#[derive(Clone, Debug)]
struct HotBitsIntoIterator {
    remaining: u8,
    bitboard: Bitboard,
}

impl IntoIterator for HotBits {
    type Item = Position;
    type IntoIter = HotBitsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        HotBitsIntoIterator {
            remaining: self.remaining,
            bitboard: self.bitboard,
        }
    }
}

impl Iterator for HotBitsIntoIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.is_empty() {
            None
        } else {
            self.remaining -= 1;
            let position = 1 << (63 - self.bitboard.raw().leading_zeros());
            self.bitboard ^= position;

            Some(Position::new_unchecked(position))
        }
    }
}

impl ExactSizeIterator for HotBitsIntoIterator {
    fn len(&self) -> usize {
        self.remaining.into()
    }
}

impl From<u64> for Bitboard {
    fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}
