use std::env;
use std::path::Path;

/// Read the input file for the current day's puzzle, i.e. `input/dayxx.txt`, and return its content as a String.
pub fn input_string() -> String {
    let mut infile = Path::new("input").join(
        Path::new(&env::args_os().next().expect("no executable name"))
            .file_name()
            .expect("no file name?"),
    );
    infile.set_extension("txt");
    std::fs::read_to_string(&infile).unwrap_or_else(|e| panic!("could not read input file: {}", e))
}
