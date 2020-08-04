use std::fmt;
use crate::common::span::Span;

/// Represents a static error (syntax, semantics, etc.) found at compile time
#[derive(PartialEq, Eq)]
pub struct Syntax {
    message: String,
    span:    Span,
}

impl Syntax {
    pub fn error(message: &str, span: Span) -> Syntax {
        Syntax { message: message.to_string(), span }
    }
}

impl fmt::Display for Syntax {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.span, f)?;
        writeln!(f, "Encountered a Static Error: {}", self.message)
    }
}

impl fmt::Debug for Syntax {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::source::Source;
    use std::rc::Rc;

    #[test]
    fn error() {
        // This is just a demo to check formatting
        // might not coincide with an actual Passerine error
        let source = Rc::new(Source::source("x = \"Hello, world\" -> y + 1"));
        let error = Syntax::error(
            "Unexpected token '\"Hello, world!\"'",
            Span::new(&source, 4, 14),
        );

        let target = "Line 1:5
  |
1 | x = \"Hello, world\" -> y + 1
  |     ^^^^^^^^^^^^^^
Encountered a Static Error: Unexpected token '\"Hello, world!\"'
";

        let result = format!("{}", error);
        assert_eq!(result, target);
    }
}