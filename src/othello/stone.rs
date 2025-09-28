#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum that represents the two different possible stone colors available on a standard Othello board.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    /// Returns the opposite side of a standard Othello stone.
    ///
    /// # Examples
    /// ```rust
    /// use magpie::othello::Stone;
    ///
    /// assert_eq!(Stone::White, Stone::Black.flip());
    /// ```
    #[must_use]
    pub fn flip(&self) -> Self {
        match &self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn stone_flip_equality() {
    let stone: Stone = kani::any();
    assert_ne!(stone, stone.flip());
    assert_eq!(stone, stone.flip().flip());
}
