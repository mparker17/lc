#[macro_use]
extern crate clap;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use clap::App;

mod lc;

fn main() {
    App::new("lc - letter count")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The lc utility displays the number of instances of each letter contained in standard input to the standard output. Newline characters are not counted.")
        .usage("lc < $file_to_parse")
        .get_matches();

    let stdin = io::stdin();
    let mut counter : HashMap<String, u64> = HashMap::new();
    let lines = stdin.lock().lines();

    // Loop through all the lines, and count the graphemes in each one.
    for line in lines {
        lc::count_graphemes_in_string(line.unwrap(), &mut counter);
    }

    lc::print_summary(counter);
}
