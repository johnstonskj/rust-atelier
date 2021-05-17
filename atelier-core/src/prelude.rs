/*!
Provides constant valued names from the prelude model described in the Smithy specification.
*/

use std::collections::HashSet;

use crate::error::Result;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

use crate::model::{Identifier, NamespaceID, ShapeID};
use std::str::FromStr;
#[doc(hidden)]
macro_rules! string_const {
    ($name:ident, $value:expr, $comment:expr) => {
        #[doc = $comment]
        pub const $name: &str = $value;
    };
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return the prelude namespace as a `NamespaceID`.
/// 
pub fn prelude_namespace_id() -> &'static NamespaceID {
    &PRELUDE_NAMESPACE_ID
}

///
/// Return a set of `ShapeID`s for all the prelude defined shapes.
/// 
pub fn defined_prelude_shapes() -> &'static HashSet<&'static str> {
    &PRELUDE_SHAPES
}

///
/// Return a set of `ShapeID`s for all the prelude defined traits.
/// 
pub fn defined_prelude_traits() -> &'static HashSet<&'static str> {
    &PRELUDE_TRAITS
}

///
/// Create a ShapeID corresponding to a prelude top-level shape.
///
pub fn prelude_shape_named(name: &str) -> Result<ShapeID> {
    Ok(ShapeID::new(
        NamespaceID::new_unchecked(PRELUDE_NAMESPACE),
        Identifier::from_str(name)?,
        None,
    ))
}

// ------------------------------------------------------------------------------------------------
// Public Names
// ------------------------------------------------------------------------------------------------

string_const!(
    PRELUDE_NAMESPACE,
    "smithy.api",
    "The namespace for the Smithy prelude model."
);

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref PRELUDE_NAMESPACE_ID: NamespaceID = NamespaceID::new_unchecked(PRELUDE_NAMESPACE);
}

lazy_static! {
    static ref PRELUDE_SHAPES: HashSet<&'static str> = [
        SHAPE_STRING,
        SHAPE_BLOB,
        SHAPE_BIGINTEGER,
        SHAPE_BIGDECIMAL,
        SHAPE_TIMESTAMP,
        SHAPE_DOCUMENT,
        SHAPE_BOOLEAN,
        SHAPE_PRIMITIVEBOOLEAN,
        SHAPE_BYTE,
        SHAPE_PRIMITIVEBYTE,
        SHAPE_SHORT,
        SHAPE_PRIMITIVESHORT,
        SHAPE_INTEGER,
        SHAPE_PRIMITIVEINTEGER,
        SHAPE_LONG,
        SHAPE_PRIMITIVELONG,
        SHAPE_FLOAT,
        SHAPE_PRIMITIVEFLOAT,
        SHAPE_DOUBLE,
        SHAPE_PRIMITIVEDOUBLE,
    ]
    .iter()
    .cloned()
    .collect();
}

lazy_static! {
    static ref PRELUDE_TRAITS: HashSet<&'static str> = [
        TRAIT_XMLFLATTENED,
        TRAIT_REFERENCES,
        TRAIT_STRUCTURALLYEXCLUSIVE,
        TRAIT_STREAMING,
        TRAIT_REQUIRESLENGTH,
        TRAIT_UNIQUEITEMS,
        TRAIT_EXAMPLES,
        TRAIT_TIMESTAMPFORMAT,
        TRAIT_HTTPERROR,
        TRAIT_HTTPBASICAUTH,
        TRAIT_DOCUMENTATION,
        TRAIT_HTTPQUERY,
        TRAIT_SUPPRESS,
        TRAIT_SINCE,
        TRAIT_TRAIT,
        TRAIT_HTTPBEARERAUTH,
        TRAIT_HTTPPAYLOAD,
        TRAIT_HTTPAPIKEYAUTH,
        TRAIT_CORS,
        TRAIT_IDREF,
        TRAIT_UNSTABLE,
        TRAIT_HOSTLABEL,
        TRAIT_NONEMPTYSTRING,
        TRAIT_PRIVATE,
        TRAIT_TITLE,
        TRAIT_REQUIRED,
        TRAIT_ENUM,
        TRAIT_IDEMPOTENCYTOKEN,
        TRAIT_TAGS,
        TRAIT_HTTPAPIKEYLOCATIONS,
        TRAIT_OPTIONALAUTH,
        TRAIT_XMLATTRIBUTE,
        TRAIT_XMLNAME,
        TRAIT_HTTPHEADER,
        TRAIT_AUTHDEFINITION,
        TRAIT_RESOURCEIDENTIFIER,
        TRAIT_EXAMPLE,
        TRAIT_MEDIATYPE,
        TRAIT_HTTPCHECKSUMREQUIRED,
        TRAIT_IDEMPOTENT,
        TRAIT_ENDPOINT,
        TRAIT_EVENTHEADER,
        TRAIT_SENSITIVE,
        TRAIT_TRAITSHAPEIDLIST,
        TRAIT_ERROR,
        TRAIT_ENUMDEFINITION,
        TRAIT_PROTOCOLDEFINITION,
        TRAIT_ENUMCONSTANTBODYNAME,
        TRAIT_DEPRECATED,
        TRAIT_HTTPPREFIXHEADERS,
        TRAIT_EVENTPAYLOAD,
        TRAIT_NOREPLACE,
        TRAIT_HTTP,
        TRAIT_EXTERNALDOCUMENTATION,
        TRAIT_JSONNAME,
        TRAIT_AUTH,
        TRAIT_RETRYABLE,
        TRAIT_HTTPDIGESTAUTH,
        TRAIT_RANGE,
        TRAIT_BOX,
        TRAIT_TRAITSHAPEID,
        TRAIT_REFERENCE,
        TRAIT_NONEMPTYSTRINGMAP,
        TRAIT_PAGINATED,
        TRAIT_NONEMPTYSTRINGLIST,
        TRAIT_PATTERN,
        TRAIT_READONLY,
        TRAIT_HTTPLABEL,
        TRAIT_AUTHTRAITREFERENCE,
        TRAIT_XMLNAMESPACE,
        TRAIT_LENGTH,
    ]
    .iter()
    .cloned()
    .collect();
}

