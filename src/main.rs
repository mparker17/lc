extern crate unicode_segmentation;

use std::io;
use std::io::Lines;
use std::io::prelude::*;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

/// Prints a summary of the contents of a grapheme counter.
fn print_summary(counter: HashMap<String, u64>) {
    for (key, val) in counter.iter() {
        println!("{}: {}", key, val);
    }
}

/// Counts graphemes in a set of lines.
fn count_graphemes_in_lines(lines: Lines<std::io::StdinLock>, mut counter: &mut HashMap<String, u64>) {
    for line in lines {
        let line_string = line.unwrap();

        count_graphemes_in_string(line_string, &mut counter);
    }
}

/// Counts the number of graphemes in a string.
fn count_graphemes_in_string(to_parse: String, mut counter: &mut HashMap<String, u64>) {
    // The UnicodeSegmentation library can only handle string slices. Convert
    // the input string to that before parsing.
    let to_parse_str = to_parse.as_str();

    // Loop through each character in the current string...
    for grapheme in UnicodeSegmentation::graphemes(to_parse_str, true) {
        // If the character we are looking at already exists in the counter
        // hash, get its value. Otherwise, start a new counter at zero.
        let count = counter.entry(grapheme.to_string()).or_insert(0);

        // In either case, increment the counter.
        *count += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut counter : HashMap<String, u64> = HashMap::new();
    let lines = stdin.lock().lines();

    count_graphemes_in_lines(lines, &mut counter);
    print_summary(counter);
}
