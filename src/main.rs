use clap::Parser;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use crate::expander::expand_file;

mod expander;

#[derive(Parser, Debug)]
struct Args {
    source_file: PathBuf,

    #[arg(short, long, default_value = ".")]
    lib: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let contents = expand_file(args.source_file, args.lib)?;

    let mut output = File::create("000SUBMITME.cpp")?;
    output.write_all(contents.as_bytes())?;

    Ok(())
}
