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
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
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

lazy_static! {
    static ref KNOWN_ACRONYMS: HashSet<&'static str> = standard_acronyms();
}

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
        self.check_for_acronyms("Shape", id, id.shape_name(), false, issues);
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
        self.check_for_acronyms("Trait", id, id.shape_name(), true, issues);
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
        self.check_for_acronyms("Member", shape_id, member_id, true, issues);
    }
    fn check_applied_trait_names(&self, traits: &[Trait], issues: &mut Vec<ActionIssue>) {
        for a_trait in traits {
            self.check_trait_name(a_trait.id(), issues);
        }
    }
    fn check_for_acronyms(
        &self,
        shape_kind: &str,
        shape_id: &ShapeID,
        id: &Identifier,
        lower: bool,
        issues: &mut Vec<ActionIssue>,
    ) {
        let acronym = Regex::new(r"[[:lower:]]?([[:upper:]][[:upper:]]+)").unwrap();
        let name = id.to_string();
        for cap in acronym.captures_iter(&name) {
            let word = &cap[1];
            if KNOWN_ACRONYMS.contains(word) {
                issues.push(ActionIssue::info_at(
                    &self.label(),
                    &format!(
                        "{} name '{}' appears to contain a known acronym, consider renaming i.e. {}",
                        shape_kind,
                        name,
                        if lower && name.starts_with(word) {
                            word.to_mixed_case()
                        } else {
                            word.to_camel_case()
                        }
                    ),
                    shape_id.clone(),
                ));
            }
            let word = &word[..word.len() - 1];
            if word.len() >= 2 && KNOWN_ACRONYMS.contains(word) {
                issues.push(ActionIssue::info_at(
                    &self.label(),
                    &format!(
                        "{} name '{}' appears to contain a known acronym, consider renaming i.e. {}",
                        shape_kind,
                        name,
                        if lower && name.starts_with(word) {
                            word.to_mixed_case()
                        } else {
                            word.to_camel_case()
                        }
                    ),
                    shape_id.clone(),
                ));
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn standard_acronyms() -> HashSet<&'static str> {
    HashSet::from_iter(
        [
            "ARN", "AWS", "CPU", "DB", "HTML", "IAM", "ID", "JSON", "OK", "PID", "RDF", "REST",
            "SES", "SMS", "SNS", "SQS", "XHML", "XML",
        ]
        .iter()
        .copied(),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        use crate::action::lint::{run_linter_actions, NamingConventions};
        use crate::model::builder::{
            ModelBuilder, ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
        };
        use crate::model::Model;
        use crate::Version;

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
        let result = run_linter_actions(&[Box::new(NamingConventions::default())], &model, false);
        println!("{:#?}", result);
    }
}
