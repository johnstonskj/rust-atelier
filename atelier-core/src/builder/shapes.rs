use crate::builder::{PartialShapeID, TraitBuilder};
use crate::error::ErrorSource;
use crate::model::shapes::{
    AppliedTrait, ListOrSet, Map, Member, Service, Shape, ShapeKind, Simple,
};
use crate::model::values::{Value, ValueMap};
use crate::model::ShapeID;
use crate::prelude::PRELUDE_NAMESPACE;
use crate::syntax::MEMBER_MEMBER;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! shape_constructor {
    ($shape_variant:ident ( $( $i:ident : $t:ty ),* ), $init_expr:expr) => {
        #[doc = "Construct a new shape builder, with id and required values."]
        pub fn new(id: &str, $( $i: $t ),*) -> Self {
            Self {
                inner: Shape::new(
                    ShapeID::from_str(id).unwrap(),
                    ShapeKind::$shape_variant($init_expr),
                ),
            }
        }
    };
    ($shape_variant:ident, $init_expr:expr) => {
        #[doc = "Construct a new shape builder, with id."]
        pub fn new(id: &str) -> Self {
            Self {
                inner: Shape::new(
                    ShapeID::from_str(id).unwrap(),
                    ShapeKind::$shape_variant($init_expr),
                ),
            }
        }
    };
    ($shape_variant:ident) => {
        shape_constructor! { $shape_variant, Default::default() }
    };
}

#[doc(hidden)]
macro_rules! structured_member {
    ($fn_name:ident, $id_ref:expr) => {
        /// Add a member named `id`, with a reference to the corresponding simple shape, to this shape.
        pub fn $fn_name(&mut self, id: &str) -> &mut Self {
            self.member(id, $id_ref)
        }
    };
}

#[doc(hidden)]
macro_rules! structured_members {
    ($shape_variant:ident) => {
        /// Add a member named `id`, as a reference to the shape `id_ref` to this shape.
        pub fn member(&mut self, id: &str, id_ref: &str) -> &mut Self {
            if let ShapeKind::$shape_variant(inner) = self.inner.body_mut() {
                let _ = inner.add_a_member(Box::new(MemberBuilder::new(id, id_ref).into()));
            }
            self
        }

        /// Add `member` to this shape.
        pub fn add_member(&mut self, member: Shape) -> &mut Self {
            if let ShapeKind::$shape_variant(inner) = self.inner.body_mut() {
                let _ = inner.add_a_member(Box::new(member));
            }
            self
        }

        structured_member! { blob, "Blob" }

        structured_member! { boolean, "Boolean" }

        structured_member! { document, "Document" }

        structured_member! { string, "String" }

        structured_member! { byte, "Byte" }

        structured_member! { short, "Short" }

        structured_member! { integer, "Integer" }

        structured_member! { long, "Long" }

        structured_member! { float, "Float" }

        structured_member! { double, "Double" }

        structured_member! { big_integer, "BigInteger" }

        structured_member! { big_decimal, "BigDecimal" }

        structured_member! { timestamp, "Timestamp" }
    };
}

#[doc(hidden)]
macro_rules! shape_member {
    ($fn_name:ident, $shape_variant:ident, $setter:ident, $plural:ident, $appender:ident) => {
        shape_member! { $fn_name, $shape_variant, $setter }

        /// Append the shapes referenced by `ids` to the named member value.
        pub fn $plural(&mut self, ids: &[&str]) -> &mut Self {
            if let ShapeKind::$shape_variant(inner) = self.inner.body_mut() {
                inner.$appender(
                    &ids.iter()
                        .map(|s| ShapeID::from_str(s).unwrap())
                        .collect::<Vec<ShapeID>>(),
                )
            }
            self
        }
    };
    ($fn_name:ident, $shape_variant:ident, $setter:ident) => {
        /// Add the named member value to the shape referenced by `id`.
        pub fn $fn_name(&mut self, id: &str) -> &mut Self {
            if let ShapeKind::$shape_variant(inner) = &mut self.inner.body_mut() {
                inner.$setter(ShapeID::from_str(id).unwrap())
            }
            self
        }
    };
}

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
#[derive(Debug)]
pub struct SimpleShapeBuilder {
    pub(crate) id: PartialShapeID,
    pub(crate) applied_traits: Vec<AppliedTrait>,
    pub(crate) simple_shape: Simple,
}

/// Builder for `ShapeKind::List` shapes.
#[derive(Debug)]
pub struct ListBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    member: MemberBuilder,
}

/// Builder for `ShapeKind::Set` shapes.
#[derive(Debug)]
pub struct SetBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    member: MemberBuilder,
}

/// Builder for `ShapeKind::Map` shapes.
#[derive(Debug)]
pub struct MapBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    key: MemberBuilder,
    value: MemberBuilder,
}

