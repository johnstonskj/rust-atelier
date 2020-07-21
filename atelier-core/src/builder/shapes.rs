use crate::builder::TraitBuilder;
use crate::error::ErrorSource;
use crate::model::shapes::{AppliedTrait, Simple};
use crate::model::values::{Value, ValueMap};
use crate::model::{Identifier, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;
use crate::syntax::{MEMBER_KEY, MEMBER_MEMBER, MEMBER_VALUE};
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! add_trait {
    (pub $trait_fn:ident) => {
        /// Add the correspondingly named prelude trait to this model element
        pub fn $trait_fn(&mut self) -> &mut Self {
            self.apply_trait(TraitBuilder::$trait_fn().into())
        }
    };
    ($trait_fn:ident) => {
        fn $trait_fn(&mut self) -> &mut Self {
            self.apply_trait(TraitBuilder::$trait_fn().into())
        }
    };
    (pub $trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        /// Add the correspondingly named prelude trait, and value(s), to this model element
        pub fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.apply_trait(TraitBuilder::$trait_fn($( $i ),*).into())
        }
    };
    ($trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.apply_trait(TraitBuilder::$trait_fn($( $i ),*).into())
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Builder for `ShapeKind::Simple` shapes.
#[derive(Clone, Debug)]
pub struct SimpleShapeBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) simple_shape: Simple,
}

/// Builder for `ShapeKind::List` shapes.
#[derive(Clone, Debug)]
pub struct ListBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) member: MemberBuilder,
}

/// Builder for `ShapeKind::Map` shapes.
#[derive(Clone, Debug)]
pub struct MapBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) key: MemberBuilder,
    pub(super) value: MemberBuilder,
}

/// Builder for `ShapeKind::Structure` shapes.
#[derive(Clone, Debug)]
pub struct StructureBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) members: Vec<MemberBuilder>,
}

/// Builder for `ShapeKind::Service` shapes.
#[derive(Clone, Debug)]
pub struct ServiceBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) version: String,
    pub(super) operations: Vec<ShapeID>,
    pub(super) resources: Vec<ShapeID>,
}

/// Builder for `ShapeKind::Operation` shapes.
#[derive(Clone, Debug)]
pub struct OperationBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) input: Option<ShapeID>,
    pub(super) output: Option<ShapeID>,
    pub(super) errors: Vec<ShapeID>,
}

/// Builder for `ShapeKind::Resource` shapes.
#[derive(Clone, Debug)]
pub struct ResourceBuilder {
    pub(super) shape_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) identifiers: ValueMap,
    pub(super) create: Option<ShapeID>,
    pub(super) put: Option<ShapeID>,
    pub(super) read: Option<ShapeID>,
    pub(super) update: Option<ShapeID>,
    pub(super) delete: Option<ShapeID>,
    pub(super) list: Option<ShapeID>,
    pub(super) operations: Vec<ShapeID>,
    pub(super) collection_operations: Vec<ShapeID>,
    pub(super) resources: Vec<ShapeID>,
}
/// Builder for `ShapeKind::Unresolved` shapes.
#[derive(Clone, Debug)]
pub struct ReferenceBuilder {
    pub(super) shape_id: ShapeID,
    pub(super) applied_traits: Vec<AppliedTrait>,
}

/// Builder for `MemberShape` shapes.
#[derive(Clone, Debug)]
pub struct MemberBuilder {
    pub(super) member_name: Identifier,
    pub(super) applied_traits: Vec<AppliedTrait>,
    pub(super) target: ShapeID,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SimpleShapeBuilder {
    ///Construct a new simple shape builder.
    pub fn new(shape_name: &str, simple_shape: Simple) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
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

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub boxed);

    add_trait!(pub sensitive);

    add_trait!(pub streaming);
}

// ------------------------------------------------------------------------------------------------

impl ListBuilder {
    ///Construct a new list or set shape builder.
    pub fn new(shape_name: &str, member_target: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
            applied_traits: Default::default(),
            member: MemberBuilder::new(MEMBER_MEMBER, member_target),
        }
    }

    pub fn target(&mut self, member: MemberBuilder) {
        self.member = member;
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub sensitive);

    add_trait!(pub unique_items);
}

