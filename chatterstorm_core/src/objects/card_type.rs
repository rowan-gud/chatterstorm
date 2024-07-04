use crate::{
    tokens::{ObjectType, Subtype, Supertype},
    ObjectTypes, Subtypes, Supertypes,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CardType {
    pub supertypes: Supertypes,
    pub object_types: ObjectTypes,
    pub subtypes: Subtypes,

    pub is_token: bool,

    original_supertypes: Supertypes,
    original_object_types: ObjectTypes,
    original_subtypes: Subtypes,
}

impl CardType {
    pub fn new(
        supertypes: Vec<Supertype>,
        object_types: Vec<ObjectType>,
        subtypes: Vec<Subtype>,
        is_token: bool,
    ) -> Self {
        let supertypes = Supertypes::from(supertypes);
        let object_types = ObjectTypes::from(object_types);
        let subtypes = Subtypes::from(subtypes);

        Self {
            original_supertypes: supertypes.clone(),
            original_object_types: object_types.clone(),
            original_subtypes: subtypes.clone(),

            is_token,

            supertypes,
            object_types,
            subtypes,
        }
    }

    pub fn restore(&mut self) {
        self.supertypes.clone_from(&self.original_supertypes);
        self.object_types.clone_from(&self.original_object_types);
        self.subtypes.clone_from(&self.original_subtypes);
    }

    pub fn restore_supertypes(&mut self) {
        self.supertypes.clone_from(&self.original_supertypes);
    }

    pub fn restore_object_types(&mut self) {
        self.object_types.clone_from(&self.original_object_types);
    }

    pub fn restore_subtypes(&mut self) {
        self.subtypes.clone_from(&self.original_subtypes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let card_type = CardType::new(
            vec![Supertype::Basic],
            vec![ObjectType::Creature],
            vec![Subtype::Human],
            false,
        );

        assert_eq!(card_type.supertypes.len(), 1);
        assert_eq!(card_type.object_types.len(), 1);
        assert_eq!(card_type.subtypes.len(), 1);
    }
}
