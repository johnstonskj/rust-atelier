use crate::model::{Identifier, NamespaceID, ShapeID};
use crate::prelude::*;
use crate::Version;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref PRELUDE_IDENTIFIERS: HashMap<Version, HashSet<ShapeID>> =
        make_prelude_model_all_shape_ids();
}

///
/// Return a list of shape IDs defined in the standard prelude for `version` of the Smithy
/// specification.
///
pub(crate) fn prelude_model_shape_ids(version: &Version) -> &HashSet<ShapeID> {
    PRELUDE_IDENTIFIERS.get(version).unwrap()
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

fn make_prelude_model_all_shape_ids() -> HashMap<Version, HashSet<ShapeID>> {
    let namespace = NamespaceID::from_str(PRELUDE_NAMESPACE).unwrap();
    let names = [
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
    .map(|shape_name| namespace.make_shape(Identifier::from_str(shape_name).unwrap()))
    .collect::<HashSet<ShapeID>>();
    let mut hash: HashMap<Version, HashSet<ShapeID>> = Default::default();
    let _ = hash.insert(Version::V10, names);
    hash
}
