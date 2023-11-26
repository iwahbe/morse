use crate::encoding::Place;

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

    match c {
        // Space
        0 => ' ',

        // Letters
        14 => 'A',
        171 => 'B',
        187 => 'C',
        43 => 'D',
        2 => 'E',
        186 => 'F',
        47 => 'G',
        170 => 'H',
        10 => 'I',
        254 => 'J',
        59 => 'K',
        174 => 'L',
        15 => 'M',
        11 => 'N',
        63 => 'O',
        190 => 'P',
        239 => 'Q',
        46 => 'R',
        42 => 'S',
        3 => 'T',
        58 => 'U',
        234 => 'V',
        62 => 'W',
        235 => 'X',
        251 => 'Y',
        175 => 'Z',

        // Numbers
        1022 => '1',
        1018 => '2',
        1002 => '3',
        938 => '4',
        682 => '5',
        683 => '6',
        687 => '7',
        703 => '8',
        767 => '9',
        1023 => '0',

        // Punctuation
        1003 => ':',
        3822 => '.',
        2810 => '?',
        747 => '/',
        3755 => '-',
        763 => '(',
        3835 => ')',
        _ => panic!("Unknown char {c:?}"),
    }
}
