mod decoding;
mod encoding;

pub use crate::decoding::decode;
pub use crate::encoding::encode;

pub(crate) const DIT: u8 = 2;
pub(crate) const DAH: u8 = 3;
pub(crate) const END: u8 = 0;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Place {
    Zero,
    One,
    Two,
    Three,
}

impl Place {
    pub fn incr(self: Self) -> Self {
        match self {
            Place::Zero => Place::One,
            Place::One => Place::Two,
            Place::Two => Place::Three,
            Place::Three => Place::Zero,
        }
    }

    /// Get the 2 bits at the place.
    ///
    /// Possible return values are 0, 1, 2 and 3.
    pub fn idx(self: Self, b: u8) -> u8 {
        let shift = match self {
            Place::Zero => 0,
            Place::One => 2,
            Place::Two => 4,
            Place::Three => 6,
        };
        (b & (3 << shift)) >> shift
    }
}
