use crate::error::ErrorSource;
use crate::model::builder::TraitBuilder;
use crate::model::shapes::{ListOrSet, Map, Member, Shape, ShapeInner, SimpleType, Trait, Valued};
use crate::model::values::NodeValue;
use crate::model::{Annotated, Documented, Identifier, ShapeID};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! simple_shape_constructor {
    ($fn_name:ident, $var_name:ident) => {
        pub fn $fn_name(id: &str) -> Self {
            Self {
                inner: Shape::new(
                    Identifier::from_str(id).unwrap(),
                    ShapeInner::SimpleType(SimpleType::$var_name),
                ),
            }
        }
    };
}

#[doc(hidden)]
macro_rules! concrete_builder {
    ($struct_name:ident, $inner_type:ty) => {
        #[derive(Debug)]
        pub struct $struct_name {
            inner: $inner_type,
        }
        impl Builder<$inner_type> for $struct_name {
            fn inner(&self) -> &$inner_type {
                &self.inner
            }

            fn inner_mut(&mut self) -> &mut $inner_type {
                &mut self.inner
            }
        }
    };
}

#[doc(hidden)]
macro_rules! shape_constructor {
    ($shape_variant:ident ( $( $i:ident : $t:ty ),* ), $init_expr:expr) => {
        pub fn new(id: &str, $( $i: $t ),*) -> Self {
            Self {
                inner: Shape::new(
                    Identifier::from_str(id).unwrap(),
                    ShapeInner::$shape_variant($init_expr),
                ),
            }
        }
    };
    ($shape_variant:ident, $init_expr:expr) => {
        pub fn new(id: &str) -> Self {
            Self {
                inner: Shape::new(
                    Identifier::from_str(id).unwrap(),
                    ShapeInner::$shape_variant($init_expr),
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
        pub fn $fn_name(&mut self, id: &str) -> &mut Self {
            self.member(id, $id_ref)
        }
    };
}

#[doc(hidden)]
macro_rules! structured_members {
    ($shape_variant:ident) => {
        pub fn member(&mut self, id: &str, id_ref: &str) -> &mut Self {
            if let ShapeInner::$shape_variant(inner) = self.inner.inner_mut() {
                inner.add_member(MemberBuilder::new(id).refers_to(id_ref).build())
            }
            self
        }

        pub fn add_member(&mut self, member: Member) -> &mut Self {
            if let ShapeInner::$shape_variant(inner) = self.inner.inner_mut() {
                inner.add_member(member)
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

        pub fn $plural(&mut self, ids: &[&str]) -> &mut Self {
            if let ShapeInner::$shape_variant(inner) = self.inner.inner_mut() {
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
        pub fn $fn_name(&mut self, id: &str) -> &mut Self {
            if let ShapeInner::$shape_variant(inner) = &mut self.inner.inner_mut() {
                inner.$setter(ShapeID::from_str(id).unwrap())
            }
            self
        }
    };
}

#[doc(hidden)]
macro_rules! add_trait {
    (pub $trait_fn:ident) => {
        pub fn $trait_fn(&mut self) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn().build())
        }
    };
    ($trait_fn:ident) => {
        fn $trait_fn(&mut self) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn().build())
        }
    };
    (pub $trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        pub fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn($( $i ),*).build())
        }
    };
    ($trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn($( $i ),*).build())
        }
    };
}

#[doc(hidden)]
macro_rules! member_constructor {
    ($fn_name:ident, $id_ref:expr) => {
        pub fn $fn_name(id: &str) -> Self {
            Self::reference(id, $id_ref)
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Builder<T>
where
    T: Clone + Documented + Annotated,
{
    fn inner(&self) -> &T;
    fn inner_mut(&mut self) -> &mut T;

    fn doc_comment(&mut self, documentation: &str) -> &mut Self {
        self.inner_mut().set_documentation(documentation);
        self
    }

    fn doc_trait(&mut self, documentation: &str) -> &mut Self {
        self.add_trait(TraitBuilder::documentation(documentation).build())
    }

    fn add_trait(&mut self, a_trait: Trait) -> &mut Self {
        self.inner_mut().add_trait(a_trait);
        self
    }

    add_trait!(external_documentation(map: &[(&str, &str)]));

    add_trait!(deprecated(message: Option<&str>, since: Option<&str>));

    add_trait!(since(date: &str));

    add_trait!(tagged(tags: &[&str]));

    add_trait!(unstable);

    fn build(&self) -> T {
        self.inner().clone()
    }
}

concrete_builder!(SimpleShapeBuilder, Shape);

concrete_builder!(ListBuilder, Shape);

concrete_builder!(SetBuilder, Shape);

concrete_builder!(MapBuilder, Shape);

concrete_builder!(StructureBuilder, Shape);

concrete_builder!(UnionBuilder, Shape);

concrete_builder!(ServiceBuilder, Shape);

concrete_builder!(OperationBuilder, Shape);

concrete_builder!(ResourceBuilder, Shape);

concrete_builder!(MemberBuilder, Member);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
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
        ListOrSet::new(ShapeID::from_str(member_id).unwrap())
    }

    add_trait!(pub sensitive);

    add_trait!(pub unique_items);
}

// ------------------------------------------------------------------------------------------------

impl SetBuilder {
    shape_constructor! {
        Set(member_id: &str),
        ListOrSet::new(ShapeID::from_str(member_id).unwrap())
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
    shape_constructor! { Service }

    pub fn version(&mut self, version: &str) -> &mut Self {
        if let ShapeInner::Service(inner) = &mut self.inner.inner_mut() {
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

    pub fn identifier(&mut self, id: &str, shape: &str) -> &mut Self {
        if let ShapeInner::Resource(inner) = &mut self.inner.inner_mut() {
            inner.add_identifier(
                Identifier::from_str(id).unwrap(),
                ShapeID::from_str(shape).unwrap(),
            )
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
    pub fn new(id: &str) -> Self {
        Self {
            inner: Member::new(Identifier::from_str(id).unwrap()),
        }
    }

    pub fn reference(id: &str, id_ref: &str) -> Self {
        let mut new = Self::new(id);
        let _ = new.refers_to(id_ref);
        new
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

    pub fn value(&mut self, value: NodeValue) -> &mut Self {
        self.inner.set_value(value);
        self
    }

    pub fn refers_to(&mut self, ref_id: &str) -> &mut Self {
        self.inner
            .set_value(NodeValue::ShapeID(ShapeID::from_str(ref_id).unwrap()));
        self
    }

    add_trait!(pub required);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