// ------------------------------------------------------------------------------------------------

string_const!(
    SHAPE_STRING,
    "String",
    "The identifier for the simple shape `String`"
);

string_const!(
    SHAPE_BLOB,
    "Blob",
    "The identifier for the simple shape `Blob`"
);

string_const!(
    SHAPE_BIGINTEGER,
    "BigInteger",
    "The identifier for the simple shape `BigInteger`"
);

string_const!(
    SHAPE_BIGDECIMAL,
    "BigDecimal",
    "The identifier for the simple shape `BigDecimal`"
);

string_const!(
    SHAPE_TIMESTAMP,
    "Timestamp",
    "The identifier for the simple shape `Timestamp`"
);

string_const!(
    SHAPE_DOCUMENT,
    "Document",
    "The identifier for the simple shape `Document`"
);

string_const!(
    SHAPE_BOOLEAN,
    "Boolean",
    "The identifier for the simple shape `Boolean`"
);

string_const!(
    SHAPE_PRIMITIVEBOOLEAN,
    "PrimitiveBoolean",
    "The identifier for the simple shape `PrimitiveBoolean`"
);

string_const!(
    SHAPE_BYTE,
    "Byte",
    "The identifier for the simple shape `Byte`"
);

string_const!(
    SHAPE_PRIMITIVEBYTE,
    "PrimitiveByte",
    "The identifier for the simple shape `PrimitiveByte`"
);

string_const!(
    SHAPE_SHORT,
    "Short",
    "The identifier for the simple shape `Short`"
);

string_const!(
    SHAPE_PRIMITIVESHORT,
    "PrimitiveShort",
    "The identifier for the simple shape `PrimitiveShort`"
);

string_const!(
    SHAPE_INTEGER,
    "Integer",
    "The identifier for the simple shape `Integer`"
);

string_const!(
    SHAPE_PRIMITIVEINTEGER,
    "PrimitiveInteger",
    "The identifier for the simple shape `PrimitiveInteger`"
);

string_const!(
    SHAPE_LONG,
    "Long",
    "The identifier for the identifier for the simple shape `Long`"
);

string_const!(
    SHAPE_PRIMITIVELONG,
    "PrimitiveLong",
    "The identifier for the identifier for the simple shape `PrimitiveLong`"
);

string_const!(
    SHAPE_FLOAT,
    "Float",
    "The identifier for the simple shape `Float`"
);

string_const!(
    SHAPE_PRIMITIVEFLOAT,
    "PrimitiveFloat",
    "The identifier for the simple shape `PrimitiveFloat`"
);

string_const!(
    SHAPE_DOUBLE,
    "Double",
    "The identifier for the simple shape `Double`"
);

string_const!(
    SHAPE_PRIMITIVEDOUBLE,
    "PrimitiveDouble",
    "The identifier for the simple shape `PrimitiveDouble`"
);

// ------------------------------------------------------------------------------------------------

string_const!(
    TRAIT_XMLFLATTENED,
    "xmlFlattened",
    "The identifier for the structure trait `xmlFlattened`"
);

string_const!(
    TRAIT_REFERENCES,
    "references",
    " The identifier for the list trait `references`"
);

