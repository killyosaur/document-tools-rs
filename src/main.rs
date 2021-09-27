use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("file-sample_100kB.doc")?;
    let mut buffer = Vec::new();

    // reads all the bytes
    let n = f.read_to_end(&mut buffer)?;

    println!("length of the buffer: {:?}", n);
    Ok(())
}
