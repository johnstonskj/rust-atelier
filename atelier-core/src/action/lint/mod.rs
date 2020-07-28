/*!
* This module contains core `Linter` implementations. It also provides a function,
* `run_linter_actions`, that takes a list of linters to run against a model. This is the
* preferred way to run the linter actions as it allows for _fast fail_ on detecting errors
* in an action.
*
* # Example
*
* The example model below will result in a number of errors:
*
* 1. The shape named "shouldBeUpper" should be "**S**houldBeUpper",
* 1. The member "BadName" in structure "MyStructure", should be "**b**adName".
* 1. The trait "BadTraitName" on structure "MyStructure", should be "**b**adTraitName".
* 1. The shape "ThingAsJSON", reference by "MyStructure#thing", includes a known acronym "JSON".
*
* ```rust
* use atelier_core::action::lint::{run_linter_actions, NamingConventions};
* use atelier_core::action::Linter;
* use atelier_core::builder::{
*     ListBuilder, ModelBuilder, ShapeTraits, SimpleShapeBuilder, StructureBuilder, TraitBuilder
* };
* use atelier_core::model::{NamespaceID, Model};
* use atelier_core::Version;
*
* let model: Model = ModelBuilder::new(Version::V10, "smithy.example")
*     .uses("amazon.fashion#BadTraitName")
*     .simple_shape(SimpleShapeBuilder::string("smithy.example#shouldBeUpper"))
*     .simple_shape(SimpleShapeBuilder::string("MyString"))
*     .simple_shape(SimpleShapeBuilder::string("ThingAsJSON"))
*     .list(ListBuilder::new("TheBlacklist", "String"))
*     .structure(
*         StructureBuilder::new("MyStructure")
*             .member("okName", "String")
*             .member("BadName", "MyString")
*             .member("thing", "ThingAsJSON")
*             .member("checkAgainst", "TheBlacklist")
*             .member("killMasterNode", "Boolean")
*             .apply_trait(TraitBuilder::new("amazon.fashion#BadTraitName"))
*             .into(),
*     )
*     .into();
* let result = run_linter_actions(&mut [
*         Box::new(NamingConventions::default()),
*     ], &model, false);
* ```
*
*/

use crate::action::{Action, ActionIssue, IssueLevel, Linter};
use crate::error::Result as ModelResult;
use crate::model::shapes::{AppliedTrait, Shape, ShapeKind};
use crate::model::{Identifier, Model, ShapeID};
use heck::{CamelCase, MixedCase};
use std::cell::RefCell;
use std::collections::HashSet;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

linter_or_validator_defn! {
NamingConventions,
r#"This will report any violations of the naming
conventions described in the Smithy [style guide](https://awslabs.github.io/smithy/1.0/guides/style-guide.html?highlight=naming#naming).

 * `Shape` names should be in UpperCamelCase.
 * `Member` names and `Trait` names should be in lowerCamelCase."# }

linter_or_validator_defn! {
    UnwelcomeTerms
    {
        checked: RefCell<HashSet<Identifier>>
    },
    "This will report any use of any unwelcome, or problematic terms in names."
}

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
    linters: &mut [Box<dyn Linter>],
    model: &Model,
    fail_fast: bool,
) -> ModelResult<Vec<ActionIssue>> {
    let mut issues: Vec<ActionIssue> = Default::default();

    for linter in linters.iter_mut() {
        linter.check(model)?;
        let new_issues = linter.issues_mut();
        issues.append(new_issues);
        if fail_fast
            && new_issues
                .iter()
                .any(|issue| issue.level > IssueLevel::Warning)
        {
            break;
        }
    }
    Ok(issues)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

linter_or_validator_default_impl! { NamingConventions }

linter_or_validator_action_impl! { NamingConventions, "NamingConventions" }

impl Linter for NamingConventions {
    fn check(&mut self, model: &Model) -> ModelResult<()> {
        for shape in model.shapes() {
            if shape.has_trait(&ShapeID::from_str("smithy.api#trait").unwrap()) {
                self.check_trait_name(shape.id());
            } else {
                self.check_shape_name(shape.id(), false);
            }
            self.check_applied_trait_names(shape.traits());

            match shape.body() {
                // TODO: term used in names
                // (https://github.com/johnstonskj/rust-atelier/issues/7)
                ShapeKind::Structure(body) | ShapeKind::Union(body) => {
                    for member in body.members() {
                        self.check_member_name(member.id());
                        self.check_shape_name(member.target(), true);
                        self.check_applied_trait_names(member.traits());
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl NamingConventions {
    fn check_shape_name(&mut self, id: &ShapeID, reference: bool) {
        let shape_name = id.shape_name().to_string();
        if shape_name.to_camel_case() != shape_name {
            self.issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "{} shape names should conform to UpperCamelCase, i.e. {}",
                    if reference {
                        "References to"
                    } else {
                        "Defined"
                    },
                    shape_name.to_camel_case()
                ),
                id.clone(),
            ));
        }
    }
    fn check_trait_name(&mut self, id: &ShapeID) {
        let shape_name = id.shape_name().to_string();
        if shape_name.to_mixed_case() != shape_name {
            self.issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "Trait names should conform to lowerCamelCase, i.e. {}",
                    shape_name.to_mixed_case()
                ),
                id.clone(),
            ));
        }
    }
    fn check_member_name(&mut self, id: &ShapeID) {
        let shape_name = id.member_name().as_ref().unwrap();
        let shape_name = shape_name.to_string();
        if shape_name.to_mixed_case() != shape_name {
            self.issues.push(ActionIssue::info_at(
                &self.label(),
                &format!(
                    "Member names should conform to lowerCamelCase, i.e. {}",
                    shape_name.to_mixed_case()
                ),
                id.clone(),
            ));
        }
    }
    fn check_applied_trait_names(&mut self, traits: &[AppliedTrait]) {
        for a_trait in traits {
            self.check_trait_name(a_trait.id());
        }
    }
}

// ------------------------------------------------------------------------------------------------

linter_or_validator_default_impl! { UnwelcomeTerms { checked: RefCell::new(Default::default()) } }

linter_or_validator_action_impl! { UnwelcomeTerms, "UnwelcomeTerms" }

impl Linter for UnwelcomeTerms {
    fn check(&mut self, model: &Model) -> ModelResult<()> {
        for shape in model.shapes() {
            let shape_id = shape.id();
            self.check_shape_id(shape_id);
            for a_trait in shape.traits() {
                self.check_shape_id(a_trait.id());
            }

            match shape.body() {
                ShapeKind::Structure(body) | ShapeKind::Union(body) => {
                    for member in body.members() {
                        self.check_shape_id(member.id());
                        self.check_shape_id(member.target());
                        for a_trait in member.traits() {
                            self.check_shape_id(a_trait.id());
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl UnwelcomeTerms {
    fn check_shape_id(&mut self, shape_id: &ShapeID) {
        for id in shape_id.namespace().split() {
            self.check_identifier(&id, Some(shape_id));
        }
        self.check_identifier(&shape_id.shape_name(), Some(shape_id));
        if let Some(member_name) = shape_id.member_name() {
            self.check_identifier(member_name, Some(shape_id));
        }
    }

    #[inline]
    fn check_identifier(&mut self, id: &Identifier, in_shape: Option<&ShapeID>) {
        if !self.checked.borrow().contains(id) {
            let _ = self.checked.borrow_mut().insert(id.clone());
            for word in terms::split_words(&id.to_string()) {
                if terms::is_unwelcome_term(&word) {
                    self.issues.push(match in_shape {
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
