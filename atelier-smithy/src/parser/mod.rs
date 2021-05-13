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
        trace!("{}({:?})", $fn_name, &$pair.as_rule());
    };
}

macro_rules! pair_as_str {
    ($fn_name:expr, $item:expr, $rule:expr) => {
        if $item.as_rule() == $rule {
            $item.as_str().to_string()
        } else {
            unexpected!($fn_name, $item)
        }
    };
}

macro_rules! next_pair_as_str {
    ($fn_name:expr, $outer:expr, $rule:expr) => {{
        let item = $outer.next().unwrap();
        pair_as_str!($fn_name, item, $rule)
    }};
}

macro_rules! pair_into {
    ($fn_name:expr, $item:expr, $rule:expr, $into_fn:ident) => {
        if $item.as_rule() == $rule {
            $into_fn($item)?
        } else {
            unexpected!($fn_name, $item)
        }
    };
    ($fn_name:expr, $item:expr, $rule:expr, $into_fn:expr) => {
        if $item.as_rule() == $rule {
            $into_fn($item)?
        } else {
            unexpected!($fn_name, $item)
        }
    };
}

macro_rules! next_pair_into {
    ($fn_name:expr, $outer:expr, $rule:expr, $into_fn:ident) => {{
        let item = $outer.next().unwrap();
        pair_into!($fn_name, item, $rule, $into_fn)
    }};
    ($fn_name:expr, $outer:expr, $rule:expr, $into_fn:expr) => {{
        let item = $outer.next().unwrap();
        pair_into!($fn_name, item, $rule, $into_fn)
    }};
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
