use atelier_core::error::{Error as CoreError, ErrorKind, Result as CoreResult};
use pest::iterators::Pair;
use pest::RuleType;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ParserError {
    fn_name: String,
    rule: Option<String>,
    expecting: Option<String>,
    unreachable: bool,
    context: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ParserError> for CoreError {
    fn from(e: ParserError) -> Self {
        ErrorKind::Deserialization(
            "Smithy".to_string(),
            format!(
                "{}{}{}{}",
                e.fn_name,
                match e.rule {
                    None => String::new(),
                    Some(s) => format!(" ({})", s),
                },
                match e.expecting {
                    None => String::new(),
                    Some(s) => format!(" expecting {}", s),
                },
                if e.unreachable {
                    " should have been unreachable".to_string()
                } else {
                    String::new()
                },
            ),
            e.context,
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
    pub(crate) fn new(fn_name: &str) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: false,
            context: None,
        }
    }

    pub(crate) fn unreachable(fn_name: &str) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: true,
            context: None,
        }
    }

    pub(crate) fn unexpected<T: RuleType>(fn_name: &str, pair: &Pair<'_, T>) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: true,
            context: Some(format!("{:?}: {:?}", pair.as_rule(), pair.as_str())),
        }
    }

    pub(crate) fn in_rule(&mut self, rule: &str) -> &mut Self {
        self.rule = Some(rule.to_string());
        self
    }

    pub(crate) fn expecting(&mut self, expecting: &str) -> &mut Self {
        self.expecting = Some(expecting.to_string());
        self
    }

    pub(crate) fn unreachable_rule(&mut self) -> &mut Self {
        self.unreachable = true;
        self
    }

    pub(crate) fn context(&mut self, context: &dyn Display) -> &mut Self {
        self.context = Some(format!("{}", context));
        self
    }

    pub(crate) fn debug_context(&mut self, context: &dyn Debug) -> &mut Self {
        self.context = Some(format!("{:?}", context));
        self
    }
}