// ------------------------------------------------------------------------------------------------

impl MapBuilder {
    ///Construct a new map shape builder.
    pub fn new(shape_name: &str, key_target: &str, value_target: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
            applied_traits: Default::default(),
            key: MemberBuilder::new(MEMBER_KEY, key_target),
            value: MemberBuilder::new(MEMBER_VALUE, value_target),
        }
    }

    pub fn key(&mut self, key: MemberBuilder) {
        self.key = key;
    }

    pub fn value(&mut self, value: MemberBuilder) {
        self.value = value;
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);
}

// ------------------------------------------------------------------------------------------------

impl StructureBuilder {
    ///Construct a new map shape builder.
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
            applied_traits: Default::default(),
            members: Default::default(),
        }
    }

    pub fn member(&mut self, member_name: &str, member_target: &str) -> &mut Self {
        let _ = self.add_member(MemberBuilder::new(member_name, member_target));
        self
    }
    pub fn add_member(&mut self, member: MemberBuilder) -> &mut Self {
        self.members.push(member);
        self
    }

    pub fn blob(&mut self, id: &str) -> &mut Self {
        self.member(id, "Blob")
    }

    pub fn boolean(&mut self, id: &str) -> &mut Self {
        self.member(id, "Boolean")
    }

    pub fn document(&mut self, id: &str) -> &mut Self {
        self.member(id, "Document")
    }

    pub fn string(&mut self, id: &str) -> &mut Self {
        self.member(id, "String")
    }

    pub fn byte(&mut self, id: &str) -> &mut Self {
        self.member(id, "Byte")
    }

    pub fn short(&mut self, id: &str) -> &mut Self {
        self.member(id, "Short")
    }

    pub fn integer(&mut self, id: &str) -> &mut Self {
        self.member(id, "Integer")
    }

    pub fn long(&mut self, id: &str) -> &mut Self {
        self.member(id, "Long")
    }

    pub fn float(&mut self, id: &str) -> &mut Self {
        self.member(id, "Float")
    }

    pub fn double(&mut self, id: &str) -> &mut Self {
        self.member(id, "Double")
    }

    pub fn big_integer(&mut self, id: &str) -> &mut Self {
        self.member(id, "BigInteger")
    }

    pub fn big_decimal(&mut self, id: &str) -> &mut Self {
        self.member(id, "BigDecimal")
    }

    pub fn timestamp(&mut self, id: &str) -> &mut Self {
        self.member(id, "Timestamp")
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub error_source(src: ErrorSource));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub sensitive);
}

// ------------------------------------------------------------------------------------------------

impl ServiceBuilder {
    pub fn new(shape_name: &str, version: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
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

    pub fn operation(&mut self, shape_id: &str) -> &mut Self {
        self.operations.push(shape_id.parse().unwrap());
        self
    }
    pub fn operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.operation(shape_id);
        }
        self
    }

    pub fn resource(&mut self, shape_id: &str) -> &mut Self {
        self.resources.push(shape_id.parse().unwrap());
        self
    }
    pub fn resources(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.resource(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub sensitive);

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

impl OperationBuilder {
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
            applied_traits: Default::default(),
            input: None,
            output: None,
            errors: Default::default(),
        }
    }

    pub fn input(&mut self, shape_id: &str) -> &mut Self {
        self.input = Some(shape_id.parse().unwrap());
        self
    }

    pub fn output(&mut self, shape_id: &str) -> &mut Self {
        self.output = Some(shape_id.parse().unwrap());
        self
    }

    pub fn error(&mut self, shape_id: &str) -> &mut Self {
        self.errors.push(shape_id.parse().unwrap());
        self
    }
    pub fn errors(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.error(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub readonly);

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub sensitive);

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));
}

// ------------------------------------------------------------------------------------------------

