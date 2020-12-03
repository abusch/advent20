use std::env;
use std::path::Path;

use anyhow::*;

/// Read the input file for the current day's puzzle, i.e. `input/dayxx.txt`, and return its content as a String.
pub fn input_string() -> Result<String> {
    let executable_name = env::args_os()
        .next()
        .ok_or(format_err!("no executable name?"))?;
    let mut infile = Path::new("input").join(
        Path::new(&executable_name)
            .file_name()
            .ok_or(format_err!("no file name?"))?,
    );
    infile.set_extension("txt");
    std::fs::read_to_string(&infile).context("Could not read input file")
}
