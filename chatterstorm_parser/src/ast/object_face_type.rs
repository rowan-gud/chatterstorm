use crate::tokens::{ObjectType, Subtype, Supertype};

#[derive(Debug, Default, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let object_face_type = ObjectFaceType::new(
            vec![Supertype::Basic],
            vec![ObjectType::Creature],
            vec![Subtype::Advisor],
            false,
        );

        assert_eq!(object_face_type.supertypes, vec![Supertype::Basic]);
        assert_eq!(object_face_type.object_types, vec![ObjectType::Creature]);
        assert_eq!(object_face_type.subtypes, vec![Subtype::Advisor]);
        assert!(!object_face_type.is_token);
    }
}