string_const!(
    TRAIT_STRUCTURALLYEXCLUSIVE,
    "StructurallyExclusive",
    " The identifier for the string `StructurallyExclusive`"
);

string_const!(
    TRAIT_STREAMING,
    "streaming",
    " The identifier for the structure trait `streaming`"
);

string_const!(
    TRAIT_REQUIRESLENGTH,
    "requiresLength",
    " The identifier for the structure trait `requiresLength`"
);

string_const!(
    TRAIT_UNIQUEITEMS,
    "uniqueItems",
    " The identifier for the structure trait `uniqueItems`"
);

string_const!(
    TRAIT_EXAMPLES,
    "examples",
    " The identifier for the list trait `examples`"
);

string_const!(
    TRAIT_TIMESTAMPFORMAT,
    "timestampFormat",
    " The identifier for the string trait `timestampFormat`"
);

string_const!(
    TRAIT_HTTPERROR,
    "httpError",
    " The identifier for the integer trait `httpError`"
);

string_const!(
    TRAIT_HTTPBASICAUTH,
    "httpBasicAuth",
    " The identifier for the structure trait `httpBasicAuth`"
);

string_const!(
    TRAIT_DOCUMENTATION,
    "documentation",
    " The identifier for the string trait `documentation`"
);

string_const!(
    TRAIT_HTTPQUERY,
    "httpQuery",
    " The identifier for the string trait `httpQuery`"
);

string_const!(
    TRAIT_SUPPRESS,
    "suppress",
    " The identifier for the list trait `suppress`"
);

string_const!(
    TRAIT_SINCE,
    "since",
    " The identifier for the string trait `since`"
);

string_const!(
    TRAIT_TRAIT,
    "trait",
    " The identifier for the structure trait `trait`"
);

string_const!(
    TRAIT_HTTPBEARERAUTH,
    "httpBearerAuth",
    " The identifier for the trait `httpBearerAuth`"
);

string_const!(
    TRAIT_HTTPPAYLOAD,
    "httpPayload",
    " The identifier for the structure trait `httpPayload`"
);

string_const!(
    TRAIT_HTTPAPIKEYAUTH,
    "httpApiKeyAuth",
    " The identifier for the structure trait `httpApiKeyAuth`"
);

string_const!(
    TRAIT_CORS,
    "cors",
    " The identifier for the structure trait `cors`"
);

string_const!(
    TRAIT_IDREF,
    "idRef",
    " The identifier for the structure trait `idRef`"
);

string_const!(
    TRAIT_UNSTABLE,
    "unstable",
    " The identifier for the structure trait `unstable`"
);

string_const!(
    TRAIT_HOSTLABEL,
    "hostLabel",
    " The identifier for the structure trait `hostLabel`"
);

string_const!(
    TRAIT_NONEMPTYSTRING,
    "NonEmptyString",
    " The identifier for the string `NonEmptyString`"
);

string_const!(
    TRAIT_PRIVATE,
    "private",
    " The identifier for the structure trait `private`"
);

string_const!(
    TRAIT_TITLE,
    "title",
    " The identifier for the string trait `title`"
);

string_const!(
    TRAIT_REQUIRED,
    "required",
    " The identifier for the structure trait `required`"
);

string_const!(
    TRAIT_ENUM,
    "enum",
    " The identifier for the list trait `enum`"
);

string_const!(
    TRAIT_IDEMPOTENCYTOKEN,
    "idempotencyToken",
    " The identifier for the structure trait `idempotencyToken`"
);

string_const!(
    TRAIT_TAGS,
    "tags",
    " The identifier for the list trait `tags`"
);

string_const!(
    TRAIT_HTTPAPIKEYLOCATIONS,
    "HttpApiKeyLocations",
    " The identifier for the string `HttpApiKeyLocations`"
);

string_const!(
    TRAIT_OPTIONALAUTH,
    "optionalAuth",
    " The identifier for the structure trait `optionalAuth`"
);

string_const!(
    TRAIT_XMLATTRIBUTE,
    "xmlAttribute",
    " The identifier for the structure trait `xmlAttribute`"
);

string_const!(
    TRAIT_XMLNAME,
    "xmlName",
    " The identifier for the string trait `xmlName`"
);

string_const!(
    TRAIT_HTTPHEADER,
    "httpHeader",
    " The identifier for the string trait `httpHeader`"
);

string_const!(
    TRAIT_AUTHDEFINITION,
    "authDefinition",
    " The identifier for the structure trait `authDefinition`"
);

string_const!(
    TRAIT_RESOURCEIDENTIFIER,
    "resourceIdentifier",
    " The identifier for the string trait `resourceIdentifier`"
);

string_const!(
    TRAIT_EXAMPLE,
    "Example",
    " The identifier for the structure `Example`"
);

