use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

mod lc;

fn main() {
    let stdin = io::stdin();
    let mut counter : HashMap<String, u64> = HashMap::new();
    let lines = stdin.lock().lines();

    // Loop through all the lines, and count the graphemes in each one.
    for line in lines {
        lc::count_graphemes_in_string(line.unwrap(), &mut counter);
    }

    lc::print_summary(counter);
}
