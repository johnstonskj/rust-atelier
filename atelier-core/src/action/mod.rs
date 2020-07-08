/*!
This module describes actions that can operate on models. These take three major forms:

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
    NoOrphanedReferences, run_validation_actions, CorrectTypeReferences
};
use atelier_core::action::Validator;
use atelier_core::model::builder::{ModelBuilder, SimpleShapeBuilder, StructureBuilder};
use atelier_core::model::Model;
use atelier_core::Version;

let model: Model = ModelBuilder::new("smithy.example", Some(Version::V10))
    .uses("foo.baz#Bar")
    .shape(SimpleShapeBuilder::string("MyString").into())
    .shape(
        StructureBuilder::new("MyStructure")
            .member("a", "MyString")
            .member("b", "smithy.example#MyString")
            .member("c", "Bar")
            .member("d", "foo.baz#Bar")
            .member("e", "foo.baz#MyString")
            .member("f", "String")
            .member("g", "MyBoolean")
            .member("h", "InvalidShape")
            .into(),
    )
    .shape(SimpleShapeBuilder::boolean("MyBoolean").into())
    .into();
let result = run_validation_actions(&[
        Box::new(NoOrphanedReferences::default()),
        Box::new(CorrectTypeReferences::default()),
    ], &model, false);
```

This will result in the following list of validation errors. Note that the error is denoted against
shape or member identifier accordingly.

```text
[
    ActionIssue {
        reporter: "NoOrphanedReferences",
        level: Error,
        message: "Shape, or member, has a trait that refers to an unknown identifier: notKnown",
        locus: Some(
            ShapeID {
                namespace: None,
                shape_name: Identifier(
                    "MyStructure",
                ),
                member_name: None,
            },
        ),
    },
    ActionIssue {
        reporter: "NoOrphanedReferences",
        level: Error,
        message: "Shape, or member, refers to an unknown identifier: foo.baz#MyString",
        locus: Some(
            ShapeID {
                namespace: None,
                shape_name: Identifier(
                    "MyStructure",
                ),
                member_name: None,
            },
        ),
    },
    ActionIssue {
        reporter: "NoOrphanedReferences",
        level: Error,
        message: "Shape, or member, refers to an unknown identifier: InvalidShape",
        locus: Some(
            ShapeID {
                namespace: None,
                shape_name: Identifier(
                    "MyStructure",
                ),
                member_name: None,
            },
        ),
    },
]
```

*/

use crate::error::Result;
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
/// A trait implemented by tools that provide validation over a model.
///
pub trait Action {
    ///
    /// This is a display label to use to determine the validator that causes an error.
    ///
    fn label(&self) -> &'static str;
}

///
/// A trait implemented by tools that provide validation over a model.
///
pub trait Linter: Action {
    ///
    /// Validate the model returning any issue, or issues, it may contain.
    ///
    fn check(&self, model: &Model) -> Option<Vec<ActionIssue>>;
}

///
/// A trait implemented by tools that provide validation over a model.
///
pub trait Validator: Action {
    ///
    /// Validate the model returning any issue, or issues, it may contain.
    ///
    fn validate(&self, model: &Model) -> Option<Vec<ActionIssue>>;
}

///
/// A trait implemented by tools that transform one model into another.
///
pub trait Transformer: Action {
    ///
    /// Transform the input model into another. This _may_ consume the input and produce an entirely
    /// new model, or it _may_ simply mutate the model and return the modified input.
    ///
    fn transform(&self, model: Model) -> Result<Model>;
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

pub mod transform;

pub mod validate;
