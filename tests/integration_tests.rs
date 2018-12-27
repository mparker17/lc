use lc;
use std::collections::HashMap;

#[test]
fn counts_graphemes() {
    let mut counter: HashMap<String, u64> = HashMap::new();
    let test = "test ¢¢¢ test हहह €€€ 𐍈𐍈𐍈 स्स्स् test";

    lc::count_graphemes_in_string(test, &mut counter);

    // Check the number of graphemes found.
    assert_eq!(9, counter.len());

    // Test 1-byte UTF-8 characters.
    assert_eq!(6, *counter.get("t").unwrap());
    assert_eq!(3, *counter.get("e").unwrap());
    assert_eq!(3, *counter.get("s").unwrap());
    assert_eq!(7, *counter.get(" ").unwrap());

    // Test 2-byte UTF-8 characters.
    assert_eq!(3, *counter.get("¢").unwrap());

    // Test 3-byte UTF-8 characters.
    assert_eq!(3, *counter.get("ह").unwrap());
    assert_eq!(3, *counter.get("€").unwrap());

    // Test 4-byte UTF-8 characters.
    assert_eq!(3, *counter.get("𐍈").unwrap());

    // Test grapheme clusters: स् is a combination of the character स and the
    // combining diacritical mark ्.
    assert_eq!(3, *counter.get("स्").unwrap());
}
