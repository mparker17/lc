#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

extern crate unicode_segmentation;

use self::unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

/// Prints a summary of the contents of a grapheme counter.
pub fn print_summary(counter: HashMap<String, u64>) {
    for (key, val) in counter.iter() {
        println!("{}: {}", key, val);
    }
}

/// Counts all the graphemes in a string.
pub fn count_graphemes_in_string(to_parse: String, counter: &mut HashMap<String, u64>) {
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
