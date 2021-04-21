use std::collections::{HashMap, HashSet};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref UNWELCOME: HashSet<&'static str> =
        ["blacklist", "kill", "master", "slave", "whitelist"]
            .iter()
            .copied()
            .collect();
    static ref SYNONYMS: HashMap<&'static str, HashSet<&'static str>> = [
        (
            "noun:list",
            ["list", "bag", "queue"].iter().copied().collect()
        ),
        (
            "noun:map",
            ["map", "dictionary", "hash"].iter().copied().collect()
        ),
        (
            "noun:structure",
            ["structure", "document", "message", "record", "table"]
                .iter()
                .copied()
                .collect()
        ),
        ("noun:service", ["service", "svc"].iter().copied().collect()),
        (
            "noun:operation",
            ["operation", "function", "lambda", "procedure"]
                .iter()
                .copied()
                .collect()
        ),
        ("noun:input", ["input", "request"].iter().copied().collect()),
        (
            "noun:output",
            ["output", "response"].iter().copied().collect()
        ),
        (
            "noun:error",
            ["error", "exception", "failure"].iter().copied().collect()
        ),
        ("verb:create", ["create", "post"].iter().copied().collect()),
        ("verb:put", ["put", "push", "set"].iter().copied().collect()),
        (
            "verb:read",
            ["read", "fetch", "get", "retrieve"]
                .iter()
                .copied()
                .collect()
        ),
        ("verb:update", ["update", "post"].iter().copied().collect()),
        (
            "verb:delete",
            ["delete", "remove"].iter().copied().collect()
        ),
        ("verb:list", ["list", "post"].iter().copied().collect()),
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
