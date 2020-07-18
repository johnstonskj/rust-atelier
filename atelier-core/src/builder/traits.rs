use crate::builder::values::ObjectBuilder;
use crate::error::ErrorKind;
use crate::error::ErrorSource;
use crate::model::shapes::AppliedTrait;
use crate::model::values::{Number, Value, ValueMap};
use crate::model::ShapeID;
use crate::prelude::PRELUDE_NAMESPACE;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `Trait` model elements.
#[derive(Clone, Debug)]
pub struct TraitBuilder {
    a_trait: AppliedTrait,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&mut TraitBuilder> for AppliedTrait {
    fn from(builder: &mut TraitBuilder) -> Self {
        builder.a_trait.clone()
    }
}

impl From<TraitBuilder> for AppliedTrait {
    fn from(builder: TraitBuilder) -> Self {
        builder.a_trait
    }
}

impl TraitBuilder {
    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn boxed() -> Self {
        Self::new_unchecked("box")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn deprecated(message: Option<&str>, since: Option<&str>) -> Self {
        let mut values = ObjectBuilder::default();
        if let Some(message) = message {
            let _ = values.string(&format!("{}#message", PRELUDE_NAMESPACE), message);
        }
        if let Some(since) = since {
            let _ = values.string(&format!("{}#since", PRELUDE_NAMESPACE), since);
        }
        Self::with_value_unchecked("deprecated", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn documentation(value: &str) -> Self {
        Self::with_value_unchecked("documentation", value.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn error(src: ErrorSource) -> Self {
        Self::with_value_unchecked("error", src.to_string().into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn external_documentation(map: &[(&str, &str)]) -> Self {
        let value: ValueMap = map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string().into()))
            .collect();
        Self::with_value_unchecked("externalDocumentation", value.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn idempotent() -> Self {
        Self::new_unchecked("idempotent")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn length(min: Option<usize>, max: Option<usize>) -> Self {
        assert!(min.is_some() || max.is_some());
        let mut values = ObjectBuilder::default();
        if let Some(min) = min {
            let _ = values.integer(&format!("{}#min", PRELUDE_NAMESPACE), min as i64);
        }
        if let Some(max) = max {
            let _ = values.integer(&format!("{}#max", PRELUDE_NAMESPACE), max as i64);
        }
        Self::with_value_unchecked("length", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn no_replace() -> Self {
        Self::new_unchecked("noReplace")
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
            let _ = values.string(&format!("{}#inputToken", PRELUDE_NAMESPACE), input_token);
        }
        if let Some(output_token) = output_token {
            let _ = values.string(&format!("{}#outputToken", PRELUDE_NAMESPACE), output_token);
        }
        if let Some(items) = items {
            let _ = values.string(&format!("{}#items", PRELUDE_NAMESPACE), items);
        }
        if let Some(page_size) = page_size {
            let _ = values.string(&format!("{}#pageSize", PRELUDE_NAMESPACE), page_size);
        }
        Self::with_value_unchecked("paginated", values.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn pattern(pat: &str) -> Self {
        assert!(!pat.is_empty());
        Self::with_value_unchecked("pattern", pat.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn private() -> Self {
        Self::new_unchecked("private")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn readonly() -> Self {
        Self::new_unchecked("readonly")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn references(reference_list: Value) -> Self {
        match reference_list {
            Value::Array(_) => Self::with_value_unchecked("references", reference_list),
            _ => panic!("{}", ErrorKind::InvalidValueVariant("Array".to_string())),
        }
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn required() -> Self {
        Self::new_unchecked("required")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn requires_length() -> Self {
        Self::new_unchecked("requiresLength")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn sensitive() -> Self {
        Self::new_unchecked("sensitive")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn streaming() -> Self {
        Self::new_unchecked("streaming")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn since(date: &str) -> Self {
        assert!(!date.is_empty());
        Self::with_value_unchecked("since", date.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn tagged(tags: &[&str]) -> Self {
        Self::with_value_unchecked(
            "tags",
            Value::Array(tags.iter().map(|s| (*s).into()).collect()),
        )
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn title(title: &str) -> Self {
        Self::with_value_unchecked("title", title.into())
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn is_trait() -> Self {
        Self::new_unchecked("trait")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn unique_items() -> Self {
        Self::new_unchecked("uniqueItems")
    }

    /// Create a new `TraitBuilder` for the corresponding prelude trait.
    pub fn unstable() -> Self {
        Self::new_unchecked("unstable")
    }

    // --------------------------------------------------------------------------------------------

    /// Construct a new `TraitBuilder` with the given shape identifier.
    pub fn new(id: &str) -> Self {
        Self {
            a_trait: AppliedTrait::new(ShapeID::from_str(id).unwrap()),
        }
    }

    /// Construct a new `TraitBuilder` with the given shape identifier and value.
    pub fn with_value(id: &str, value: Value) -> Self {
        Self {
            a_trait: AppliedTrait::with_value(ShapeID::from_str(id).unwrap(), value),
        }
    }

    fn new_unchecked(shape_name: &str) -> Self {
        Self {
            a_trait: AppliedTrait::new(ShapeID::new_unchecked(PRELUDE_NAMESPACE, shape_name, None)),
        }
    }
    fn with_value_unchecked(shape_name: &str, value: Value) -> Self {
        Self {
            a_trait: AppliedTrait::with_value(
                ShapeID::new_unchecked(PRELUDE_NAMESPACE, shape_name, None),
                value,
            ),
        }
    }

    // --------------------------------------------------------------------------------------------

    /// Sets the value for this trait to be an array.
    pub fn array(&mut self, value: Vec<Value>) -> &mut Self {
        self.value(value.into())
    }

    /// Sets the value for this trait to be an object.
    pub fn object(&mut self, value: ValueMap) -> &mut Self {
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
    pub fn value(&mut self, value: Value) -> &mut Self {
        self.a_trait.set_value(value);
        self
    }
}
