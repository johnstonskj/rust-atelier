use crate::builder::{traits, TraitBuilder};
use crate::error::ErrorSource;
use crate::model::shapes::Simple;
use crate::model::ShapeID;
use crate::prelude::{
    PRELUDE_NAMESPACE, SHAPE_BIGDECIMAL, SHAPE_BIGINTEGER, SHAPE_BLOB, SHAPE_BOOLEAN, SHAPE_BYTE,
    SHAPE_DOCUMENT, SHAPE_DOUBLE, SHAPE_FLOAT, SHAPE_INTEGER, SHAPE_LONG, SHAPE_SHORT,
    SHAPE_STRING, SHAPE_TIMESTAMP,
};
use crate::syntax::{MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE};
use std::collections::HashMap;
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! add_trait {
    ($vis:vis $trait_fn:ident) => {
        /// Add the correspondingly named prelude trait to this model element
        $vis fn $trait_fn(&mut self) -> &mut Self {
            self.apply_trait(traits::$trait_fn().into())
        }
    };
    // ($trait_fn:ident) => {
    //     fn $trait_fn(&mut self) -> &mut Self {
    //         self.apply_trait(traits::$trait_fn().into())
    //     }
    // };
    ($vis:vis $trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        /// Add the correspondingly named prelude trait, and value(s), to this model element
        $vis fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.apply_trait(traits::$trait_fn($( $i ),*).into())
        }
    };
    // ($trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
    //     fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
    //         self.apply_trait(traits::$trait_fn($( $i ),*).into())
    //     }
    // };
}

macro_rules! from_mut {
    ($builder:ident) => {
        impl From<&mut $builder> for $builder {
            fn from(v: &mut $builder) -> Self {
                <$builder>::clone(<&mut $builder>::deref(&v))
            }
        }
    };
}

macro_rules! shape_traits_impl {
    ($builder:ident) => {
        impl ShapeTraits for $builder {
            fn apply_trait(&mut self, a_trait: TraitBuilder) -> &mut Self {
                self.applied_traits.push(a_trait);
                self
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `ShapeKind::Simple` shapes.
#[derive(Clone, Debug)]
pub struct SimpleShapeBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) simple_shape: Simple,
}

/// Builder for `ShapeKind::List` shapes.
#[derive(Clone, Debug)]
pub struct ListBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) member: MemberBuilder,
}

/// Builder for `ShapeKind::Map` shapes.
#[derive(Clone, Debug)]
pub struct MapBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) key: MemberBuilder,
    pub(super) value: MemberBuilder,
}

/// Builder for `ShapeKind::Structure` shapes.
#[derive(Clone, Debug)]
pub struct StructureBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) members: Vec<MemberBuilder>,
}

/// Builder for `ShapeKind::Service` shapes.
#[derive(Clone, Debug)]
pub struct ServiceBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) version: String,
    pub(super) operations: Vec<String>,
    pub(super) resources: Vec<String>,
}

/// Builder for `ShapeKind::Operation` shapes.
#[derive(Clone, Debug)]
pub struct OperationBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) input: Option<String>,
    pub(super) output: Option<String>,
    pub(super) errors: Vec<String>,
}

/// Builder for `ShapeKind::Resource` shapes.
#[derive(Clone, Debug)]
pub struct ResourceBuilder {
    pub(super) shape_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) identifiers: HashMap<String, String>,
    pub(super) create: Option<String>,
    pub(super) put: Option<String>,
    pub(super) read: Option<String>,
    pub(super) update: Option<String>,
    pub(super) delete: Option<String>,
    pub(super) list: Option<String>,
    pub(super) operations: Vec<String>,
    pub(super) collection_operations: Vec<String>,
    pub(super) resources: Vec<String>,
}
/// Builder for `ShapeKind::Unresolved` shapes.
#[derive(Clone, Debug)]
pub struct ReferenceBuilder {
    pub(super) shape_id: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
}

/// Builder for `MemberShape` shapes.
#[derive(Clone, Debug)]
pub struct MemberBuilder {
    pub(super) member_name: String,
    pub(super) applied_traits: Vec<TraitBuilder>,
    pub(super) target: String,
}

