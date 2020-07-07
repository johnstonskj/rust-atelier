/*!
This module contains core `Linter` implementations. It also provides a function,
`run_linter_actions`, that takes a list of linters to run against a model. This is the
preferred way to run the linter actions as it allows for _fast fail_ on detecting errors
in an action.

# Example

The example model below will result in a number of errors:

1. The shape named "shouldBeUpper" should be "**S**houldBeUpper",
1. The member "BadName" in structure "MyStructure", should be "**b**adName".
1. The trait "BadTraitName" on structure "MyStructure", should be "**b**adTraitName".
1. The shape "ThingAsJSON", reference by "MyStructure#thing", includes a known acronym "JSON".

```rust
use atelier_core::action::lint::{run_linter_actions, NamingConventions};
use atelier_core::action::Linter;
use atelier_core::model::builder::{
    ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder
};
use atelier_core::model::Model;
use atelier_core::Version;

let model: Model = ModelBuilder::new("smithy.example", Some(Version::V10))
    .shape(SimpleShapeBuilder::string("shouldBeUpper").into())
    .shape(
        StructureBuilder::new("MyStructure")
            .member("okName", "String")
            .member("BadName", "MyString")
            .member("thing", "ThingAsJSON")
            .add_trait(TraitBuilder::new("BadTraitName").into())
             .into(),
    )
    .into();
let result = run_linter_actions(&[
        Box::new(NamingConventions::default()),
    ], &model, false);
```

*/