string_const!(
    TRAIT_MEDIATYPE,
    "mediaType",
    " The identifier for the string trait `mediaType`"
);

string_const!(
    TRAIT_HTTPCHECKSUMREQUIRED,
    "httpChecksumRequired",
    " The identifier for the trait `httpChecksumRequired`"
);

string_const!(
    TRAIT_IDEMPOTENT,
    "idempotent",
    " The identifier for the structure trait `idempotent`"
);

string_const!(
    TRAIT_ENDPOINT,
    "endpoint",
    " The identifier for the structure trait `endpoint`"
);

string_const!(
    TRAIT_EVENTHEADER,
    "eventHeader",
    " The identifier for the structure trait `eventHeader`"
);

string_const!(
    TRAIT_SENSITIVE,
    "sensitive",
    " The identifier for the structure trait `sensitive`"
);

string_const!(
    TRAIT_TRAITSHAPEIDLIST,
    "TraitShapeIdList",
    " The identifier for the list `TraitShapeIdList`"
);

string_const!(
    TRAIT_ERROR,
    "error",
    " The identifier for the string trait `error`"
);

string_const!(
    TRAIT_ENUMDEFINITION,
    "EnumDefinition",
    " The identifier for the structure `EnumDefinition`"
);

string_const!(
    TRAIT_PROTOCOLDEFINITION,
    "protocolDefinition",
    " The identifier for the structure trait `protocolDefinition`"
);

string_const!(
    TRAIT_ENUMCONSTANTBODYNAME,
    "EnumConstantBodyName",
    " The identifier for the string `EnumConstantBodyName`"
);

string_const!(
    TRAIT_DEPRECATED,
    "deprecated",
    " The identifier for the structure trait `deprecated`"
);

string_const!(
    TRAIT_HTTPPREFIXHEADERS,
    "httpPrefixHeaders",
    " The identifier for the string trait `httpPrefixHeaders`"
);

string_const!(
    TRAIT_EVENTPAYLOAD,
    "eventPayload",
    " The identifier for the structure trait `eventPayload`"
);

string_const!(
    TRAIT_NOREPLACE,
    "noReplace",
    " The identifier for the structure trait `noReplace`"
);

string_const!(
    TRAIT_HTTP,
    "http",
    " The identifier for the structure trait `http`"
);

string_const!(
    TRAIT_EXTERNALDOCUMENTATION,
    "externalDocumentation",
    " The identifier for the map trait `externalDocumentation`"
);

string_const!(
    TRAIT_JSONNAME,
    "jsonName",
    " The identifier for the string trait `jsonName`"
);

string_const!(
    TRAIT_AUTH,
    "auth",
    " The identifier for the list trait `auth`"
);

string_const!(
    TRAIT_RETRYABLE,
    "retryable",
    " The identifier for the structure trait `retryable`"
);

string_const!(
    TRAIT_HTTPDIGESTAUTH,
    "httpDigestAuth",
    " The identifier for the structure trait `httpDigestAuth`"
);

string_const!(
    TRAIT_RANGE,
    "range",
    " The identifier for the structure trait `range`"
);

string_const!(
    TRAIT_BOX,
    "box",
    " The identifier for the structure trait `box`"
);

string_const!(
    TRAIT_TRAITSHAPEID,
    "TraitShapeId",
    " The identifier for the string `TraitShapeId`"
);

string_const!(
    TRAIT_REFERENCE,
    "Reference",
    " The identifier for the structure `Reference`"
);

string_const!(
    TRAIT_NONEMPTYSTRINGMAP,
    "NonEmptyStringMap",
    " The identifier for the map `NonEmptyStringMap`"
);

string_const!(
    TRAIT_PAGINATED,
    "paginated",
    " The identifier for the structure trait `paginated`"
);

string_const!(
    TRAIT_NONEMPTYSTRINGLIST,
    "NonEmptyStringList",
    " The identifier for the list `NonEmptyStringList`"
);

string_const!(
    TRAIT_PATTERN,
    "pattern",
    " The identifier for the string trait `pattern`"
);

string_const!(
    TRAIT_READONLY,
    "readonly",
    " The identifier for the structure trait `readonly`"
);

string_const!(
    TRAIT_HTTPLABEL,
    "httpLabel",
    " The identifier for the structure trait `httpLabel`"
);

string_const!(
    TRAIT_AUTHTRAITREFERENCE,
    "AuthTraitReference",
    " The identifier for the string `AuthTraitReference`"
);

string_const!(
    TRAIT_XMLNAMESPACE,
    "xmlNamespace",
    " The identifier for the structure trait `xmlNamespace`"
);

string_const!(
    TRAIT_LENGTH,
    "length",
    " The identifier for the structure trait `length`"
);