/// Builder for `ShapeKind::Structure` shapes.
#[derive(Debug)]
pub struct StructureBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    members: Vec<MemberBuilder>,
}

/// Builder for `ShapeKind::Union` shapes.
#[derive(Debug)]
pub struct UnionBuilder {
    id: ShapeID,
    applied_traits: Vec<AppliedTrait>,
    members: Vec<MemberBuilder>,
}

/// Builder for `ShapeKind::Service` shapes.
#[derive(Debug)]
pub struct ServiceBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    version: String,
    operations: Vec<PartialShapeID>,
    resources: Vec<PartialShapeID>,
}

/// Builder for `ShapeKind::Operation` shapes.
#[derive(Debug)]
pub struct OperationBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    input: Option<PartialShapeID>,
    output: Option<PartialShapeID>,
    errors: Vec<PartialShapeID>,
}

/// Builder for `ShapeKind::Resource` shapes.
#[derive(Debug)]
pub struct ResourceBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    identifiers: ValueMap,
    create: Option<PartialShapeID>,
    put: Option<PartialShapeID>,
    read: Option<PartialShapeID>,
    update: Option<PartialShapeID>,
    delete: Option<PartialShapeID>,
    list: Option<PartialShapeID>,
    operations: Vec<PartialShapeID>,
    collection_operations: Vec<PartialShapeID>,
    resources: Vec<PartialShapeID>,
}

/// Builder for `ShapeKind::Member` shapes.
#[derive(Debug)]
pub struct MemberBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
    target: ShapeID,
}