use crate::action::{Action, ActionIssue, IssueLevel, Linter};
use crate::model::shapes::{ShapeBody, Trait, Valued};
use crate::model::{Annotated, Identifier, Model, Named, ShapeID};
use heck::{CamelCase, MixedCase};
use std::cell::RefCell;
use std::collections::HashSet;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This will report any violations of the naming conventions described in the Smithy [style
/// guide](https://awslabs.github.io/smithy/1.0/guides/style-guide.html?highlight=naming#naming).
///
/// * `Shape` names should be in UpperCamelCase.
/// * `Member` names and `Trait` names should be in lowerCamelCase.
///
#[derive(Debug)]
pub struct NamingConventions {}

///
/// This will report any use of any unwelcome, or problematic terms in names.
///
#[derive(Debug)]
pub struct UnwelcomeTerms {
    checked: RefCell<HashSet<Identifier>>,
    issues: RefCell<Vec<ActionIssue>>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Run each provided `Linter`, in order, against the provided `Model`. All issues will be collated
/// and returned together.
///
/// The `fail_fast` flag determines the behavior if a linter action returns an error. If `true` the
/// process stops and returns all reported issues up to that point, if `false` it continues on.
///
pub fn run_linter_actions(
    linters: &[Box<dyn Linter>],
    model: &Model,
    fail_fast: bool,
) -> Option<Vec<ActionIssue>> {
    let mut issues: Vec<ActionIssue> = Default::default();

    for linter in linters {
        if let Some(mut new_issues) = linter.check(model) {
            issues.append(&mut new_issues);
            if fail_fast
                && new_issues
                    .iter()
                    .any(|issue| issue.level > IssueLevel::Warning)
            {
                return Some(issues);
            }
        }
    }

    if issues.is_empty() {
        None
    } else {
        Some(issues)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NamingConventions {
    fn default() -> Self {
        Self {}
    }
}

impl Action for NamingConventions {
    fn label(&self) -> &'static str {
        "NamingConventions"
    }
}

impl Linter for NamingConventions {
    fn check(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        let mut issues: Vec<ActionIssue> = Default::default();
        for shape in model.shapes() {
            if let Some(shape) = model.shape(shape.id()) {
                if shape.has_trait(&ShapeID::from_str("trait").unwrap()) {
                    self.check_trait_name(shape.id(), &mut issues);
                } else {
                    self.check_shape_name(shape.id(), &mut issues);
                }
            } else {
                self.check_shape_name(shape.id(), &mut issues);
            }
            self.check_applied_trait_names(shape.traits(), &mut issues);

            match shape.body() {
                // TODO: term used in names
                // simple shape, list/set/map/structure/union should not contain other's terms
                // operation usage should check any prefix for the correct verb
                // operation input/output should check suffix
                //
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        self.check_member_name(shape.id(), member.id(), &mut issues);
                        self.check_shape_name(
                            member.value().as_ref().unwrap().as_shape_id().unwrap(),
                            &mut issues,
                        );
                        self.check_applied_trait_names(member.traits(), &mut issues);
                    }
                }
                _ => {}
            }
        }
        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }
}

impl NamingConventions {
    fn check_shape_name(&self, id: &ShapeID, issues: &mut Vec<ActionIssue>) {
        let shape_name = id.shape_name().to_string();
        if shape_name.to_camel_case() != shape_name {
            issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "Shape names should conform to UpperCamelCase, i.e. {}",
                    shape_name.to_camel_case()
                ),
                id.clone(),
            ));
        }
    }
    fn check_trait_name(&self, id: &ShapeID, issues: &mut Vec<ActionIssue>) {
        let shape_name = id.shape_name().to_string();
        if shape_name.to_mixed_case() != shape_name {
            issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "Trait names should conform to lowerCamelCase, i.e. {}",
                    shape_name.to_mixed_case()
                ),
                id.clone(),
            ));
        }
    }
    fn check_member_name(
        &self,
        shape_id: &ShapeID,
        member_id: &Identifier,
        issues: &mut Vec<ActionIssue>,
    ) {
        let member_name = member_id.to_string();
        if member_name.to_mixed_case() != member_name {
            issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "Member names should conform to lowerCamelCase, i.e. {}",
                    member_name.to_mixed_case()
                ),
                shape_id.to_member(member_id.clone()),
            ));
        }
    }
    fn check_applied_trait_names(&self, traits: &[Trait], issues: &mut Vec<ActionIssue>) {
        for a_trait in traits {
            self.check_trait_name(a_trait.id(), issues);
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for UnwelcomeTerms {
    fn default() -> Self {
        Self {
            checked: RefCell::new(Default::default()),
            issues: RefCell::new(Default::default()),
        }
    }
}

impl Action for UnwelcomeTerms {
    fn label(&self) -> &'static str {
        "UnwelcomeTerms"
    }
}

impl Linter for UnwelcomeTerms {
    fn check(&self, model: &Model) -> Option<Vec<ActionIssue>> {
        for shape in model.shapes() {
            self.check_shape_id(shape.id());
            for a_trait in shape.traits() {
                self.check_shape_id(a_trait.id());
            }

            match shape.body() {
                ShapeBody::Structure(body) | ShapeBody::Union(body) => {
                    for member in body.members() {
                        self.check_identifier(member.id(), Some(shape.id()));
                        self.check_shape_id(
                            member.value().as_ref().unwrap().as_shape_id().unwrap(),
                        );
                        for a_trait in member.traits() {
                            self.check_shape_id(a_trait.id());
                        }
                    }
                }
                _ => {}
            }
        }
        if self.issues.borrow().is_empty() {
            None
        } else {
            Some(self.issues.borrow().clone())
        }
    }
}

impl UnwelcomeTerms {
    fn check_shape_id(&self, shape_id: &ShapeID) {
        if let Some(namespace) = shape_id.namespace() {
            for id in namespace.split() {
                self.check_identifier(&id, Some(shape_id));
            }
        }
        self.check_identifier(&shape_id.shape_name(), Some(shape_id));
        if let Some(member_name) = shape_id.member_name() {
            self.check_identifier(member_name, Some(shape_id));
        }
    }

    #[inline]
    fn check_identifier(&self, id: &Identifier, in_shape: Option<&ShapeID>) {
        if !self.checked.borrow().contains(id) {
            let _ = self.checked.borrow_mut().insert(id.clone());
            for word in terms::split_words(&id.to_string()) {
                if terms::is_unwelcome_term(&word) {
                    self.issues.borrow_mut().push(match in_shape {
                        None => ActionIssue::warning(
                            &self.label(),
                            &format!(
                                "The term '{}' is considered either insensitive, divisive, or otherwise unwelcome",
                                word
                            )),
                        Some(in_shape) => ActionIssue::warning_at(
                            &self.label(),
                            &format!(
                                "The term '{}' is considered either insensitive, divisive, or otherwise unwelcome",
                                word
                            ), in_shape.clone())
                    })
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod terms;
