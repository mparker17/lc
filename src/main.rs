extern crate unicode_segmentation;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let stdin = io::stdin();
    let mut counter = HashMap::new();

    // Read one line at a time.
    for line in stdin.lock().lines() {
        let line_string = line.unwrap();

        // Loop through each character in the current line. If the character we
        // are looking at already exists in the counter hash, increment the
        // counter. Otherwise, start a new counter for the character.
        for grapheme in UnicodeSegmentation::graphemes(line_string.as_str(), true) {
            let num = match counter.get(grapheme) {
                Some(value) => *value,
                None => 0 as u64,
            };
            counter.insert(grapheme.to_string(), num + 1);
        }
    }

    // Print summary.
    for (key, val) in counter.iter() {
        println!("key: {} val: {}", key, val);
    }
}
