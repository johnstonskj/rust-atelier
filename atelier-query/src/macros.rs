// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! is_only {
    ($name:ident, $variant:ident) => {
        paste! {
        #[doc = "Returns `true` if `self` is the `" $variant "` variant, else `false`."]
        pub fn [<is_ $name>](&self) -> bool {
            matches!(self, Self::$variant)
        }}
    };
}

macro_rules! is_as {
    ($name:ident, $variant:ident, $ret_type:ty) => {
        is_as! { $name, $variant }

        paste! {
        #[doc = "Returns `Some(...)` if `self` is the `" $variant "` variant, else `None`."]
        pub fn [<as_ $name>](&self) -> Option<&$ret_type> {
            match self {
                Self::$variant(v) => Some(v),
                _ => None,
            }
        }}
    };
    ($name:ident, $variant:ident) => {
        paste! {
        #[doc = "Returns `true` if `self` is the `" $variant "` variant, else `false`."]
        pub fn [<is_ $name>](&self) -> bool {
            matches!(self, Self::$variant(_))
        }}
    };
}

macro_rules! is_as_array {
    ($name:ident, $variant:ident, $ret_type:ty) => {
        is_as! { $name, $variant }

        paste! {
        #[doc = "Returns an iterator of `(" $ret_type ")`."]
        pub fn [<as_ $name>](&self) -> Option<impl Iterator<Item = &$ret_type>> {
            match self {
                Self::$variant(v) => Some(v.iter()),
                _ => None,
            }
        }}
    };
}
