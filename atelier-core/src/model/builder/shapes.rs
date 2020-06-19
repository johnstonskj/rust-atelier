use crate::error::ErrorSource;
use crate::model::builder::TraitBuilder;
use crate::model::shapes::{ListOrSet, Map, Member, Shape, ShapeInner, SimpleType, Trait};
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
        #[derive(Clone, Debug)]
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
macro_rules! add_trait {
    (pub $trait_fn:ident) => {
        pub fn $trait_fn(&mut self) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn().build());
            self
        }
    };
    ($trait_fn:ident) => {
        fn $trait_fn(&mut self) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn().build());
            self
        }
    };
    (pub $trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        pub fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn($( $i ),*).build());
            self
        }
    };
    ($trait_fn:ident ( $( $i:ident : $t:ty ),* ) ) => {
        fn $trait_fn(&mut self, $( $i: $t ),* ) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn($( $i ),*).build());
            self
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
        self.add_trait(TraitBuilder::documentation(documentation).build());
        self
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
    pub fn new(id: &str, member_id: &str) -> Self {
        Self {
            inner: Shape::new(
                Identifier::from_str(id).unwrap(),
                ShapeInner::List(ListOrSet::new(ShapeID::from_str(member_id).unwrap())),
            ),
        }
    }

    add_trait!(pub sensitive);

    add_trait!(pub unique_items);
}

// ------------------------------------------------------------------------------------------------

impl SetBuilder {
    pub fn new(id: &str, member_id: &str) -> Self {
        Self {
            inner: Shape::new(
                Identifier::from_str(id).unwrap(),
                ShapeInner::Set(ListOrSet::new(ShapeID::from_str(member_id).unwrap())),
            ),
        }
    }

    add_trait!(pub sensitive);
}

// ------------------------------------------------------------------------------------------------

impl MapBuilder {
    pub fn new(id: &str, key_id: &str, value_id: &str) -> Self {
        Self {
            inner: Shape::new(
                Identifier::from_str(id).unwrap(),
                ShapeInner::Map(Map::new(
                    ShapeID::from_str(key_id).unwrap(),
                    ShapeID::from_str(value_id).unwrap(),
                )),
            ),
        }
    }

    add_trait!(pub error(src: ErrorSource));
}

// ------------------------------------------------------------------------------------------------

impl StructureBuilder {
    pub fn new(_id: &str) -> Self {
        unimplemented!()
    }

    add_trait!(pub sensitive);
}

// ------------------------------------------------------------------------------------------------

impl UnionBuilder {
    pub fn new(_id: &str) -> Self {
        unimplemented!()
    }

    add_trait!(pub streaming);
}

// ------------------------------------------------------------------------------------------------

impl ServiceBuilder {
    pub fn new(_id: &str) -> Self {
        unimplemented!()
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        if let ShapeInner::Service(service) = &mut self.inner.inner_mut() {
            service.set_version(version)
        }
        self
    }

    pub fn operation(&mut self, id: &str) -> &mut Self {
        if let ShapeInner::Service(service) = self.inner.inner_mut() {
            service.add_operation(ShapeID::from_str(id).unwrap())
        }
        self
    }

    pub fn operations(&mut self, ids: &[&str]) -> &mut Self {
        if let ShapeInner::Service(service) = self.inner.inner_mut() {
            service.append_operations(
                &ids.iter()
                    .map(|s| ShapeID::from_str(s).unwrap())
                    .collect::<Vec<ShapeID>>(),
            )
        }
        self
    }

    pub fn resource(&mut self, id: &str) -> &mut Self {
        if let ShapeInner::Service(service) = &mut self.inner.inner_mut() {
            service.add_resource(ShapeID::from_str(id).unwrap())
        }
        self
    }

    pub fn resources(&mut self, ids: &[&str]) -> &mut Self {
        if let ShapeInner::Service(service) = self.inner.inner_mut() {
            service.append_resources(
                &ids.iter()
                    .map(|s| ShapeID::from_str(s).unwrap())
                    .collect::<Vec<ShapeID>>(),
            )
        }
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl OperationBuilder {
    pub fn new(_id: &str) -> Self {
        unimplemented!()
    }

    pub fn input(&mut self, id: &str) -> &mut Self {
        if let ShapeInner::Operation(operation) = self.inner.inner_mut() {
            operation.set_input(ShapeID::from_str(id).unwrap())
        }
        self
    }

    pub fn output(&mut self, id: &str) -> &mut Self {
        if let ShapeInner::Operation(operation) = self.inner.inner_mut() {
            operation.set_output(ShapeID::from_str(id).unwrap())
        }
        self
    }

    pub fn error(&mut self, id: &str) -> &mut Self {
        if let ShapeInner::Operation(operation) = self.inner.inner_mut() {
            operation.add_error(ShapeID::from_str(id).unwrap())
        }
        self
    }

    pub fn errors(&mut self, ids: &[&str]) -> &mut Self {
        if let ShapeInner::Operation(operation) = self.inner.inner_mut() {
            operation.append_errors(
                &ids.iter()
                    .map(|s| ShapeID::from_str(s).unwrap())
                    .collect::<Vec<ShapeID>>(),
            )
        }
        self
    }

    add_trait!(pub idempotent);

    add_trait!(pub readonly);
}

// ------------------------------------------------------------------------------------------------

impl ResourceBuilder {
    pub fn new(_id: &str) -> Self {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl MemberBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            inner: Member::new(Identifier::from_str(id).unwrap()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
