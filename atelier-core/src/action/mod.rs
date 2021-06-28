/*!
This module provides a set of traits that describes actions that can operate on models. These
actions take three major forms; linters, validators, and transformers.

1. **Linters**; these inspect the model for stylistic issues, they are a subset of validators.
1. **Validators**; these inspect models for errors and warnings that may produce errors when the
   model is used.
1. **Transformers**; these take in a model and transform it into another model.

# Example

The following example is taken from the Smithy specification discussing
[relative name resolution](https://awslabs.github.io/smithy/1.0/spec/core/shapes.html#relative-shape-id-resolution).
The `run_validation_actions` function is commonly used to take a list of actions to be performed
on the model in sequence.

```rust
use atelier_core::action::validate::{
    run_validation_actions, CorrectTypeReferences
};
use atelier_core::action::Validator;
use atelier_core::builder::{
    ModelBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder
};
use atelier_core::model::Model;
use atelier_core::Version;
use std::convert::TryInto;

let model: Model = ModelBuilder::new(Version::V10, "smithy.example")
    .uses("foo.baz#Bar")
    .structure(
        StructureBuilder::new("MyStructure")
            .member("a", "MyString")
            .member("b", "smithy.example#MyString")
            .member("d", "foo.baz#Bar")
            .member("f", "String")
            .member("g", "MyBoolean")
            .apply_trait(TraitBuilder::new("documentation"))
            .into(),
    )
    .simple_shape(SimpleShapeBuilder::string("MyString"))
    .simple_shape(SimpleShapeBuilder::boolean("MyBoolean"))
    .try_into().unwrap();
let result = run_validation_actions(&mut [
        Box::new(CorrectTypeReferences::default()),
    ], &model, false);
```

This will result in the following list of validation errors. Note that the error is denoted against
shape or member identifier accordingly.

```text
[
    ActionIssue {
        reporter: "CorrectTypeReferences",
        level: Info,
        message: "The simple shape (smithy.example#MyBoolean) is simply a synonym, did you mean to add any constraint traits?",
        locus: Some(
            ShapeID {
                namespace: NamespaceID(
                    "smithy.example",
                ),
                shape_name: Identifier(
                    "MyBoolean",
                ),
                member_name: None,
            },
        ),
    },
    ActionIssue {
        reporter: "CorrectTypeReferences",
        level: Info,
        message: "The simple shape (smithy.example#MyString) is simply a synonym, did you mean to add any constraint traits?",
        locus: Some(
            ShapeID {
                namespace: NamespaceID(
                    "smithy.example",
                ),
                shape_name: Identifier(
                    "MyString",
                ),
                member_name: None,
            },
        ),
    },
]
```

*/

use crate::error::Result as ModelResult;
use crate::model::{Model, ShapeID};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Denotes the level associated with an issue reported by an action.
///
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum IssueLevel {
    /// Informational, linters _should only_ report informational issues.
    Info,
    /// Warnings which represent issues that may cause the model to produce erroneous results.
    Warning,
    /// Errors in the model, it cannot be used as-is.
    Error,
}

///
/// An issue reported by an action. An issue may, or may not, be associated with a shape but will
/// always include a message.
///
#[derive(Debug, Clone)]
pub struct ActionIssue {
    reporter: String,
    level: IssueLevel,
    message: String,
    locus: Option<ShapeID>,
}

///
/// A trait required by `Linter`, `Validator`, and `Transformer`.
///
pub trait Action {
    ///
    /// This is a display label to use to determine the validator that causes an error.
    ///
    fn label(&self) -> &'static str;

    ///
    /// Return all the issues reported by this action, note that the set of issues aggregates across
    /// multiple uses of the implementation, so for a lint tool the same `Linter::check` can be
    /// for different models and the issues are the sum of all found in all models.
    ///
    fn issues(&self) -> &Vec<ActionIssue>;

    ///
    /// Return all the issues reported by this action in a mutable collection.
    ///
    fn issues_mut(&mut self) -> &mut Vec<ActionIssue>;
}

