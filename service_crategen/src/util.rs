use std::collections::BTreeMap;

/// Capitalize the first character in the given string.
/// If the input string is empty an empty string is returned.
pub fn capitalize_first<S>(word: S) -> String
where
    S: Into<String>,
{
    let s = word.into();
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Returns the first value from a `String`-keyed `BTreeMap` matching the
/// provided case-insensitive `String` key.
#[inline]
pub fn case_insensitive_btreemap_get<'a, V>(
    map: &'a BTreeMap<String, V>,
    key: &str,
) -> Option<&'a V> {
    map.iter()
        .filter(|&(k, _)| k.to_lowercase() == key.to_lowercase())
        .map(|(_, v)| v)
        .next()
}

#[test]
fn capitalize_first_test() {
    assert_eq!(capitalize_first("a &str test"), "A &str test".to_owned());
    assert_eq!(
        capitalize_first("a String test".to_owned()),
        "A String test".to_owned()
    );
}
