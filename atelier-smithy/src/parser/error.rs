use super::Rule;
use atelier_core::error::{Error as CoreError, ErrorKind, Result as CoreResult};
use pest::iterators::Pair;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(super) struct ParserError {
    fn_name: String,
    rule: Option<String>,
    expecting: Option<String>,
    unreachable: bool,
    context: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Into<CoreError> for ParserError {
    fn into(self) -> CoreError {
        ErrorKind::Deserialization(
            "Smithy".to_string(),
            format!(
                "{}{}{}{}",
                self.fn_name,
                match self.rule {
                    None => String::new(),
                    Some(s) => format!(" ({})", s),
                },
                match self.expecting {
                    None => String::new(),
                    Some(s) => format!(" expecting {}", s),
                },
                if self.unreachable {
                    " should have been unreachable".to_string()
                } else {
                    String::new()
                },
            ),
            self.context,
        )
        .into()
    }
}

impl<T> From<ParserError> for CoreResult<T> {
    fn from(e: ParserError) -> Self {
        Err(e.into())
    }
}

impl<T> From<&mut ParserError> for CoreResult<T> {
    fn from(e: &mut ParserError) -> Self {
        Err(e.clone().into())
    }
}

#[allow(dead_code)]
impl ParserError {
    pub(super) fn new(fn_name: &str) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: false,
            context: None,
        }
    }

    pub(super) fn unreachable(fn_name: &str) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: true,
            context: None,
        }
    }

    pub(super) fn unexpected(fn_name: &str, pair: &Pair<'_, Rule>) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: true,
            context: Some(format!("{:?}", pair)),
        }
    }

    pub(super) fn in_rule(&mut self, rule: &str) -> &mut Self {
        self.rule = Some(rule.to_string());
        self
    }

    pub(super) fn expecting(&mut self, expecting: &str) -> &mut Self {
        self.expecting = Some(expecting.to_string());
        self
    }

    pub(super) fn unreachable_rule(&mut self) -> &mut Self {
        self.unreachable = true;
        self
    }

    pub(super) fn context(&mut self, context: &dyn Display) -> &mut Self {
        self.context = Some(format!("{}", context));
        self
    }

    pub(super) fn debug_context(&mut self, context: &dyn Debug) -> &mut Self {
        self.context = Some(format!("{:?}", context));
        self
    }
}
