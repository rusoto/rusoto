use crate::generated::{AttributeValue};

#[test]
fn attribute_value_default_is_empty() {
    let all_default = AttributeValue{
        ..Default::default()
    };

    let serialized = serde_json::to_string(&all_default).unwrap();
    assert_eq!(&serialized, "{}");
}

#[test]
fn attribute_value_with_blob_contains_only_blob() {
    let all_default = AttributeValue{
        b: Some("foo".bytes().collect()),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&all_default).unwrap();
    assert_eq!(&serialized, r#"{"B":"Zm9v"}"#);
}

#[test]
fn attribute_value_with_number_contains_only_number() {
    let all_default = AttributeValue{
        n: Some(1234.to_string()),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&all_default).unwrap();
    assert_eq!(&serialized, r#"{"N":"1234"}"#);
}

#[test]
fn attribute_value_with_binary_set() {
    let all_default = AttributeValue{
        bs: Some(vec![
            "foo".bytes().collect(),
            "bar".bytes().collect(),
            "baz".bytes().collect()
        ]),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&all_default).unwrap();
    assert_eq!(&serialized, r#"{"BS":["Zm9v","YmFy","YmF6"]}"#);
}