impl ResourceBuilder {
    pub fn new(shape_name: &str) -> Self {
        Self {
            shape_name: shape_name.parse().unwrap(),
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
            .insert(id.to_string(), Value::String(shape_id.to_string()));
        self
    }

    pub fn create(&mut self, shape_id: &str) -> &mut Self {
        self.create = Some(shape_id.parse().unwrap());
        self
    }

    pub fn put(&mut self, shape_id: &str) -> &mut Self {
        self.put = Some(shape_id.parse().unwrap());
        self
    }

    pub fn read(&mut self, shape_id: &str) -> &mut Self {
        self.read = Some(shape_id.parse().unwrap());
        self
    }

    pub fn update(&mut self, shape_id: &str) -> &mut Self {
        self.update = Some(shape_id.parse().unwrap());
        self
    }

    pub fn delete(&mut self, shape_id: &str) -> &mut Self {
        self.delete = Some(shape_id.parse().unwrap());
        self
    }

    pub fn list(&mut self, shape_id: &str) -> &mut Self {
        self.list = Some(shape_id.parse().unwrap());
        self
    }

    pub fn operation(&mut self, shape_id: &str) -> &mut Self {
        self.operations.push(shape_id.parse().unwrap());
        self
    }
    pub fn operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.operation(shape_id);
        }
        self
    }

    pub fn collection_operation(&mut self, shape_id: &str) -> &mut Self {
        self.collection_operations.push(shape_id.parse().unwrap());
        self
    }
    pub fn collection_operations(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.collection_operation(shape_id);
        }
        self
    }

    pub fn resource(&mut self, shape_id: &str) -> &mut Self {
        self.resources.push(shape_id.parse().unwrap());
        self
    }
    pub fn resources(&mut self, shape_ids: &[&str]) -> &mut Self {
        for shape_id in shape_ids {
            let _ = self.resource(shape_id);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub sensitive);

    add_trait!(pub no_replace);

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

impl ReferenceBuilder {
    /// Construct a new `ShapeKind::Unresolved` builder, with id.
    pub fn new(id: &str) -> Self {
        Self {
            shape_id: id.parse().unwrap(),
            applied_traits: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);
}

// ------------------------------------------------------------------------------------------------

impl From<&mut MemberBuilder> for MemberBuilder {
    fn from(v: &mut MemberBuilder) -> Self {
        <MemberBuilder>::clone(<&mut MemberBuilder>::deref(&v))
    }
}

impl MemberBuilder {
    /// Construct a new member shape builder, with id target
    pub fn new(member_name: &str, target: &str) -> Self {
        Self {
            member_name: member_name.parse().unwrap(),
            applied_traits: Default::default(),
            target: target.parse().unwrap(),
        }
    }

    fn new_unchecked(member_name: &str, target_namespace: &str, target_shape_name: &str) -> Self {
        Self {
            member_name: member_name.parse().unwrap(),
            applied_traits: Default::default(),
            target: ShapeID::new_unchecked(target_namespace, target_shape_name, None),
        }
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Blob`.
    pub fn blob(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Blob")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Boolean`.
    pub fn boolean(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Boolean")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Document`.
    pub fn document(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Document")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::String`.
    pub fn string(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "String")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Byte`.
    pub fn byte(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Byte")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Short`.
    pub fn short(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Short")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Integer`.
    pub fn integer(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Integer")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Long`.
    pub fn long(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Long")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Float`.
    pub fn float(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Float")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Double`.
    pub fn double(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Double")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigInteger`.
    pub fn big_integer(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "BigInteger")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigDecimal`.
    pub fn big_decimal(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "BigDecimal")
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Timestamp`.
    pub fn timestamp(member_name: &str) -> Self {
        Self::new_unchecked(member_name, PRELUDE_NAMESPACE, "Timestamp")
    }

    // --------------------------------------------------------------------------------------------

    pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
        self.applied_traits.push(a_trait);
        self
    }

    add_trait!(pub documentation(text: &str));

    add_trait!(pub external_documentation(map: &[(&str, &str)]));

    add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(pub since(date: &str));

    add_trait!(pub tagged(tags: &[&str]));

    add_trait!(pub unstable);

    add_trait!(pub required);
}
