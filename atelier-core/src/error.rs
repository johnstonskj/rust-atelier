/*!
Standard `Error`, `ErrorKind`, and `Result` types.
*/

#![allow(missing_docs)]
#![allow(clippy::upper_case_acronyms)]

use crate::action::ActionIssue;
use crate::model::identity::ShapeID;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

error_chain! {
    errors {
        // ----------------------------------------------------------------------------------------
        // Shape Identifier Errors
        // ----------------------------------------------------------------------------------------

        #[doc = "Invalid shape ID format."]
        InvalidShapeID(id: String) {
            description("Invalid shape ID format.")
            display("Invalid shape ID: '{}'.", id)
        }

        #[doc = "Expected an absolute shape ID."]
        AbsoluteShapeIDExpected(id: ShapeID) {
            description("Expected an absolute shape ID.")
            display("Expected an absolute shape ID: '{}'.", id)
        }

        #[doc = "Expected a shape, not member, ID."]
        ShapeIDExpected(id: ShapeID) {
            description("Expected a shape, not member, ID.")
            display("Expected a shape, not member, ID: '{}'.", id)
        }

        #[doc = "Expected a member, not shape, ID."]
        MemberIDExpected(id: ShapeID) {
            description("Expected a member, not shape, ID.")
            display("Expected a member, not shape, ID: '{}'.", id)
        }

        // ----------------------------------------------------------------------------------------
        // Shape Resolution Errors
        // ----------------------------------------------------------------------------------------

        #[doc = "A reference to an unknown shape ID was encountered."]
        UnknownShape(s: String) {
            description("A reference to an unknown shape ID was encountered.")
            display("A reference to an unknown shape ID was encountered: '{}'.", s)
        }

        #[doc = "An unknown member ID was encountered."]
        UnknownMember(s: String) {
            description("An unknown member ID was encountered.")
            display("An unknown member ID was encountered: '{}'.", s)
        }

        #[doc = "A shape name resolved to multiple shape IDs"]
        AmbiguousShape(s: String) {
            description("A shape name resolved to multiple shape IDs")
            display("A shape name resolved to multiple shape IDs: '{}'.", s)
        }

        // ----------------------------------------------------------------------------------------
        // Shape Resolution Errors
        // ----------------------------------------------------------------------------------------

        #[doc = "Merge cannot take place between models with different versions."]
        MergeVersionConflict(v1: String, v2: String) {
            description("Merge cannot take place between models with different versions.")
            display("Merge cannot take place between models with different versions ('{}' != '{}').", v1, v2)
        }

        #[doc = "A merge conflict occurred between two shapes."]
        MergeShapeConflict(id: ShapeID) {
            description("A merge conflict occurred between two shapes.")
            display("A merge conflict occurred between two shapes named '{}'.", id)
        }

        #[doc = "A merge conflict occurred between two applied traits."]
        MergeTraitConflict(id: ShapeID) {
            description("A merge conflict occurred between two applied traits.")
            display("A merge conflict occurred between two applied traits named '{}'.", id)
        }

        #[doc = "A merge conflict occurred between two metadata values."]
        MergeMetadataConflict(key: String) {
            description("A merge conflict occurred between two metadata values.")
            display("A merge conflict occurred between two metadata values keyed '{}'.", key)
        }

        // ----------------------------------------------------------------------------------------
        // Model Parsing Errors
        // ----------------------------------------------------------------------------------------

        #[doc = "Invalid, or missing, version number."]
        InvalidVersionNumber(v: Option<String>) {
            description("Invalid, or missing,, or missing, version number.")
            display("Invalid version number: '{:?}'.", v)
        }

        #[doc = "Invalid, or missing, value specified for trait."]
        InvalidTraitValue(name: String) {
            description("Invalid, or missing, value specified for trait.")
            display("Invalid, or missing, value specified for trait named '{}'.", name)
        }

        #[doc = "Invalid simple shape name."]
        InvalidSimpleShape(s: String) {
            description("Invalid simple shape name.")
            display("Invalid simple shape name: '{}'.", s)
        }

        #[doc = "The selector expression(s) failed to parse."]
        InvalidSelectorExpression(expr: String) {
            description("The selector expression(s) failed to parse.")
            display("The selector expression(s) failed to parse: '{}'.", expr)
        }

        // ----------------------------------------------------------------------------------------
        // Model Representation Errors
        // ----------------------------------------------------------------------------------------

        #[doc = "Requested action is not supported by the selected representation."]
        InvalidRepresentation(repr: String) {
            description("Requested action is not supported by the selected representation.")
            display("Requested action is not supported by the selected representation '{}'.", repr)
        }

        #[doc = "An error occurred serializing a model."]
        Serialization(repr: String) {
            description("An error occurred serializing a model.")
            display("An error occurred serializing a model into representation '{}'.", repr)
        }

        #[doc = "An error occurred de-serializing a model."]
        Deserialization(representation: String, location: String, context: Option<String>) {
            description("An error occurred de-serializing a model.")
            display("An error occurred de-serializing a model from representation '{}' at location '{}' (context '{:?}').", representation, location, context)
        }

        // ----------------------------------------------------------------------------------------
        // Action-Reported Error
        // ----------------------------------------------------------------------------------------

        #[doc = "Reporting issues found by an Action."]
        ActionIssue(reasons: Vec<ActionIssue>) {
            description("Reporting issues found by an Action.")
            display("Reporting issues found by an Action: {:?}", reasons)
        }
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}
