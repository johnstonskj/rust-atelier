use crate::model::builder::TraitBuilder;
use crate::model::shapes::{Shape, ShapeInner, SimpleType, Trait};
use crate::model::{Annotated, Documented, Identifier};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ShapeBuilder {
    fn as_shape(&self) -> &Shape;
    fn as_shape_mut(&mut self) -> &mut Shape;

    fn doc_comment(&mut self, documentation: &str) -> &mut Self {
        self.as_shape_mut().set_documentation(documentation);
        self
    }

    fn doc_trait(&mut self, documentation: &str) -> &mut Self {
        self.add_trait(TraitBuilder::documentation(documentation).build());
        self
    }

    fn add_trait(&mut self, a_trait: Trait) -> &mut Self {
        self.as_shape_mut().add_trait(a_trait);
        self
    }

    fn external_documentation(&mut self, map: &[(&str, &str)]) -> &mut Self {
        self.add_trait(TraitBuilder::external_documentation(map).build());
        self
    }
    fn deprecated(&mut self, message: Option<&str>, since: Option<&str>) -> &mut Self {
        self.add_trait(TraitBuilder::deprecated(message, since).build());
        self
    }

    fn private(&mut self) -> &mut Self {
        self.add_trait(TraitBuilder::private().build());
        self
    }

    fn since(&mut self, date: &str) -> &mut Self {
        self.add_trait(TraitBuilder::since(date).build());
        self
    }

    fn tagged(&mut self, tags: &[&str]) -> &mut Self {
        self.add_trait(TraitBuilder::tagged(tags).build());
        self
    }

    fn build(&self) -> Shape {
        self.as_shape().clone()
    }
}

#[derive(Clone, Debug)]
pub struct SimpleShapeBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct ListBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct SetBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct MapBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct StructureBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct UnionBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct ServiceBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct OperationBuilder {
    shape: Shape,
}

#[derive(Clone, Debug)]
pub struct ResourceBuilder {
    shape: Shape,
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
macro_rules! simple_constructor {
    ($fn_name:ident, $var_name:ident) => {
        pub fn $fn_name(id: &str) -> Self {
            Self {
                shape: Shape::new(
                    Identifier::from_str(id).unwrap(),
                    ShapeInner::SimpleType(SimpleType::$var_name),
                ),
            }
        }
    };
}

#[doc(hidden)]
macro_rules! default_builder_impl {
    ($struct_name:ident) => {
        impl ShapeBuilder for $struct_name {
            fn as_shape(&self) -> &Shape {
                &self.shape
            }

            fn as_shape_mut(&mut self) -> &mut Shape {
                &mut self.shape
            }
        }
    };
}

#[doc(hidden)]
macro_rules! add_trait {
    ($trait_fn:ident) => {
        pub fn $trait_fn(&mut self) -> &mut Self {
            self.add_trait(TraitBuilder::$trait_fn().build());
            self
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

default_builder_impl!(SimpleShapeBuilder);

impl SimpleShapeBuilder {
    simple_constructor!(blob, Blob);

    simple_constructor!(boolean, Boolean);

    simple_constructor!(document, Document);

    simple_constructor!(string, String);

    simple_constructor!(byte, Byte);

    simple_constructor!(short, Short);

    simple_constructor!(integer, Integer);

    simple_constructor!(long, Long);

    simple_constructor!(float, Float);

    simple_constructor!(double, Double);

    simple_constructor!(big_integer, BigInteger);

    simple_constructor!(big_decimal, BigDecimal);

    simple_constructor!(timestamp, Timestamp);

    add_trait!(boxed);

    add_trait!(sensitive);
}

// ------------------------------------------------------------------------------------------------

default_builder_impl!(ListBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(SetBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(MapBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(StructureBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(UnionBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(ServiceBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(OperationBuilder);

// ------------------------------------------------------------------------------------------------

default_builder_impl!(ResourceBuilder);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
