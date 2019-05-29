use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use hoedown::renderer::html::{Flags, Html};
use hoedown::{Markdown, Render};

pub struct Item<T>(pub T);

impl<T: AsRef<str>> Display for Item<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        preprocess(self.0.as_ref(), "///", fmt)
    }
}

pub struct Module<T>(pub T);

impl<T: AsRef<str>> Display for Module<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        preprocess(self.0.as_ref(), "//!", fmt)
    }
}

fn preprocess(input: &str, pre: &str, fmt: &mut Formatter<'_>) -> FmtResult {
    // fix up problems in AWS docs
    let escaped_1 = input.replace("<code>\\</code>", "<code>&bsol;</code>");
    let escaped_2 = escaped_1.replace("\"</p>\"", "</p>");

    // parse and render markdown
    let markdown = Markdown::new(&escaped_2);
    let mut html_renderer = Html::new(Flags::empty(), 0);
    let buffer = html_renderer.render(&markdown);
    let rendered = buffer.to_str().unwrap();

    // prefix and write to formatter
    prefix(&rendered, pre, fmt)
}

fn prefix(input: &str, pre: &str, fmt: &mut Formatter<'_>) -> FmtResult {
    for (i, line) in input.lines().enumerate() {
        if i > 0 {
            fmt.write_char('\n')?;
        }
        if line.is_empty() {
            write!(fmt, "{}", pre)?;
        } else {
            write!(fmt, "{} {}", pre, line.trim())?;
        }
    }
    Ok(())
}
