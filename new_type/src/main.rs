use std::collections::HashMap;

use crate::case_insensitive_str::CaseInsensitiveString;

mod case_insensitive_str;

fn main() {
    let first = CaseInsensitiveString::from("Content-Type");
    let second = CaseInsensitiveString::from("content-type");

    assert_eq!(first, second);

    let mut headers = HashMap::new();

    headers.insert(
        CaseInsensitiveString::from("Content-Type"),
        "application/json",
    );

    headers.insert(
        CaseInsensitiveString::from("Authorization"),
        "Bearer token",
    );

    // Different capitalization still finds the same key.
    let key = CaseInsensitiveString::from("CONTENT-TYPE");

    println!("{}: {}", key, headers[&key]);

    // Inserting this replaces the existing Content-Type entry.
    headers.insert(
        CaseInsensitiveString::from("content-TYPE"),
        "text/plain",
    );

    assert_eq!(headers.len(), 2);
    assert_eq!(headers[&key], "text/plain");
}