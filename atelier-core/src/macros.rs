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

macro_rules! delegate {
    ($fn_name:ident ( $( $i:ident : $t:ty ),* ), inner = $inner_name:ident) => {
        paste! {
        #[doc = "Delegate calls to `" $fn_name "` to the inner implementation `" $inner_name "`."]
        pub fn $fn_name(&self) -> bool {
            self.$inner_name.$fn_name($( $i: $t ),*)
        }}
    };
    ($fn_name:ident ( $( $i:ident : $t:ty ),* )) => {
        paste! {
        #[doc = "Delegate calls to `" $fn_name "` to the inner implementation."]
        pub fn $fn_name(&self) -> bool {
            self.inner.$fn_name($( $i: $t ),*)
        }}
    };
    ($fn_name:ident, inner = $inner_name:ident) => {
        paste! {
        #[doc = "Delegate calls to `" $fn_name "` to the inner implementation."]
        pub fn $fn_name(&self) -> bool {
            self.$inner_name.$fn_name()
        }}
    };
    ($fn_name:ident) => {
        paste! {
        #[doc = "Delegate calls to `" $fn_name "` to the inner implementation."]
        pub fn $fn_name(&self) -> bool {
            self.inner.$fn_name()
        }}
    };
}

macro_rules! required_member {
    ($member_name:ident, $member_type:ty) => {
        paste! {
        #[doc = "Return the current value of `" $member_name "`."]
        pub fn $member_name(&self) -> &$member_type {
            &self.$member_name
        }

        #[doc = "Set the current value of `" $member_name "`."]
        pub fn [<set_ $member_name>](&mut self, $member_name: $member_type) {
            self.$member_name = $member_name;
        }}
    };
}

macro_rules! boolean_member {
    ($member_name:ident) => {
        paste! {
        #[doc = "Return the current value of `" $member_name "`."]
        pub fn [<is_ $member_name>](&self) -> bool {
            self.$member_name
        }

        #[doc = "Set the current value of `" $member_name "`."]
        pub fn [<set_ $member_name>](&mut self, $member_name: bool) {
            self.$member_name = $member_name;
        }}
    };
}

macro_rules! optional_member {
    ($member_name:ident, $member_type:ty) => {
        paste! {
        #[doc = "Returns `true` if `" $member_name "` is present, else `false`."]
        pub fn [<has_ $member_name>](&self) -> bool {
            self.$member_name.is_some()
        }

        #[doc = "Return the current value of `" $member_name "`."]
        pub fn $member_name(&self) -> &Option<$member_type> {
            &self.$member_name
        }

        #[doc = "Return the current value of `" $member_name "`."]
        pub fn [<set_ $member_name>](&mut self, $member_name: $member_type) {
            self.$member_name = Some($member_name);
        }

        #[doc = "Return the current value of `" $member_name "` to `None`."]
        pub fn [<unset_ $member_name>](&mut self) {
            self.$member_name = None;
        }}
    };
}

macro_rules! array_member {
    ($collection:ident, $member_name:ident, $member_type:ty) => {
        paste! {
        #[doc = "Returns `true` if the `" $collection "` collection has _any_ elements, else `false`."]
        pub fn [<has_ $collection>](&self) -> bool {
            !self.$collection.is_empty()
        }

        #[doc = "Return an iterator over all elements in the `" $collection "` collection."]
        pub fn $collection(&self) -> impl Iterator<Item = &$member_type> {
            self.$collection.iter()
        }

        #[doc = "Add an element to the `" $collection "` collection."]
        pub fn [<add_ $member_name>](&mut self, $member_name: $member_type) {
            self.$collection.push($member_name);
        }

        #[doc = "Add all provided elements to the `" $collection "` collection."]
        pub fn [<append_ $collection>](&mut self, $collection: &[$member_type]) {
            for $member_name in $collection {
                self.[<add_ $member_name>]($member_name.clone());
            }
        }

        #[doc = "Remove any element, equal to the `" $member_name "`, from the `" $collection "` collection."]
        pub fn [<remove_ $member_name>](&mut self, $member_name: &$member_type) {
            self.$collection.retain(|item| item != $member_name)
        }}
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
