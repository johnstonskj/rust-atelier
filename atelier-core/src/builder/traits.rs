use crate::builder::values::ObjectBuilder;
use crate::error::ErrorKind;
use crate::error::ErrorSource;
use crate::model::values::{Number, Value, ValueMap};
use crate::model::ShapeID;
use crate::prelude::{
    PRELUDE_NAMESPACE, TRAIT_BOX, TRAIT_DEPRECATED, TRAIT_DOCUMENTATION, TRAIT_ERROR,
    TRAIT_EXTERNALDOCUMENTATION, TRAIT_IDEMPOTENT, TRAIT_LENGTH, TRAIT_NOREPLACE, TRAIT_PAGINATED,
    TRAIT_PATTERN, TRAIT_PRIVATE, TRAIT_READONLY, TRAIT_REFERENCES, TRAIT_REQUIRED,
    TRAIT_REQUIRESLENGTH, TRAIT_SENSITIVE, TRAIT_SINCE, TRAIT_STREAMING, TRAIT_TAGS, TRAIT_TITLE,
    TRAIT_TRAIT, TRAIT_UNIQUEITEMS, TRAIT_UNSTABLE,
};
use crate::syntax::SHAPE_ID_ABSOLUTE_SEPARATOR;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `AppliedTrait` model elements.
#[derive(Clone, Debug)]
pub struct TraitBuilder {
    pub(crate) shape_id: String,
    pub(crate) value: Option<Value>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn boxed() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_BOX))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn deprecated(message: Option<&str>, since: Option<&str>) -> TraitBuilder {
    let mut values = ObjectBuilder::default();
    if let Some(message) = message {
        let _ = values.string(&format!("{}#message", PRELUDE_NAMESPACE), message);
    }
    if let Some(since) = since {
        let _ = values.string(&format!("{}#since", PRELUDE_NAMESPACE), since);
    }
    TraitBuilder::with_value(&prelude_name(TRAIT_DEPRECATED), values.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn documentation(value: &str) -> TraitBuilder {
    TraitBuilder::with_value(&prelude_name(TRAIT_DOCUMENTATION), value.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn error_source(src: ErrorSource) -> TraitBuilder {
    TraitBuilder::with_value(&prelude_name(TRAIT_ERROR), src.to_string().into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn external_documentation(map: &[(&str, &str)]) -> TraitBuilder {
    let value: ValueMap = map
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string().into()))
        .collect();
    TraitBuilder::with_value(&prelude_name(TRAIT_EXTERNALDOCUMENTATION), value.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn idempotent() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_IDEMPOTENT))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn length(min: Option<usize>, max: Option<usize>) -> TraitBuilder {
    assert!(min.is_some() || max.is_some());
    let mut values = ObjectBuilder::default();
    if let Some(min) = min {
        let _ = values.integer(&format!("{}#min", PRELUDE_NAMESPACE), min as i64);
    }
    if let Some(max) = max {
        let _ = values.integer(&format!("{}#max", PRELUDE_NAMESPACE), max as i64);
    }
    TraitBuilder::with_value(&prelude_name(TRAIT_LENGTH), values.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn no_replace() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_NOREPLACE))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn paginated(
    input_token: Option<&str>,
    output_token: Option<&str>,
    items: Option<&str>,
    page_size: Option<&str>,
) -> TraitBuilder {
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
    TraitBuilder::with_value(&prelude_name(TRAIT_PAGINATED), values.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn pattern(pat: &str) -> TraitBuilder {
    assert!(!pat.is_empty());
    TraitBuilder::with_value(&prelude_name(TRAIT_PATTERN), pat.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn private() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_PRIVATE))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn readonly() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_READONLY))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn references(reference_list: Value) -> TraitBuilder {
    match reference_list {
        Value::Array(_) => {
            TraitBuilder::with_value(&prelude_name(TRAIT_REFERENCES), reference_list)
        }
        _ => panic!("{}", ErrorKind::InvalidValueVariant("Array".to_string())),
    }
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn required() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_REQUIRED))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn requires_length() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_REQUIRESLENGTH))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn sensitive() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_SENSITIVE))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn streaming() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_STREAMING))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn since(date: &str) -> TraitBuilder {
    assert!(!date.is_empty());
    TraitBuilder::with_value(&prelude_name(TRAIT_SINCE), date.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn tagged(tags: &[&str]) -> TraitBuilder {
    TraitBuilder::with_value(
        &prelude_name(TRAIT_TAGS),
        Value::Array(tags.iter().map(|s| (*s).into()).collect()),
    )
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn title(title: &str) -> TraitBuilder {
    TraitBuilder::with_value(&prelude_name(TRAIT_TITLE), title.into())
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn a_trait() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_TRAIT))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn unique_items() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_UNIQUEITEMS))
}

/// Create a new `TraitBuilder` for the corresponding prelude trait.
pub fn unstable() -> TraitBuilder {
    TraitBuilder::new(&prelude_name(TRAIT_UNSTABLE))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TraitBuilder {
    /// Construct a new `TraitBuilder` with the given shape identifier.
    pub fn new(id: &str) -> Self {
        Self {
            shape_id: id.to_string(),
            value: None,
        }
    }

    /// Construct a new `TraitBuilder` with the given shape identifier and value.
    pub fn with_value(id: &str, value: Value) -> Self {
        Self {
            shape_id: id.to_string(),
            value: Some(value),
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
        self.value = Some(value);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn prelude_name(name: &str) -> String {
    format!(
        "{}{}{}",
        PRELUDE_NAMESPACE, SHAPE_ID_ABSOLUTE_SEPARATOR, name
    )
}
