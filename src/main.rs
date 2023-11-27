use std::{
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::{Parser, ValueEnum};

use morse::{decode, encode};

/// A morse code translator
#[derive(Parser, Debug)]
#[command(author, version = "0.0.1", about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    direction: Direction,

    /// The source of the translation
    #[arg(short, long)]
    src: Option<PathBuf>,

    /// The destination of the translation
    #[arg(short, long)]
    dst: Option<PathBuf>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
enum Direction {
    TextToMorse,
    MorseToText,
}
use Direction::*;

fn main() -> io::Result<()> {
    let arg = Args::parse();

    // Read the src into a u8 buffer.
    let src = match arg.src {
        Some(src) => std::fs::read(src)?,
        None => {
            let mut v = Vec::new();
            let mut stdin = std::io::stdin().lock();
            stdin.read_to_end(&mut v)?;
            v
        }
    };

    match arg.direction {
        TextToMorse => {
            let encoded = encode(
                String::from_utf8(src)
                    .expect("Source is not valid utf-8")
                    .trim(),
            );
            match arg.dst {
                Some(dst) => std::fs::write(dst, encoded),
                None => std::io::stdout().lock().write_all(encoded.as_slice()),
            }
        }
        MorseToText => {
            let mut decoded = decode(src);
            decoded.push('\n');
            match arg.dst {
                Some(dst) => std::fs::write(dst, decoded.as_bytes()),
                None => std::io::stdout().lock().write_all(decoded.as_bytes()),
            }
        }
    }
}
