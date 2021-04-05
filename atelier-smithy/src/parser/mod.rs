// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! unexpected {
    ($fn_name:expr, $pair:expr) => {{
        error!("ParserError::unexpected({}, {:?})", $fn_name, $pair);
        return ParserError::unexpected($fn_name, &$pair).into();
    }};
}

macro_rules! expecting {
    ($fn_name:expr, $rule:expr) => {{
        error!("ParserError::new({}, {:?})", $fn_name, $rule);
        return ParserError::new($fn_name)
            .expecting(stringify!($rule.to_string()))
            .into();
    }};
}

macro_rules! entry {
    ($fn_name:expr, $pair:expr) => {
        debug!("{}({:?})", $fn_name, &$pair.as_rule());
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod error;

mod smithy;
pub use smithy::parse_model;

#[cfg(feature = "debug")]
pub use smithy::parse_and_debug_model;

mod selector;
pub use selector::parse_selector;