///
/// Provides all the traits for Smithy where trait selector = "*".
pub trait ShapeTraits {
    /// Add the provided trait builder to this shape.
    fn apply_trait(&mut self, a_trait: TraitBuilder) -> &mut Self
    where
        Self: Sized;

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn documentation(&mut self, text: &str) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::documentation(text))
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn external_documentation(&mut self, map: &[(&str, &str)]) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::external_documentation(map))
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn deprecated(&mut self, message: Option<&str>, since: Option<&str>) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::deprecated(message, since))
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn private(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::private())
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn since(&mut self, date: &str) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::since(date))
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn tagged(&mut self, tags: &[&str]) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::tagged(tags))
    }

    /// Add the correspondingly named prelude trait, and value(s), to this model element
    fn unstable(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.apply_trait(traits::unstable())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

from_mut! { SimpleShapeBuilder }

shape_traits_impl! { SimpleShapeBuilder }

impl SimpleShapeBuilder {
    ///Construct a new simple shape builder.
    pub fn new(shape_name: &str, simple_shape: Simple) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            simple_shape,
        }
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn blob(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Blob)
    }

    ///Construct a new simple shape builder for Simple::Boolean.
    pub fn boolean(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Boolean)
    }

    ///Construct a new simple shape builder for Simple::Document.
    pub fn document(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Document)
    }

    ///Construct a new simple shape builder for Simple::String.
    pub fn string(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::String)
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn byte(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Byte)
    }

    ///Construct a new simple shape builder for Simple::Short.
    pub fn short(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Short)
    }

    ///Construct a new simple shape builder for Simple::Integer.
    pub fn integer(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Integer)
    }

    ///Construct a new simple shape builder for Simple::Long.
    pub fn long(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Long)
    }

    ///Construct a new simple shape builder for Simple::Float.
    pub fn float(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Float)
    }

    ///Construct a new simple shape builder for Simple::Double.
    pub fn double(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Double)
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn big_integer(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::BigInteger)
    }

    ///Construct a new simple shape builder for Simple::BigDecimal.
    pub fn big_decimal(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::BigDecimal)
    }

    ///Construct a new simple shape builder for Simple::Timestamp.
    pub fn timestamp(shape_name: &str) -> Self {
        Self::new(shape_name, Simple::Timestamp)
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub boxed);

    add_trait!(pub sensitive);

    add_trait!(pub streaming);
}

// ------------------------------------------------------------------------------------------------

from_mut! { ListBuilder }

shape_traits_impl! { ListBuilder }

impl ListBuilder {
    ///Construct a new list or set shape builder.
    pub fn new(shape_name: &str, member_target: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            member: MemberBuilder::new(MEMBER_MEMBER, member_target),
        }
    }

    /// Set the target type for members in this list or set.
    pub fn target(&mut self, member: MemberBuilder) {
        self.member = member;
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub sensitive);

    add_trait!(pub unique_items);
}

// ------------------------------------------------------------------------------------------------

from_mut! { MapBuilder }

shape_traits_impl! { MapBuilder }

impl MapBuilder {
    ///Construct a new map shape builder.
    pub fn new(shape_name: &str, key_target: &str, value_target: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            key: MemberBuilder::new(MEMBER_KEY, key_target),
            value: MemberBuilder::new(MEMBER_VALUE, value_target),
        }
    }

    /// Set the target type for the keys in this map.
    pub fn key(&mut self, key: MemberBuilder) {
        self.key = key;
    }

    /// Set the target type for the values in this map.
    pub fn value(&mut self, value: MemberBuilder) {
        self.value = value;
    }
}

// ------------------------------------------------------------------------------------------------

from_mut! { StructureBuilder }

shape_traits_impl! { StructureBuilder }

