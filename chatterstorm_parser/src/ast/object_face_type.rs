use crate::tokens::{ObjectType, Subtype, Supertype};

#[derive(Debug, Default)]
pub struct ObjectFaceType {
    pub supertypes: Vec<Supertype>,
    pub object_types: Vec<ObjectType>,
    pub subtypes: Vec<Subtype>,

    pub is_token: bool,
}

impl ObjectFaceType {
    pub fn new(
        supertypes: Vec<Supertype>,
        object_types: Vec<ObjectType>,
        subtypes: Vec<Subtype>,
        is_token: bool,
    ) -> Self {
        Self {
            supertypes,
            object_types,
            subtypes,
            is_token,
        }
    }
}