///
/// Check the model for stylistic or other conventions that the author should be aware of. An
/// error represents a failure in the linter itself, not the presence of any issues which
/// should be fetched using `Action::issues` or `Action::issues_mut`.
///
pub trait Linter: Action {
    ///
    /// Check the model adding any issues found to the `Action:issues` collection.
    ///
    fn check(&mut self, model: &Model) -> ModelResult<()>;
}

///
/// Validate the model according to rules that determine whether it is complete and usable.. An
/// error represents a failure in the validator itself, not the presence of any issues which
/// should be fetched using `Action::issues` or `Action::issues_mut`.
///
pub trait Validator: Action {
    ///
    /// Validate the model adding any issues found to the `Action:issues` collection.
    ///
    fn validate(&mut self, model: &Model) -> ModelResult<()>;
}

///
/// Create a new model from an existing one; this might be a filter, a decorator, or generator. An
/// error represents a failure in the transformer itself, not the presence of any issues which
/// should be fetched using `Action::issues` or `Action::issues_mut`.
///
pub trait Transformer: Action {
    ///
    /// Transform the input model into another. This _may_ consume the input and produce an entirely
    /// new model, or it _may_ simply mutate the model and return the modified input.
    ///
    fn transform(&mut self, model: Model) -> ModelResult<Model>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for IssueLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IssueLevel::Info => "info",
                IssueLevel::Warning => "warning",
                IssueLevel::Error => "error",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ActionIssue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{}: {}",
            self.reporter(),
            self.level(),
            match self.locus() {
                Some(id) => format!(" {}", id),
                None => String::new(),
            },
            self.message()
        )
    }
}

impl std::error::Error for ActionIssue {}

impl ActionIssue {
    /// Create a new report with the provided level and message.
    pub fn new(level: IssueLevel, reporter: &str, message: &str) -> Self {
        assert!(!message.is_empty());
        Self {
            reporter: reporter.to_string(),
            level,
            message: message.to_string(),
            locus: None,
        }
    }

    /// Create a new report with the provided level and message and denote the given `ShapeID` as
    /// the locus of the issue.
    pub fn new_at(level: IssueLevel, reporter: &str, message: &str, locus: ShapeID) -> Self {
        assert!(!message.is_empty());
        Self {
            reporter: reporter.to_string(),
            level,
            message: message.to_string(),
            locus: Some(locus),
        }
    }

    /// Create a new informational report with the provided message.
    pub fn info(reporter: &str, message: &str) -> Self {
        Self::new(IssueLevel::Info, reporter, message)
    }

    /// Create a new informational report with the provided message and denote the given `ShapeID` as
    /// the locus of the issue.
    pub fn info_at(reporter: &str, message: &str, locus: ShapeID) -> Self {
        Self::new_at(IssueLevel::Info, reporter, message, locus)
    }

    /// Create a new warning report with the provided message.
    pub fn warning(reporter: &str, message: &str) -> Self {
        Self::new(IssueLevel::Warning, reporter, message)
    }

    /// Create a new warning report with the provided message and denote the given `ShapeID` as
    /// the locus of the issue.
    pub fn warning_at(reporter: &str, message: &str, locus: ShapeID) -> Self {
        Self::new_at(IssueLevel::Warning, reporter, message, locus)
    }

    /// Create a new error report with the provided message.
    pub fn error(reporter: &str, message: &str) -> Self {
        Self::new(IssueLevel::Error, reporter, message)
    }

    /// Create a new error report with the provided message and denote the given `ShapeID` as
    /// the locus of the issue.
    pub fn error_at(reporter: &str, message: &str, locus: ShapeID) -> Self {
        Self::new_at(IssueLevel::Error, reporter, message, locus)
    }

    /// Return the action that reported this issue.
    pub fn reporter(&self) -> &String {
        &self.reporter
    }

    /// Return the level associated with this issue.
    pub fn level(&self) -> &IssueLevel {
        &self.level
    }

    /// Return the message associated with this issue.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Return the locus of the error, if one is recorded.
    pub fn locus(&self) -> &Option<ShapeID> {
        &self.locus
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod lint;

#[doc(hidden)]
pub mod transform;

pub mod validate;