/// Builder for `ShapeKind::Unresolved` shapes.
#[derive(Debug)]
pub struct ReferenceBuilder {
    id: PartialShapeID,
    applied_traits: Vec<AppliedTrait>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SimpleShapeBuilder {
    ///Construct a new simple shape builder.
    pub fn new(id: &str, simple_shape: Simple) -> Self {
        Self {
            id: PartialShapeID::from_str(id).unwrap(),
            applied_traits: Default::default(),
            simple_shape,
        }
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn blob(id: &str) -> Self {
        Self::new(id, Simple::Blob)
    }

    ///Construct a new simple shape builder for Simple::Boolean.
    pub fn boolean(id: &str) -> Self {
        Self::new(id, Simple::Boolean)
    }

    ///Construct a new simple shape builder for Simple::Document.
    pub fn document(id: &str) -> Self {
        Self::new(id, Simple::Document)
    }

    ///Construct a new simple shape builder for Simple::String.
    pub fn string(id: &str) -> Self {
        Self::new(id, Simple::String)
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn byte(id: &str) -> Self {
        Self::new(id, Simple::Byte)
    }

    ///Construct a new simple shape builder for Simple::Short.
    pub fn short(id: &str) -> Self {
        Self::new(id, Simple::Short)
    }

    ///Construct a new simple shape builder for Simple::Integer.
    pub fn integer(id: &str) -> Self {
        Self::new(id, Simple::Integer)
    }

    ///Construct a new simple shape builder for Simple::Long.
    pub fn long(id: &str) -> Self {
        Self::new(id, Simple::Long)
    }

    ///Construct a new simple shape builder for Simple::Float.
    pub fn float(id: &str) -> Self {
        Self::new(id, Simple::Float)
    }

    ///Construct a new simple shape builder for Simple::Double.
    pub fn double(id: &str) -> Self {
        Self::new(id, Simple::Double)
    }

    ///Construct a new simple shape builder for Simple::Blob.
    pub fn big_integer(id: &str) -> Self {
        Self::new(id, Simple::BigInteger)
    }

    ///Construct a new simple shape builder for Simple::BigDecimal.
    pub fn big_decimal(id: &str) -> Self {
        Self::new(id, Simple::BigDecimal)
    }

    ///Construct a new simple shape builder for Simple::Timestamp.
    pub fn timestamp(id: &str) -> Self {
        Self::new(id, Simple::Timestamp)
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
    ///Construct a new list shape builder.
    pub fn new(id: &str, member_target: &str) -> Self {
        let shape_id = ShapeID::from_str(id).unwrap();
        Self {
            id: shape_id.clone(),
            applied_traits: Default::default(),
            member: Shape::new(
                shape_id.make_member(MEMBER_MEMBER.parse().unwrap()),
                ShapeKind::List(ListOrSet::new_list(member_target.parse().unwrap())),
            ),
        }
    }

    pub fn target(&mut self, member: Shape) {
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

impl SetBuilder {
    ///Construct a new set shape builder.
    pub fn new(id: &str, member_target: &str) -> Self {
        let shape_id = ShapeID::from_str(id).unwrap();
        Self {
            id: shape_id.clone(),
            applied_traits: Default::default(),
            member: Shape::new(
                shape_id.make_member(MEMBER_MEMBER.parse().unwrap()),
                ShapeKind::List(ListOrSet::new_list(member_target.parse().unwrap())),
            ),
        }
    }

    pub fn target(&mut self, member: Shape) {
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
    shape_constructor! {
        Map(key_id: &str, value_id: &str),
        Map::new(
                    ShapeID::from_str(key_id).unwrap(),
                    ShapeID::from_str(value_id).unwrap(),
                )
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureBuilder {
    shape_constructor! { Structure }

    structured_members! { Structure }

    add_trait!(pub error(src: ErrorSource));

    add_trait!(pub sensitive);
}

// ------------------------------------------------------------------------------------------------

impl UnionBuilder {
    shape_constructor! { Union }

    structured_members! { Union }

    add_trait!(pub streaming);
}

// ------------------------------------------------------------------------------------------------

impl ServiceBuilder {
    shape_constructor! { Service (version: &str), Service::new(version) }

    /// Set the version of this service.
    pub fn version(&mut self, version: &str) -> &mut Self {
        if let ShapeKind::Service(inner) = &mut self.inner.body_mut() {
            inner.set_version(version)
        }
        self
    }

    shape_member! { operation, Service, add_operation, operations, append_operations }

    shape_member! { resource, Service, add_resource, resources, append_resources }

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

impl OperationBuilder {
    shape_constructor! { Operation }

    shape_member! { input, Operation, set_input }

    shape_member! { output, Operation, set_output }

    shape_member! { error, Operation, add_error, errors, append_errors }

    add_trait!(pub idempotent);

    add_trait!(pub paginated(
        input_token: Option<&str>,
        output_token: Option<&str>,
        items: Option<&str>,
        page_size: Option<&str>));

    add_trait!(pub readonly);
}

// ------------------------------------------------------------------------------------------------

impl ResourceBuilder {
    shape_constructor! { Resource }

    /// Add an identifier to this resource; this represents the local member name and shape
    /// identifier for the member's type.
    pub fn identifier(&mut self, id: &str, shape: &str) -> &mut Self {
        if let ShapeKind::Resource(inner) = &mut self.inner.body_mut() {
            let _ = inner.add_identifier(id.to_string(), Value::String(shape.to_string()));
        }
        self
    }

    shape_member! { create, Resource, set_create }

    shape_member! { put, Resource, set_put }

    shape_member! { read, Resource, set_read }

    shape_member! { update, Resource, set_update }

    shape_member! { delete, Resource, set_delete }

    shape_member! { list, Resource, set_list }

    shape_member! { operation, Resource, add_operation, operations, append_operations }

    shape_member! { collection_operation, Resource, add_collection_operation, collection_operations, append_collection_operations }

    shape_member! { resource, Resource, add_resource, resources, append_resources }

    add_trait!(pub no_replace);

    add_trait!(pub title(title: &str));
}

// ------------------------------------------------------------------------------------------------

impl From<&mut MemberBuilder> for Shape {
    fn from(builder: &mut MemberBuilder) -> Self {
        let member = Member::new(builder.target.clone());
        let mut shape = Shape::new(builder.id.clone(), ShapeKind::Member(member));
        for a_trait in builder.applied_traits.into_iter() {
            shape.apply_trait(a_trait)
        }
        shape
    }
}

impl MemberBuilder {
    /// Construct a new member shape builder, with id target
    pub fn new(id: &str, target: &str) -> Self {
        Self {
            id: id.parse().unwrap(),
            applied_traits: Default::default(),
            target: target.parse().unwrap(),
        }
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Blob`.
    pub fn blob(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Blob"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Boolean`.
    pub fn boolean(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Boolean"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Document`.
    pub fn document(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Document"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::String`.
    pub fn string(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "String"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Byte`.
    pub fn byte(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Byte"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Short`.
    pub fn short(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Short"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Integer`.
    pub fn integer(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Integer"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Long`.
    pub fn long(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Long"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Float`.
    pub fn float(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Float"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Double`.
    pub fn double(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Double"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigInteger`.
    pub fn big_integer(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "BigInteger"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::BigDecimal`.
    pub fn big_decimal(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "BigDecimal"))
    }

    /// Constructs a new member with a target `PRELUDE_NAMESPACE::Timestamp`.
    pub fn timestamp(id: &str) -> Self {
        Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, "Timestamp"))
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

// ------------------------------------------------------------------------------------------------

impl From<&mut ReferenceBuilder> for Shape {
    fn from(builder: &mut ReferenceBuilder) -> Self {
        let mut shape = Shape::new(builder.id.clone(), ShapeKind::Unresolved);
        for a_trait in builder.applied_traits.into_iter() {
            shape.apply_trait(a_trait)
        }
        shape
    }
}

impl ReferenceBuilder {
    /// Construct a new `ShapeKind::Unresolved` builder, with id.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.parse().unwrap(),
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
