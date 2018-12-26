//! Letter count: executable.
//!
//! Displays the number of instances of each letter contained in each input
//! `file`, or standard input (if no `file` is specified) to the standard
//! output. Newline characters are not counted.

use clap::crate_authors;
use clap::crate_version;
use clap::App;
use clap::Arg;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

/// Main entry point for the letter count utility.
fn main() {
    let matches = App::new("lc - letter count")
        .version(crate_version!())
        .author(crate_authors!())
        .about(
            "The lc utility displays the number of instances of each letter contained in each \
             input, or standard input (if no file is specified) to the standard output. \
             Newline characters are not counted.",
        )
        .arg(Arg::with_name("file").help("Sets the input file to use"))
        .get_matches();

    let mut counter: HashMap<String, u64> = HashMap::new();

    // If a file to read from was passed in, use it. Otherwise, use standard
    // input.
    let file: Box<Read> = match matches.value_of("file") {
        Some(filename) => Box::new(File::open(filename).unwrap()),
        None => Box::new(io::stdin()),
    };
    let reader = BufReader::new(file);

    // Loop through all the lines, and count the graphemes in each one.
    for line in reader.lines() {
        lc::count_graphemes_in_string(&line.unwrap(), &mut counter);
    }

    lc::print_summary(&counter);
}
