use crate::{Place, DAH, DIT, END, PAUSE};

/// Encode a string slice of letters to a compressed binary of morse code.
pub fn encode<T: AsRef<str>>(src: T) -> Vec<u8> {
    let mut v = Vec::new();
    let mut place = Place::Zero;
    for c in src.as_ref().to_ascii_uppercase().chars() {
        encode_char(c, &mut place, &mut v)
    }
    emplace(&mut place, &mut v, |src, place| fmt(END, src, place));
    v
}

fn emplace<F>(place: &mut Place, v: &mut Vec<u8>, f: F)
where
    F: Fn(u8, Place) -> u8,
{
    if *place == Place::Zero {
        v.push(f(0, Place::Zero));
    } else {
        let c = *v.last().unwrap();
        *v.last_mut().unwrap() = f(c, *place);
    }
    *place = place.incr();
}

/// Encode a character onto v, where the next place to put the character is described by place.
///
/// If `place` is `Zero`, then a new u8 will be pushed onto v.
///
/// Note: Encoding `c` might require pushing multiple bytes onto `v`.
fn encode_char(c: char, place: &mut Place, v: &mut Vec<u8>) {
    let dit = |src, place| fmt(DIT, src, place);
    let dah = |src, place| fmt(DAH, src, place);
    let char_end = |src, place| fmt(PAUSE, src, place);

    macro_rules! letter_helper {
        ($typ:ident) => {
            emplace(place, v, $typ)
        };
        ($typ:ident, $($rest:ident),+) => {
            letter_helper!($typ);
            letter_helper!($($rest),+)
        }
    }

    // letter calls emplace on a series
    macro_rules! letter {
        ($($typ:ident),+) => { {
            letter_helper!($($typ),+);
            emplace(place, v, char_end)
        } }
    }
    match c {
        // Letters
        'A' => letter!(dit, dah),           // '.-'
        'B' => letter!(dah, dit, dit, dit), // '-...'
        'C' => letter!(dah, dit, dah, dit), // '-.-.'
        'D' => letter!(dah, dit, dit),      // '-..'
        'E' => letter!(dit),                // '.'
        'F' => letter!(dit, dit, dah, dit), // '..-.'
        'G' => letter!(dah, dah, dit),      // '--.'
        'H' => letter!(dit, dit, dit, dit), // '....'
        'I' => letter!(dit, dit),           // '..'
        'J' => letter!(dit, dah, dah, dah), // '.---'
        'K' => letter!(dah, dit, dah),      // '-.-',
        'L' => letter!(dit, dah, dit, dit), // '.-..'
        'M' => letter!(dah, dah),           // '--'
        'N' => letter!(dah, dit),           // '-.'
        'O' => letter!(dah, dah, dah),      // '---'
        'P' => letter!(dit, dah, dah, dit), // '.--.'
        'Q' => letter!(dah, dah, dit, dah), // '--.-'n
        'R' => letter!(dit, dah, dit),      // '.-.'
        'S' => letter!(dit, dit, dit),      // '...'
        'T' => letter!(dah),                // '-'
        'U' => letter!(dit, dit, dah),      // '..-'
        'V' => letter!(dit, dit, dit, dah), // '...-'
        'W' => letter!(dit, dah, dah),      //  '.--'
        'X' => letter!(dah, dit, dit, dah), // '-..-'
        'Y' => letter!(dah, dit, dah, dah), // '-.--'
        'Z' => letter!(dah, dah, dit, dit), // '--..'

        // Space
        ' ' => letter!(char_end),

        // Numbers
        '1' => letter!(dit, dah, dah, dah, dah), // '.----'
        '2' => letter!(dit, dit, dah, dah, dah), // '..---'
        '3' => letter!(dit, dit, dit, dah, dah), // '...--'
        '4' => letter!(dit, dit, dit, dit, dah), // '....-'
        '5' => letter!(dit, dit, dit, dit, dit), // '.....'
        '6' => letter!(dah, dit, dit, dit, dit), // '-....'
        '7' => letter!(dah, dah, dit, dit, dit), // '--...'
        '8' => letter!(dah, dah, dah, dit, dit), // '---..'
        '9' => letter!(dah, dah, dah, dah, dit), // '----.'
        '0' => letter!(dah, dah, dah, dah, dah), // '-----'

        // Punctuation
        ':' => letter!(dah, dit, dit, dah, dah), // '-..--'
        '.' => letter!(dit, dah, dit, dah, dit, dah), // '.-.-.-'
        '?' => letter!(dit, dit, dah, dah, dit, dit), // '..--..'
        '/' => letter!(dah, dit, dit, dah, dit), // '-..-.'
        '-' => letter!(dah, dit, dit, dit, dit, dah), // '-....-'
        '(' => letter!(dah, dit, dah, dah, dit), // '-.--.'
        ')' => letter!(dah, dit, dah, dah, dit, dah), // '-.--.-'

        _ => panic!("Unknown char: {:?}", c),
    }
}

fn fmt(sym: u8, src: u8, place: Place) -> u8 {
    match place {
        Place::Zero => src | (sym << 0),
        Place::One => src | (sym << 2),
        Place::Two => src | (sym << 4),
        Place::Three => src | (sym << 6),
    }
}
