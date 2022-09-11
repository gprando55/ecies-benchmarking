#![allow(unused)]
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("./rust-1kb.txt")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    f.read(&mut buffer)?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    // read into a String, so that you don't need to do the conversion.
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // and more! See the other methods for more details.
    Ok(())
}