impl StructureBuilder {
    ///Construct a new structure or union shape builder.
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            members: Default::default(),
        }
    }

    /// Create a new member in this structure or union with the given identifier and target type.
    pub fn member(&mut self, member_name: &str, member_target: &str) -> &mut Self {
        let _ = self.add_member(MemberBuilder::new(member_name, member_target));
        self
    }

    /// Create a new member in this structure or union.
    pub fn add_member(&mut self, member: MemberBuilder) -> &mut Self {
        self.members.push(member);
        self
    }

    /// Create a new member in this structure or union with the type `Simple::Blob`.
    pub fn blob(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_BLOB)
    }

    /// Create a new member in this structure or union with the type `Simple::Boolean`.
    pub fn boolean(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_BOOLEAN)
    }

    /// Create a new member in this structure or union with the type `Simple::Document`.
    pub fn document(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_DOCUMENT)
    }

    /// Create a new member in this structure or union with the type `Simple::String`.
    pub fn string(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_STRING)
    }

    /// Create a new member in this structure or union with the type `Simple::Byte`.
    pub fn byte(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_BYTE)
    }

    /// Create a new member in this structure or union with the type `Simple::Short`.
    pub fn short(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_SHORT)
    }

    /// Create a new member in this structure or union with the type `Simple::Integer`.
    pub fn integer(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_INTEGER)
    }

    /// Create a new member in this structure or union with the type `Simple::Long`.
    pub fn long(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_LONG)
    }

    /// Create a new member in this structure or union with the type `Simple::Float`.
    pub fn float(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_FLOAT)
    }

    /// Create a new member in this structure or union with the type `Simple::Double`.
    pub fn double(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_DOUBLE)
    }

    /// Create a new member in this structure or union with the type `Simple::BigInteger`.
    pub fn big_integer(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_BIGINTEGER)
    }

    /// Create a new member in this structure or union with the type `Simple::BigDecimal`.
    pub fn big_decimal(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_BIGDECIMAL)
    }

    /// Create a new member in this structure or union with the type `Simple::Timestamp`.
    pub fn timestamp(&mut self, id: &str) -> &mut Self {
        self.member(id, SHAPE_TIMESTAMP)
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub error_source(src: ErrorSource));

    add_trait!(pub sensitive);
}

// ------------------------------------------------------------------------------------------------

from_mut! { ServiceBuilder }

shape_traits_impl! { ServiceBuilder }

impl ServiceBuilder {
    pub fn new(shape_name: &str, version: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            version: version.to_string(),
            operations: Default::default(),
            resources: Default::default(),
        }
    }

    /// Set the version of this service.
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = version.to_string();
        self
    }

    /// Add an operation by reference to this service
    pub fn operation(&mut self, shape_id: &str) -> &mut Self {
        self.operations.push(shape_id.to_string());
        self
    }

    /// Add a list of operations by reference to this service
    pub fn operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.operation(shape_id);
        }
        self
    }

    /// Add a resource by reference to this service
    pub fn resource(&mut self, shape_id: &str) -> &mut Self {
        self.resources.push(shape_id.to_string());
        self
    }

    /// Add a list of resources by reference to this service
    pub fn resources(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.resource(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub sensitive);

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

from_mut! { OperationBuilder }

shape_traits_impl! { OperationBuilder }

impl OperationBuilder {
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            input: None,
            output: None,
            errors: Default::default(),
        }
    }

    /// Set the input type for this operation.
    pub fn input(&mut self, shape_id: &str) -> &mut Self {
        self.input = Some(shape_id.to_string());
        self
    }

    /// Set the output type for this operation.
    pub fn output(&mut self, shape_id: &str) -> &mut Self {
        self.output = Some(shape_id.to_string());
        self
    }

    /// Set an error type for this operation.
    pub fn error(&mut self, shape_id: &str) -> &mut Self {
        self.errors.push(shape_id.to_string());
        self
    }

    /// Set a list of error types for this operation.
    pub fn errors(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.error(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub idempotent);

    add_trait!(pub readonly);

    add_trait!(pub sensitive);

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));
}

// ------------------------------------------------------------------------------------------------

from_mut! { ResourceBuilder }

shape_traits_impl! { ResourceBuilder }

