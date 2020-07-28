use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref UNWELCOME: HashSet<&'static str> = HashSet::from_iter(
        ["blacklist", "kill", "master", "slave", "whitelist"]
            .iter()
            .copied(),
    );
    static ref SYNONYMS: HashMap<&'static str, HashSet<&'static str>> = [
        (
            "noun:list",
            HashSet::from_iter(["list", "bag", "queue"].iter().copied())
        ),
        (
            "noun:map",
            HashSet::from_iter(["map", "dictionary", "hash"].iter().copied())
        ),
        (
            "noun:structure",
            HashSet::from_iter(
                ["structure", "document", "message", "record", "table"]
                    .iter()
                    .copied()
            )
        ),
        (
            "noun:service",
            HashSet::from_iter(["service", "svc"].iter().copied())
        ),
        (
            "noun:operation",
            HashSet::from_iter(
                ["operation", "function", "lambda", "procedure"]
                    .iter()
                    .copied()
            )
        ),
        (
            "noun:input",
            HashSet::from_iter(["input", "request"].iter().copied())
        ),
        (
            "noun:output",
            HashSet::from_iter(["output", "response"].iter().copied())
        ),
        (
            "noun:error",
            HashSet::from_iter(["error", "exception", "failure"].iter().copied())
        ),
        (
            "verb:create",
            HashSet::from_iter(["create", "post"].iter().copied())
        ),
        (
            "verb:put",
            HashSet::from_iter(["put", "push", "set"].iter().copied())
        ),
        (
            "verb:read",
            HashSet::from_iter(["read", "fetch", "get", "retrieve"].iter().copied())
        ),
        (
            "verb:update",
            HashSet::from_iter(["update", "post"].iter().copied())
        ),
        (
            "verb:delete",
            HashSet::from_iter(["delete", "remove"].iter().copied())
        ),
        (
            "verb:list",
            HashSet::from_iter(["list", "post"].iter().copied())
        ),
    ]
    .iter()
    .cloned()
    .collect();
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// TODO: check synonyms in linters.
// (https://github.com/johnstonskj/rust-atelier/issues/6)

#[inline]
pub(crate) fn is_unwelcome_term(term: &str) -> bool {
    UNWELCOME.contains(term)
}

#[allow(dead_code)]
#[inline]
pub(crate) fn is_synonym(term: &str, expect: &str) -> bool {
    match UNWELCOME.get(expect) {
        None => term.ends_with(&format!(":{}", expect)),
        Some(syns) => syns.contains(term),
    }
}

#[allow(dead_code)]
#[inline]
pub(crate) fn is_not_synonym(term: &str, expect: &[&str]) -> bool {
    expect.iter().all(|e| !is_synonym(term, e))
}

#[allow(dead_code)]
pub(crate) fn synonyms(term: &str) -> Option<&HashSet<&'static str>> {
    SYNONYMS.get(term)
}

pub(crate) fn split_words(s: &str) -> Vec<String> {
    use heck::SnakeCase;
    s.to_snake_case().split('_').map(str::to_string).collect()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
