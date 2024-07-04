use crate::tokens::{ObjectType, Subtype, Supertype};

use super::card_type::CardType;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CardFace {
    pub card_face_type: CardType,
}

impl CardFace {
    pub fn has_supertype(&self, supertype: Supertype) -> bool {
        self.card_face_type.supertypes.contains(supertype)
    }

    pub fn has_object_type(&self, object_type: ObjectType) -> bool {
        self.card_face_type.object_types.contains(object_type)
    }

    pub fn has_subtype(&self, subtype: Subtype) -> bool {
        self.card_face_type.subtypes.contains(subtype)
    }
}
