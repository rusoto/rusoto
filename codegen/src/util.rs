use std::collections::BTreeMap;

/// Returns the first value from a `String`-keyed `BTreeMap` matching the
/// provided case-insensitive `String` key.
#[inline]
pub fn case_insensitive_btreemap_get<'a, V>(map: &'a BTreeMap<String, V>, key: &str) -> Option<&'a V> {
    map.iter()
        .filter(|&(k, _)| k.to_lowercase() == key.to_lowercase())
        .map(|(_, v)| v)
        .next()
}
