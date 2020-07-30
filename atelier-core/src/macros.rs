// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! is_as {
    ($is_fn:ident, $variant:ident) => {
        /// Returns `true` if `self` is the corresponding variant, else `false`.
        pub fn $is_fn(&self) -> bool {
            match self {
                Self::$variant => true,
                _ => false,
            }
        }
    };
    ($is_fn:ident, $variant:ident, $as_fn:ident, $ret_type:ty) => {
        /// Returns `true` if `self` is the corresponding variant, else `false`.
        pub fn $is_fn(&self) -> bool {
            match self {
                Self::$variant(_) => true,
                _ => false,
            }
        }

        /// Returns `Some(v)` if `self` is the corresponding variant, else `None`.
        pub fn $as_fn(&self) -> Option<&$ret_type> {
            match self {
                Self::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

macro_rules! delegate {
    ($fn_name:ident ( $( $i:ident : $t:ty ),* ), inner = $inner_name:ident) => {
        /// Delegate this call to the inner implementation.
        pub fn $fn_name(&self) -> bool {
            self.$inner_name.$fn_name($( $i: $t ),*)
        }
    };
    ($fn_name:ident ( $( $i:ident : $t:ty ),* )) => {
        /// Delegate this call to the inner implementation.
        pub fn $fn_name(&self) -> bool {
            self.inner.$fn_name($( $i: $t ),*)
        }
    };
    ($fn_name:ident, inner = $inner_name:ident) => {
        /// Delegate this call to the inner implementation.
        pub fn $fn_name(&self) -> bool {
            self.$inner_name.$fn_name()
        }
    };
    ($fn_name:ident) => {
        /// Delegate this call to the inner implementation.
        pub fn $fn_name(&self) -> bool {
            self.inner.$fn_name()
        }
    };
}

#[allow(unused_macros)]
macro_rules! required_member {
    ($member_name:ident, $member_type:ty, $setter_fn:ident) => {
        required_member! { $member_name, $member_type }

        /// Set the current value of this member.
        pub fn $setter_fn(&mut self, $member_name: $member_type) {
            self.$member_name = $member_name;
        }
    };
    ($member_name:ident, $member_type:ty) => {
        /// Return the current value of this member.
        pub fn $member_name(&self) -> &$member_type {
            &self.$member_name
        }
    };
}

macro_rules! optional_member {
    ($member_name:ident, $member_type:ty, $has_fn:ident, $setter_fn:ident, $unsetter_fn:ident) => {
        optional_member! { $member_name, $has_fn, $member_type }

        /// Set the current value of this member.
        pub fn $setter_fn(&mut self, $member_name: $member_type) {
            self.$member_name = Some($member_name);
        }

        /// Set the current value of this member to `None`.
        pub fn $unsetter_fn(&mut self) {
            self.$member_name = None;
        }
    };
    ($member_name:ident, $has_fn:ident, $member_type:ty) => {
        /// Returns `true` if this shape has a value for this member, else `false`.
        pub fn $has_fn(&self) -> bool {
            self.$member_name.is_some()
        }

        /// Return the current value of this member.
        pub fn $member_name(&self) -> &Option<$member_type> {
            &self.$member_name
        }
    };
}

macro_rules! array_member {
    ($collection:ident, $member_name:ident, $member_type:ty, $has_fn:ident, $add_fn:ident, $append_fn:ident, $remove_fn:ident) => {
        /// Returns `true` if this member's collection has _any_ elements, else `false`.
        pub fn $has_fn(&self) -> bool {
            !self.$collection.is_empty()
        }

        /// Return an iterator over all elements in this member's collection.
        pub fn $collection(&self) -> impl Iterator<Item = &$member_type> {
            self.$collection.iter()
        }

        /// Add an element to this member's collection.
        pub fn $add_fn(&mut self, $member_name: $member_type) {
            self.$collection.push($member_name);
        }

        /// Add all these elements to this member's collection.
        pub fn $append_fn(&mut self, $collection: &[$member_type]) {
            for member in $collection {
                self.$add_fn(member.clone());
            }
        }

        /// Remove any element, equal to the provided value, from this member's collection.
        pub fn $remove_fn(&mut self, $member_name: &$member_type) {
            self.$collection.retain(|item| item != $member_name)
        }
    };
}

macro_rules! linter_or_validator_defn {
    ($struct_name:ident { $( $i:ident : $t:ty ),* },  $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $struct_name {
            issues: Vec<ActionIssue>,
            $( $i: $t ),*
        }
    };
    ($struct_name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $struct_name {
            issues: Vec<ActionIssue>,
        }
    };
}

macro_rules! linter_or_validator_default_impl {
    ($struct_name:ident { $( $i:ident : $d:expr ),* }) => {
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    issues: Default::default(),
                    $( $i: $d ),*
               }
            }
        }
    };
    ($struct_name:ident) => {
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    issues: Default::default(),
                }
            }
        }
    };
}

macro_rules! linter_or_validator_action_impl {
    ($struct_name:ident, $label:expr) => {
        impl Action for $struct_name {
            fn label(&self) -> &'static str {
                $label
            }

            fn issues(&self) -> &Vec<ActionIssue> {
                &self.issues
            }

            fn issues_mut(&mut self) -> &mut Vec<ActionIssue> {
                &mut self.issues
            }
        }
    };
}
