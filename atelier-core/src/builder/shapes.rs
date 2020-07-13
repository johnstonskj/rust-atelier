use crate::builder::TraitBuilder;
use crate::error::ErrorSource;
use crate::model::shapes::{
    AppliedTrait, ListOrSet, Map, Member, Service, Shape, ShapeKind, Simple,
};
use crate::model::values::Value;
use crate::model::{Identifier, ShapeID};
use crate::prelude::PRELUDE_NAMESPACE;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! simple_shape_constructor {
    ($fn_name:ident, $var_name:ident) => {
        ///Construct a new shape builder.
        pub fn $fn_name(id: &str) -> Self {
            Self {
                inner: Shape::new(
                    ShapeID::from_str(id).unwrap(),
                    ShapeKind::Simple(Simple::$var_name),
                ),
            }
        }
    };
}

#[doc(hidden)]
macro_rules! shape_builder {
    ($struct_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $struct_name {
            inner: Shape,
        }

        impl From<&mut $struct_name> for Shape {
            fn from(builder: &mut $struct_name) -> Self {
                builder.inner.clone()
            }
        }

        impl From<$struct_name> for Shape {
            fn from(builder: $struct_name) -> Self {
                builder.inner.clone()
            }
        }

        impl $struct_name {
            pub fn documentation(&mut self, documentation: &str) -> &mut Self {
                self.apply_trait(TraitBuilder::documentation(documentation).into())
            }

            pub fn apply_trait(&mut self, a_trait: AppliedTrait) -> &mut Self {
                self.inner.apply_trait(a_trait);
                self
            }

            add_trait!(pub external_documentation(map: &[(&str, &str)]));

            add_trait!(pub deprecated(message: Option<&str>, since: Option<&str>));

            add_trait!(pub since(date: &str));

            add_trait!(pub tagged(tags: &[&str]));

            add_trait!(pub unstable);
        }
    };
}

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
                let _ = inner.add_member(Box::new(MemberBuilder::new(id, id_ref).into()));
            }
            self
        }

        /// Add `member` to this shape.
        pub fn add_member(&mut self, member: Member) -> &mut Self {
            let id = self.inner.id().to_member(member.name().clone());
            if let ShapeKind::$shape_variant(inner) = self.inner.body_mut() {
                let _ = inner.add_a_member(Box::new(Shape::new(id, ShapeKind::Member(member))));
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

#[doc(hidden)]
macro_rules! member_constructor {
    ($fn_name:ident, $id_ref:expr) => {
        /// Constructs a new member with a reference to the type given by `id`.
        pub fn $fn_name(id: &str) -> Self {
            Self::new(id, &format!("{}#{}", PRELUDE_NAMESPACE, $id_ref))
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

shape_builder!(
    SimpleShapeBuilder,
    "Builder for `ShapeBody::Simple` shapes."
);

shape_builder!(ListBuilder, "Builder for `ShapeBody::List` shapes.");

shape_builder!(SetBuilder, "Builder for `ShapeBody::Set` shapes.");

shape_builder!(MapBuilder, "Builder for `ShapeBody::Map` shapes.");

shape_builder!(
    StructureBuilder,
    "Builder for `ShapeBody::Structure` shapes."
);

shape_builder!(UnionBuilder, "Builder for `ShapeBody::Union` shapes.");

shape_builder!(ServiceBuilder, "Builder for `ShapeBody::Service` shapes.");

shape_builder!(
    OperationBuilder,
    "Builder for `ShapeBody::Operation` shapes."
);

shape_builder!(ResourceBuilder, "Builder for `ShapeBody::Resource` shapes.");

shape_builder!(MemberBuilder, "Builder for `Member` objects within shapes.");

// ------------------------------------------------------------------------------------------------
// Additional Implementations
// ------------------------------------------------------------------------------------------------

impl SimpleShapeBuilder {
    simple_shape_constructor!(blob, Blob);

    simple_shape_constructor!(boolean, Boolean);

    simple_shape_constructor!(document, Document);

    simple_shape_constructor!(string, String);

    simple_shape_constructor!(byte, Byte);

    simple_shape_constructor!(short, Short);

    simple_shape_constructor!(integer, Integer);

    simple_shape_constructor!(long, Long);

    simple_shape_constructor!(float, Float);

    simple_shape_constructor!(double, Double);

    simple_shape_constructor!(big_integer, BigInteger);

    simple_shape_constructor!(big_decimal, BigDecimal);

    simple_shape_constructor!(timestamp, Timestamp);

    add_trait!(pub boxed);

    add_trait!(pub sensitive);

    add_trait!(pub streaming);
}

// ------------------------------------------------------------------------------------------------

impl ListBuilder {
    shape_constructor! {
        List(member_id: &str),
        ListOrSet::new_list(ShapeID::from_str(member_id).unwrap())
    }

    add_trait!(pub sensitive);

    add_trait!(pub unique_items);
}

// ------------------------------------------------------------------------------------------------

impl SetBuilder {
    shape_constructor! {
        Set(member_id: &str),
        ListOrSet::new_set(ShapeID::from_str(member_id).unwrap())
    }

    add_trait!(pub sensitive);
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

impl MemberBuilder {
    /// Construct a new member shape builder, with id target
    pub fn new(id: &str, target: &str) -> Self {
        Self {
            inner: Shape::new(
                ShapeID::from_str(id).unwrap(),
                ShapeKind::Member(Member::new(
                    Identifier::from_str(id).unwrap(),
                    ShapeID::from_str(target).unwrap(),
                )),
            ),
        }
    }

    pub fn refers_to(&mut self, target: &str) -> &mut Self {
        if let ShapeKind::Member(member) = self.inner.body_mut() {
            member.set_target(ShapeID::from_str(target).unwrap());
        }
        self
    }

    member_constructor! { blob, "Blob" }

    member_constructor! { boolean, "Boolean" }

    member_constructor! { document, "Document" }

    member_constructor! { string, "String" }

    member_constructor! { byte, "Byte" }

    member_constructor! { short, "Short" }

    member_constructor! { integer, "Integer" }

    member_constructor! { long, "Long" }

    member_constructor! { float, "Float" }

    member_constructor! { double, "Double" }

    member_constructor! { big_integer, "BigInteger" }

    member_constructor! { big_decimal, "BigDecimal" }

    member_constructor! { timestamp, "Timestamp" }

    add_trait!(pub required);
}
