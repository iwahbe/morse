mod decoding;
mod encoding;

pub use crate::decoding::decode;
pub use crate::encoding::encode;

/// A morse code dit, representing a short tap.
pub(crate) const DIT: u8 = 2;

/// A morse code dah, representing a long tap.
pub(crate) const DAH: u8 = 3;

/// A symbol that represents a pause on the line.
pub(crate) const PAUSE: u8 = 0;

/// An optional indication that the message is over.
///
/// This is necessary since morse operates on a bits, but we are unable to control how
/// many bits are sent. Only bytes can be controlled, meaning that there might be as many
/// as 3 extra zero-value bit pairs at the end of the [u8] stream. For example, the
/// encoding of `e` is `1000`. If the entire message is `e`, then we will send a `[u8; 1]
/// = [0b10000000]` which is identical to the sequence for `e `.
///
/// By ending sequences with `END`, we can distinguish these cases (`e` = `[0b10000100]`,
/// `e ` = `[0b10000000, 0b01000000]`).
pub(crate) const END: u8 = 1; // The message is over

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

#[cfg(test)]
mod test {

    #[test]
    fn roundtrip() {
        let test = |s: &'static str| {
            let encoded = super::encode(s);
            let decoded = super::decode(encoded);
            assert_eq!(s, decoded);
        };

        test("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); // letters
        test("1234567890"); // numbers
        test(":.?/-() "); // symbols

        // Test that we correctly cap partial bytes
        test("E");
        test("E ");
        test("E  ");
        test("E   ");
    }
}
