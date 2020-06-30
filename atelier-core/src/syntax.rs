/*!
String constants for elements of the model.
*/

// ------------------------------------------------------------------------------------------------
// ShapeID Separators
// ------------------------------------------------------------------------------------------------

/// The character separating components of a `Namespace` string.
pub const SHAPE_ID_NAMESPACE_SEPARATOR: char = '.';

/// The character separating a `Namespace` and `Identifier` in an absolute `ShapeID`.
pub const SHAPE_ID_ABSOLUTE_SEPARATOR: char = '#';

/// The character separating the shape name and member name in a `ShapeID`.
pub const SHAPE_ID_MEMBER_SEPARATOR: char = '$';

// ------------------------------------------------------------------------------------------------
// Simple Shape Names
// ------------------------------------------------------------------------------------------------

/// String identifier for the simple shape `SimpleShape::Blob`.
pub const SHAPE_BLOB: &str = "blob";

/// String identifier for the simple shape `SimpleShape::Boolean`.
pub const SHAPE_BOOLEAN: &str = "boolean";

/// String identifier for the simple shape `SimpleShape::Document`.
pub const SHAPE_DOCUMENT: &str = "document";

/// String identifier for the simple shape `SimpleShape::String`.
pub const SHAPE_STRING: &str = "string";

/// String identifier for the simple shape `SimpleShape::Byte`.
pub const SHAPE_BYTE: &str = "byte";

/// String identifier for the simple shape `SimpleShape::Short`.
pub const SHAPE_SHORT: &str = "short";

/// String identifier for the simple shape `SimpleShape::Integer`.
pub const SHAPE_INTEGER: &str = "integer";

/// String identifier for the simple shape `SimpleShape::Long`.
pub const SHAPE_LONG: &str = "long";

/// String identifier for the simple shape `SimpleShape::Float`.
pub const SHAPE_FLOAT: &str = "float";

/// String identifier for the simple shape `SimpleShape::Double`.
pub const SHAPE_DOUBLE: &str = "double";

/// String identifier for the simple shape `SimpleShape::BigInteger`.
pub const SHAPE_BIG_INTEGER: &str = "bigInteger";

/// String identifier for the simple shape `SimpleShape::BigDecimal`.
pub const SHAPE_BIG_DECIMAL: &str = "bigDecimal";

/// String identifier for the simple shape `SimpleShape::Timestamp`.
pub const SHAPE_TIMESTAMP: &str = "timestamp";

// ------------------------------------------------------------------------------------------------
// Shape Names
// ------------------------------------------------------------------------------------------------

/// String identifier for the shape `ShapeBody::List`.
pub const SHAPE_LIST: &str = "list";

/// String identifier for the shape `ShapeBody::Set`.
pub const SHAPE_SET: &str = "set";

/// String identifier for the shape `ShapeBody::Map`.
pub const SHAPE_MAP: &str = "map";

/// String identifier for the shape `ShapeBody::Structure`.
pub const SHAPE_STRUCTURE: &str = "structure";

/// String identifier for the shape `ShapeBody::Union`.
pub const SHAPE_UNION: &str = "union";

/// String identifier for the shape `ShapeBody::Service`.
pub const SHAPE_SERVICE: &str = "service";

/// String identifier for the shape `ShapeBody::Operation`.
pub const SHAPE_OPERATION: &str = "operation";

/// String identifier for the shape `ShapeBody::Resource`.
pub const SHAPE_RESOURCE: &str = "resource";

/// String identifier for the shape `ShapeBody::Apply`.
pub const SHAPE_APPLY: &str = "apply";

// ------------------------------------------------------------------------------------------------
// Member Names
// ------------------------------------------------------------------------------------------------

/// The member named "member" on the shapes List and Set.
pub const MEMBER_MEMBER: &str = "member";

/// The member named "key" on the shape Map.
pub const MEMBER_KEY: &str = "key";

/// The member named "value" on the shape Map.
pub const MEMBER_VALUE: &str = "value";

/// The member named "version" on the shape Service.
pub const MEMBER_VERSION: &str = "version";

/// The member named "operations" on the shapes Service and Resource.
pub const MEMBER_OPERATIONS: &str = "operations";

/// The member named "resources" on the shapes Service and Resource.
pub const MEMBER_RESOURCES: &str = "resources";

/// The member named "input" on the shape Operation.
pub const MEMBER_INPUT: &str = "input";

/// The member named "output" on the shape Operation.
pub const MEMBER_OUTPUT: &str = "output";

/// The member named "errors" on the shape Operation.
pub const MEMBER_ERRORS: &str = "errors";

/// The member named "identifiers" on the shape Resource.
pub const MEMBER_IDENTIFIERS: &str = "identifiers";

/// The member named "create" on the shape Resource.
pub const MEMBER_CREATE: &str = "create";

/// The member named "put" on the shape Resource.
pub const MEMBER_PUT: &str = "put";

/// The member named "read" on the shape Resource.
pub const MEMBER_READ: &str = "read";

/// The member named "update" on the shape Resource.
pub const MEMBER_UPDATE: &str = "update";

/// The member named "delete" on the shape Resource.
pub const MEMBER_DELETE: &str = "delete";

/// The member named "list" on the shape Resource.
pub const MEMBER_LIST: &str = "list";

/// The member named "collectionOperations" on the shape Resource.
pub const MEMBER_COLLECTION_OPERATIONS: &str = "collectionOperations";
