/// Route53 TXT entries require quotations around their values.
/// Use this function to add quotes if needed. These examples show how
/// this adds quotes if needed:
/// ```ignore
/// let rr = vec![
///   ResourceRecord {
///     value: quote_txt_record("foo"),
///   },
///   ResourceRecord {
///     value: quote_txt_record("\"baz\""),
///   },
/// ];
/// ```
pub fn quote_txt_record(record_contents: &str) -> String {
    let mut quoted_string = String::from(record_contents);
    if !quoted_string.starts_with("\"") {
        quoted_string = format!("\"{}", quoted_string);
    }
    if !quoted_string.ends_with("\"") {
        quoted_string = format!("{}\"", quoted_string);
    }
    quoted_string
}
