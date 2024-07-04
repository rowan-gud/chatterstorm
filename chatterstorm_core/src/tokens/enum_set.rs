macro_rules! enum_set {
    ($name:ident, $enum_name:ident) => {
        #[derive(Clone, Default, PartialEq, Eq)]
        pub struct $name(bit_set::BitSet);

        impl $name {
            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn contains(&self, value: $enum_name) -> bool {
                self.0.contains(value as usize)
            }

            pub fn add(&mut self, value: $enum_name) -> bool {
                self.0.insert(value as usize)
            }

            pub fn add_all(&mut self, other: &Self) {
                self.0.union_with(&other.0)
            }

            pub fn remove(&mut self, value: $enum_name) -> bool {
                self.0.remove(value as usize)
            }

            pub fn remove_all(&mut self, other: &Self) {
                self.0.difference_with(&other.0)
            }
        }

        impl From<$name> for bit_set::BitSet {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<bit_set::BitSet> for $name {
            fn from(value: bit_set::BitSet) -> Self {
                Self(value)
            }
        }

        impl From<Vec<$enum_name>> for $name {
            fn from(value: Vec<$enum_name>) -> Self {
                let bitset =
                    bit_set::BitSet::from_iter(value.into_iter().map(|item| item as usize));

                Self(bitset)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_list()
                    .entries(
                        self.0
                            .iter()
                            .filter_map(|item| $enum_name::try_from(item).ok()),
                    )
                    .finish()
            }
        }
    };
}
