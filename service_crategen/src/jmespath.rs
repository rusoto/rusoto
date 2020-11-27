use lazy_static::lazy_static;
/// a limited implementation of JMESPath (https://jmespath.org/)
/// this only supports the tiny subset of features used as part of the
/// pagination definitions
///
/// foo
/// foo.bar
/// foo[-1]
/// foo || bar
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JMESTerm {
    Key(String),
    Last,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JMESPath {
    Path(Vec<JMESTerm>),
    Or(Vec<JMESPath>),
}

impl JMESPath {
    pub fn parse(path: &str) -> JMESPath {
        lazy_static! {
            static ref SANITIZE: Regex = Regex::new(r"[^a-z0-9A-Z\. |\[\]-]").unwrap();
        }
        lazy_static! {
            static ref NAME: Regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9]*").unwrap();
        }
        assert!(
            !SANITIZE.is_match(path),
            "JMESPath '{}' uses unsupported features",
            path
        );
        if path.contains("||") {
            let ors = path.split("||").into_iter().map(JMESPath::parse).collect();
            JMESPath::Or(ors)
        } else {
            let mut bits = vec![];
            for bit in path.split('.') {
                let trimmed = bit.trim();
                if let Some(name) = trimmed.strip_suffix("[-1]") {
                    assert!(
                        NAME.is_match(name),
                        "invalid name '{}' in path '{}'",
                        name,
                        path
                    );
                    bits.push(JMESTerm::Key(name.to_string()));
                    bits.push(JMESTerm::Last);
                } else {
                    assert!(
                        NAME.is_match(trimmed),
                        "invalid name '{}' in path '{}'",
                        trimmed,
                        path
                    );
                    bits.push(JMESTerm::Key(trimmed.to_string()));
                }
            }
            JMESPath::Path(bits)
        }
    }
}

#[cfg(test)]
mod tests {
    use self::JMESPath::*;
    use self::JMESTerm::*;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(JMESPath::parse("a"), Path(vec![Key("a".to_string())]));
        assert_eq!(
            JMESPath::parse("a.b"),
            Path(vec![Key("a".to_string()), Key("b".to_string())])
        );
        assert_eq!(
            JMESPath::parse("a[-1]"),
            Path(vec![Key("a".to_string()), Last])
        );
        assert_eq!(
            JMESPath::parse("a || b"),
            Or(vec![
                Path(vec![Key("a".to_string())]),
                Path(vec![Key("b".to_string())])
            ])
        );
        assert_eq!(
            JMESPath::parse("NextMarker || Contents[-1].Key"),
            Or(vec![
                Path(vec![Key("NextMarker".to_string())]),
                Path(vec![
                    Key("Contents".to_string()),
                    Last,
                    Key("Key".to_string())
                ])
            ])
        );
    }
}
