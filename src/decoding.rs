use crate::encoding::{Place, DAH, DIT, END};

pub fn decode<T: AsRef<[u8]>>(src: T) -> String {
    let mut src = src.as_ref();
    let mut s = String::new();
    let mut place = Place::Zero;

    while !src.is_empty() {
        let (c, rest) = decode_char(src, &mut place);
        s.push(c);
        src = rest;
    }

    s
}

/// Decode a morse character from the beginning of src (at byte location position place).
///
/// The parsed character and un-parsed tape are returned while `place` is mutated in
/// place.
fn decode_char<'a>(mut src: &'a [u8], place: &mut Place) -> (char, &'a [u8]) {
    // We will encode the next morse char into a u16: c. All morse chars we know how to
    // decode fit into a u16, but they don't fit into a u8.
    let mut c: u16 = 0;

    // idx represents the offset into c that the next byte pair should go into.
    let mut idx: u8 = 0;

    loop {
        if src.is_empty() {
            debug_assert!(
                c == 0,
                "Invalid input: src is empty before char finished decoding. {idx} bytes decoded: {c:x}"
            );
            break (' ', src);
        }

        // r is the 2 bytes we are using for the next sequence. It is already aligned.
        let r = place.idx(src[0]);

        // Move the reader infra (src & place) to the next byte pair.
        *place = place.incr();
        if *place == Place::Zero {
            src = &src[1..];
        }

        // If we have read some bytes and we encounter a stop sequence, break from the
        // loop and return the captured byte (along with the remaining tape).
        if r == 0 && idx != 0 {
            break (decode_morse(c), src);
        }

        c = c | ((r as u16) << idx); // Set r into c at the appropriate byte index.
        idx += 2;
        debug_assert!(idx <= 16, "{idx:?} has exceeded the size of u16")
    }
}

/// Decode a left aligned morse char into a `char`.
fn decode_morse(c: u16) -> char {
    // Each morse sequence fits into a u16 byte.
    //
    // While it is possible to deduce each number for a given symbol from parsing it's
    // representation, I found it easier to use guess and check.

    // letter defines a representation (u16) for a char by combining `dit`s and `dah`s.
    macro_rules! letter {
        ($h:tt, $($sep:tt),+) => {
            (letter!($h) | (letter!($($sep),+)<<2))
        };
        (dit) => {(DIT as u16)};
        (dah) => {(DAH as u16)};
        (char_end) => {(END as u16)};
    }

    // decode creates a match statement where each pattern is a constant result of some
    // expression. This is necessary because match's pattern syntax conflicts with rust's
    // normal expression syntax: `x | y` is `x` bit-or `y` in an expression context but
    // `x` or `y` in a pattern context.
    macro_rules! decode {
        ($($name:ident => $num:expr, $result:expr),+) => { {
            $(const $name: u16 = $num; )+
                match c {
                    $($name => $result,)+
                    _ => panic!("Unknown char {c:?}"),
                }
        } }
    }

    decode! {
        A => letter!(dit, dah), 'A',           // '.-'
        B => letter!(dah, dit, dit, dit), 'B', // '-...'
        C => letter!(dah, dit, dah, dit), 'C', // '-.-.'
        D => letter!(dah, dit, dit), 'D',      // '-..'
        E => letter!(dit), 'E',                // '.'
        F => letter!(dit, dit, dah, dit), 'F', // '..-.'
        G => letter!(dah, dah, dit), 'G',      // '--.'
        H => letter!(dit, dit, dit, dit), 'H', // '....'
        I => letter!(dit, dit), 'I',           // '..'
        J => letter!(dit, dah, dah, dah), 'J', // '.---'
        K => letter!(dah, dit, dah), 'K',      // '-.-',
        L => letter!(dit, dah, dit, dit), 'L', // '.-..'
        M => letter!(dah, dah), 'M',           // '--'
        N => letter!(dah, dit), 'N',           // '-.'
        O => letter!(dah, dah, dah), 'O',      // '---'
        P => letter!(dit, dah, dah, dit), 'P', // '.--.'
        Q => letter!(dah, dah, dit, dah), 'Q', // '--.-'n
        R => letter!(dit, dah, dit), 'R',      // '.-.'
        S => letter!(dit, dit, dit), 'S',      // '...'
        T => letter!(dah), 'T',                // '-'
        U => letter!(dit, dit, dah), 'U',      // '..-'
        V => letter!(dit, dit, dit, dah), 'V', // '...-'
        W => letter!(dit, dah, dah), 'W',      //  '.--'
        X => letter!(dah, dit, dit, dah), 'X', // '-..-'
        Y => letter!(dah, dit, dah, dah), 'Y', // '-.--'
        Z => letter!(dah, dah, dit, dit), 'Z', // '--..'

        // Space
        SPACE => letter!(char_end), ' ',

        // Numbers
        ONE => letter!(dit, dah, dah, dah, dah), '1',   // '.----'
        TWO => letter!(dit, dit, dah, dah, dah), '2',   // '..---'
        THREE => letter!(dit, dit, dit, dah, dah), '3', // '...--'
        FOUR => letter!(dit, dit, dit, dit, dah), '4',  // '....-'
        FIVE => letter!(dit, dit, dit, dit, dit), '5',  // '.....'
        SIX => letter!(dah, dit, dit, dit, dit), '6',   // '-....'
        SEVEN => letter!(dah, dah, dit, dit, dit), '7', // '--...'
        EIGHT => letter!(dah, dah, dah, dit, dit), '8', // '---..'
        NINE => letter!(dah, dah, dah, dah, dit), '9',  // '----.'
        ZERO => letter!(dah, dah, dah, dah, dah), '0',  // '-----'

        // Punctuation
        COLEN => letter!(dah, dit, dit, dah, dah), ':', // '-..--'
        PERIOD => letter!(dit, dah, dit, dah, dit, dah), '.',// '.-.-.-'
        QUESTION => letter!(dit, dit, dah, dah, dit, dit), '?',// '..--..'
        SLASH => letter!(dah, dit, dit, dah, dit), '/',// '-..-.'
        DASH => letter!(dah, dit, dit, dit, dit, dah), '-', // '-....-'
        PAREN_OPEN => letter!(dah, dit, dah, dah, dit), '(',  // '-.--.'
        PAREN_CLOSE => letter!(dah, dit, dah, dah, dit, dah), ')' // '-.--.-'
    }
}