impl ResourceBuilder {
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.to_string(),
            applied_traits: Default::default(),
            identifiers: Default::default(),
            create: None,
            put: None,
            read: None,
            update: None,
            delete: None,
            list: None,
            operations: vec![],
            collection_operations: vec![],
            resources: vec![],
        }
    }

    /// Add an identifier to this resource; this represents the local member name and shape
    /// identifier for the member's type.
    pub fn identifier(&mut self, id: &str, shape_id: &str) -> &mut Self {
        let _ = self
            .identifiers
            .insert(id.to_string(), shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **create** actions.
    pub fn create(&mut self, shape_id: &str) -> &mut Self {
        self.create = Some(shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **put** actions.
    pub fn put(&mut self, shape_id: &str) -> &mut Self {
        self.put = Some(shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **read** actions.
    pub fn read(&mut self, shape_id: &str) -> &mut Self {
        self.read = Some(shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **update** actions.
    pub fn update(&mut self, shape_id: &str) -> &mut Self {
        self.update = Some(shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **delete** actions.
    pub fn delete(&mut self, shape_id: &str) -> &mut Self {
        self.delete = Some(shape_id.to_string());
        self
    }

    /// Set the operation identifier that handles **list** actions.
    pub fn list(&mut self, shape_id: &str) -> &mut Self {
        self.list = Some(shape_id.to_string());
        self
    }

    /// Add an operation by reference to this service
    pub fn operation(&mut self, shape_id: &str) -> &mut Self {
        self.operations.push(shape_id.to_string());
        self
    }

    /// Add a list of operations by reference to this service
    pub fn operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.operation(shape_id);
        }
        self
    }

    /// Add a collection operation by reference to this service
    pub fn collection_operation(&mut self, shape_id: &str) -> &mut Self {
        self.collection_operations.push(shape_id.to_string());
        self
    }

    /// Add a list of collection operations by reference to this service
    pub fn collection_operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.collection_operation(shape_id);
        }
        self
    }

    /// Add a resource by reference to this service
    pub fn resource(&mut self, shape_id: &str) -> &mut Self {
        self.resources.push(shape_id.to_string());
        self
    }

    /// Add a list of resources by reference to this service
    pub fn resources(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.resource(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub sensitive);

    add_trait!(pub no_replace);

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

from_mut! { ReferenceBuilder }

shape_traits_impl! { ReferenceBuilder }

impl ReferenceBuilder {
    /// Construct a new `ShapeKind::Unresolved` builder, with id.
    pub fn new(id: &str) -> Self {
        Self {
            shape_id: id.to_string(),
            applied_traits: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

from_mut! { MemberBuilder }

shape_traits_impl! { MemberBuilder }

impl MemberBuilder {
    /// Construct a new member shape builder, with id target
    pub fn new(member_name: &str, target: &str) -> Self {
        Self {
            member_name: member_name.to_string(),
            applied_traits: Default::default(),
            target: target.to_string(),
        }
    }

    fn new_unchecked(member_name: &str, target_namespace: &str, target_shape_name: &str) -> Self {
        Self {
            member_name: member_name.to_string(),
            applied_traits: Default::default(),
            target: ShapeID::new_unchecked(target_namespace, target_shape_name, None).to_string(),
        }
    }

    /// Return the name of this member.
    pub fn name(&self) -> &String {
        &self.member_name
    }

    /// Return the target type of this member.
    pub fn target(&self) -> &String {
        &self.target
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Blob`.
    pub fn blob(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_BLOB)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Boolean`.
    pub fn boolean(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_BOOLEAN)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Document`.
    pub fn document(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_DOCUMENT)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::String`.
    pub fn string(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_STRING)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Byte`.
    pub fn byte(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_BYTE)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Short`.
    pub fn short(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_SHORT)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Integer`.
    pub fn integer(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_INTEGER)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Long`.
    pub fn long(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_LONG)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Float`.
    pub fn float(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_FLOAT)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Double`.
    pub fn double(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_DOUBLE)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigInteger`.
    pub fn big_integer(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_BIGINTEGER)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigDecimal`.
    pub fn big_decimal(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_BIGDECIMAL)
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Timestamp`.
    pub fn timestamp(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, SHAPE_TIMESTAMP)
    }

    // --------------------------------------------------------------------------------------------

    add_trait!(pub required);
}
