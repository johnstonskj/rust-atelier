use crate::error::ErrorKind;
use crate::error::{AndPanic, ErrorSource};
use crate::model::builder::values::ObjectBuilder;
use crate::model::shapes::{Trait, Valued};
use crate::model::values::{Key, NodeValue, Number};
use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `Trait` model elements.
#[derive(Clone, Debug)]
pub struct TraitBuilder {
    a_trait: Trait,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&mut TraitBuilder> for Trait {
    fn from(builder: &mut TraitBuilder) -> Self {
        builder.a_trait.clone()
    }
}

impl From<TraitBuilder> for Trait {
    fn from(builder: TraitBuilder) -> Self {
        builder.a_trait
    }
}

impl TraitBuilder {
    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn boxed() -> Self {
        Self::new("box")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn deprecated(message: Option<&str>, since: Option<&str>) -> Self {
        let mut values = ObjectBuilder::default();
        if let Some(message) = message {
            let _ = values.string(Identifier::from_str("message").unwrap().into(), message);
        }
        if let Some(since) = since {
            let _ = values.string(Identifier::from_str("since").unwrap().into(), since);
        }
        Self::with_value("deprecated", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn documentation(value: &str) -> Self {
        Self::new("documentation").string(value).to_owned()
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn error(src: ErrorSource) -> Self {
        Self::new("error").string(&src.to_string()).to_owned()
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn external_documentation(map: &[(&str, &str)]) -> Self {
        let value: HashMap<Key, NodeValue> = map
            .iter()
            .map(|(k, v)| (k.to_string().into(), v.to_string().into()))
            .collect();
        Self::with_value("externalDocumentation", value.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn idempotent() -> Self {
        Self::new("idempotent")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn length(min: Option<usize>, max: Option<usize>) -> Self {
        assert!(min.is_some() || max.is_some());
        let mut values = ObjectBuilder::default();
        if let Some(min) = min {
            let _ = values.integer(Identifier::from_str("min").unwrap().into(), min as i64);
        }
        if let Some(max) = max {
            let _ = values.integer(Identifier::from_str("max").unwrap().into(), max as i64);
        }
        Self::with_value("length", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn no_replace() -> Self {
        Self::new("noReplace")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>,
    ) -> Self {
        let mut values = ObjectBuilder::default();
        if let Some(input_token) = input_token {
            let _ = values.string(
                Identifier::from_str("inputToken").unwrap().into(),
                input_token,
            );
        }
        if let Some(output_token) = output_token {
            let _ = values.string(
                Identifier::from_str("outputToken").unwrap().into(),
                output_token,
            );
        }
        if let Some(items) = items {
            let _ = values.string(Identifier::from_str("items").unwrap().into(), items);
        }
        if let Some(page_size) = page_size {
            let _ = values.string(Identifier::from_str("pageSize").unwrap().into(), page_size);
        }
        Self::with_value("paginated", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn pattern(pat: &str) -> Self {
        assert!(!pat.is_empty());
        Self::new("pattern").string(pat).to_owned()
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn private() -> Self {
        Self::new("private")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn readonly() -> Self {
        Self::new("readonly")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn references(reference_list: NodeValue) -> Self {
        match reference_list {
            NodeValue::Array(_) => Self::with_value("references", reference_list),
            _ => ErrorKind::InvalidValueVariant("Array".to_string()).panic(),
        }
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn required() -> Self {
        Self::new("required")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn requires_length() -> Self {
        Self::new("requiresLength")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn sensitive() -> Self {
        Self::new("sensitive")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn streaming() -> Self {
        Self::new("streaming")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn since(date: &str) -> Self {
        assert!(!date.is_empty());
        Self::new("since").string(date).to_owned()
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn tagged(tags: &[&str]) -> Self {
        Self::with_value(
            "tags",
            NodeValue::Array(
                tags.iter()
                    .map(|s| NodeValue::String(s.to_string()))
                    .collect(),
            ),
        )
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn title(title: &str) -> Self {
        Self::new("title").string(title).to_owned()
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn is_trait() -> Self {
        Self::new("trait")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn unique_items() -> Self {
        Self::new("uniqueItems")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn unstable() -> Self {
        Self::new("unstable")
    }

    // --------------------------------------------------------------------------------------------

    /// Construct a new `TraitBuilder` with the given shape identifier.
    pub fn new(id: &str) -> Self {
        Self {
            a_trait: Trait::new(ShapeID::from_str(id).unwrap()),
        }
    }

    /// Construct a new `TraitBuilder` with the given shape identifier and value.
    pub fn with_value(id: &str, value: NodeValue) -> Self {
        Self {
            a_trait: Trait::with_value(ShapeID::from_str(id).unwrap(), value),
        }
    }

    /// Sets the value for this trait to be an array.
    pub fn array(&mut self, value: Vec<NodeValue>) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be an object.
    pub fn object(&mut self, value: HashMap<Key, NodeValue>) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be a number.
    pub fn number(&mut self, value: Number) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be an integer.
    pub fn integer(&mut self, value: i64) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be a float.
    pub fn float(&mut self, value: f64) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be a boolean.
    pub fn boolean(&mut self, value: bool) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be a shape identifier.
    pub fn reference(&mut self, value: ShapeID) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be a string.
    pub fn string(&mut self, value: &str) -> &mut Self {
        self.value(value.to_string().into())
    }

    /// Sets the value for this trait.
    pub fn value(&mut self, value: NodeValue) -> &mut Self {
        self.a_trait.set_value(value);
        self
    }
}
