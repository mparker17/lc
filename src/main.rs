extern crate unicode_segmentation;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let stdin = io::stdin();
    let mut counter : HashMap<String, u64> = HashMap::new();

    // Read one line at a time.
    for line in stdin.lock().lines() {
        let line_string = line.unwrap();

        // Loop through each character in the current line...
        for grapheme in UnicodeSegmentation::graphemes(line_string.as_str(), true) {
            // If the character we are looking at already exists in the counter
            // hash, get its value. Otherwise, start a new counter at zero.
            let count = counter.entry(grapheme.to_string()).or_insert(0);

            // In either case, increment the counter.
            *count += 1;
        }
    }

    // Print summary.
    for (key, val) in counter.iter() {
        println!("key: {} val: {}", key, val);
    }
}
