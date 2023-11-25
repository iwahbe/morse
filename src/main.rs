use std::io::{Read, Write};

use clap::{Parser, ValueEnum};
use encoding::encode;

/// A morse code translator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    direction: Direction,

    /// The source of the translation
    #[arg(short, long)]
    src: Option<String>,

    /// The destination of the translation
    #[arg(short, long)]
    dst: Option<String>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
enum Direction {
    TextToMorse,
    MorseToText,
}
use Direction::*;

mod encoding;

fn main() {
    let arg = Args::parse();
    let src: String = match arg.src {
        Some(src) => std::fs::read_to_string(src).expect("Failed to read file"),
        None => {
            let mut s = String::new();
            let mut stdin = std::io::stdin().lock();
            stdin.read_to_string(&mut s).expect("Failed to read stdin");
            s
        }
    };
    match arg.direction {
        TextToMorse => {
            let encoded = encode(src.trim());
            eprintln!("Encoded: {encoded:?}");
            match arg.dst {
                Some(dst) => std::fs::write(dst, encoded).expect("Failed to write file"),
                None => std::io::stdout()
                    .lock()
                    .write_all(encoded.as_slice())
                    .expect("Failed to write to stdout"),
            }
        }
        MorseToText => todo!("Morse to text is unimplemented"),
    }
}
