use crate::error::ErrorKind;
use crate::error::{AndPanic, ErrorSource};
use crate::model::builder::values::ObjectBuilder;
use crate::model::shapes::{Trait, Valued};
use crate::model::values::{Key, NodeValue};
use crate::model::{Identifier, ShapeID};
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct TraitBuilder {
    a_trait: Trait,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TraitBuilder {
    pub fn boxed() -> Self {
        Self::new("box")
    }

    pub fn deprecated(message: Option<&str>, since: Option<&str>) -> Self {
        let mut values = ObjectBuilder::default();
        if let Some(message) = message {
            values.string(Identifier::from_str("message").unwrap().into(), message);
        }
        if let Some(since) = since {
            values.string(Identifier::from_str("since").unwrap().into(), since);
        }
        Self::with_value("length", values.build())
    }

    pub fn documentation(value: &str) -> Self {
        Self::new("deprecated").string(value).clone()
    }

    pub fn error(src: ErrorSource) -> Self {
        Self::new("error").string(&src.to_string()).clone()
    }

    pub fn external_documentation(map: &[(&str, &str)]) -> Self {
        let value: HashMap<Key, NodeValue> = map
            .into_iter()
            .map(|(k, v)| (k.to_string().into(), v.to_string().into()))
            .collect();
        Self::with_value("externalDocumentation", value.into())
    }

    pub fn idempotent() -> Self {
        Self::new("idempotent")
    }

    pub fn length(min: Option<usize>, max: Option<usize>) -> Self {
        assert!(min.is_some() || max.is_some());
        let mut values = ObjectBuilder::default();
        if let Some(min) = min {
            values.integer(Identifier::from_str("min").unwrap().into(), min as i64);
        }
        if let Some(max) = max {
            values.integer(Identifier::from_str("max").unwrap().into(), max as i64);
        }
        Self::with_value("length", values.build())
    }

    pub fn no_replace() -> Self {
        Self::new("noReplace")
    }

    pub fn paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>,
    ) -> Self {
        let mut values = ObjectBuilder::default();
        if let Some(input_token) = input_token {
            values.string(
                Identifier::from_str("inputToken").unwrap().into(),
                input_token,
            );
        }
        if let Some(output_token) = output_token {
            values.string(
                Identifier::from_str("outputToken").unwrap().into(),
                output_token,
            );
        }
        if let Some(items) = items {
            values.string(Identifier::from_str("items").unwrap().into(), items);
        }
        if let Some(page_size) = page_size {
            values.string(Identifier::from_str("pageSize").unwrap().into(), page_size);
        }
        Self::with_value("paginated", values.build())
    }

    pub fn pattern(pat: &str) -> Self {
        assert!(!pat.is_empty());
        Self::new("pattern").string(pat).clone()
    }

    pub fn private() -> Self {
        Self::new("private")
    }

    pub fn readonly() -> Self {
        Self::new("readonly")
    }

    pub fn references(reference_list: NodeValue) -> Self {
        match reference_list {
            NodeValue::Array(_) => Self::with_value("references", reference_list),
            _ => ErrorKind::InvalidValueVariant("Array".to_string()).panic(),
        }
    }

    pub fn required() -> Self {
        Self::new("required")
    }

    pub fn requires_length() -> Self {
        Self::new("requiresLength")
    }

    pub fn sensitive() -> Self {
        Self::new("sensitive")
    }

    pub fn streaming() -> Self {
        Self::new("streaming")
    }

    pub fn since(date: &str) -> Self {
        assert!(!date.is_empty());
        Self::new("since").string(date).clone()
    }

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

    pub fn title(title: &str) -> Self {
        Self::new("title").string(title).clone()
    }

    pub fn is_trait() -> Self {
        Self::new("trait")
    }

    pub fn unique_items() -> Self {
        Self::new("uniqueItems")
    }

    pub fn unstable() -> Self {
        Self::new("unstable")
    }

    // --------------------------------------------------------------------------------------------

    pub fn new(id: &str) -> Self {
        Self {
            a_trait: Trait::new(ShapeID::from_str(id).unwrap()),
        }
    }

    pub fn with_value(id: &str, value: NodeValue) -> Self {
        Self {
            a_trait: Trait::with_value(ShapeID::from_str(id).unwrap(), value),
        }
    }

    pub fn array(&mut self, value: Vec<NodeValue>) -> &mut Self {
        self.value(value.into())
    }

    pub fn object(&mut self, value: HashMap<Key, NodeValue>) -> &mut Self {
        self.value(value.into())
    }

    pub fn integer(&mut self, value: i64) -> &mut Self {
        self.value(value.into())
    }

    pub fn float(&mut self, value: f64) -> &mut Self {
        self.value(value.into())
    }

    pub fn boolean(&mut self, value: bool) -> &mut Self {
        self.value(value.into())
    }

    pub fn reference(&mut self, value: ShapeID) -> &mut Self {
        self.value(value.into())
    }

    pub fn string(&mut self, value: &str) -> &mut Self {
        self.value(value.to_string().into())
    }

    pub fn value(&mut self, value: NodeValue) -> &mut Self {
        self.a_trait.set_value(value);
        self
    }

    pub fn build(&self) -> Trait {
        self.a_trait.clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
