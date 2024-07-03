use derive_more::Display;

use crate::ast;

use super::card_parser::{TypeLineParser, PARSER};

#[derive(Debug)]
pub struct RawCardData {
    pub type_line: String,
}

#[derive(Debug)]
pub struct Card {
    pub name: String,
    pub face_types: (ast::ObjectFaceType, Option<ast::ObjectFaceType>),

    pub raw: RawCardData,
}

#[derive(Debug, Display)]
pub enum ParseError {
    #[display(fmt = "Failed to lock parser")]
    ParserLock,

    #[display(fmt = "Raw data missing field {}", _0)]
    MissingField(String),

    #[display(fmt = "Syntax error: {}", _0)]
    SyntaxError(String),
}

impl Card {
    pub fn parse(raw_card: &scryfall::Card) -> Result<Self, ParseError> {
        let parser = PARSER.get().ok_or(ParseError::ParserLock)?;

        let raw = RawCardData {
            type_line: Self::to_owned_string(&raw_card.type_line, "type_line")?,
        };

        let card = Self {
            name: raw_card.name.to_owned(),
            face_types: Self::parse_type_line(&raw.type_line, &parser.type_line)?,

            raw,
        };

        Ok(card)
    }

    pub fn parse_type_line(
        type_line: &str,
        parser: &TypeLineParser,
    ) -> Result<(ast::ObjectFaceType, Option<ast::ObjectFaceType>), ParseError> {
        let face_types = parser
            .parse(&type_line.to_lowercase())
            .map_err(|e| ParseError::SyntaxError(format!("{}", e)))?;

        Ok(face_types)
    }

    fn to_owned_string<S>(s: &Option<String>, field_name: S) -> Result<String, ParseError>
    where
        S: Into<String>,
    {
        s.to_owned()
            .ok_or(ParseError::MissingField(field_name.into()))
    }
}

#[cfg(test)]
mod parse_tests {
    use test_case::test_case;

    use crate::{tokens, Parser};

    use super::*;

    #[test_case(
        include_str!("./test_data/forest.json"),
        "Forest",
        ast::ObjectFaceType {
            supertypes: vec![tokens::Supertype::Basic],
            object_types: vec![tokens::ObjectType::Land],
            subtypes: vec![tokens::Subtype::Forest],
            ..Default::default()
        },
        None; "when input is a forest")]
    #[test_case(
        include_str!("./test_data/horn_of_the_mark.json"),
        "Horn of the Mark",
        ast::ObjectFaceType {
            supertypes: vec![tokens::Supertype::Legendary],
            object_types: vec![tokens::ObjectType::Artifact],
            ..Default::default()
        },
        None; "when input is a horn of the mark")]
    #[test_case(
        include_str!("./test_data/prepare__fight.json"),
        "Prepare // Fight",
        ast::ObjectFaceType {
            object_types: vec![tokens::ObjectType::Instant],
            ..Default::default()
        },
        Some(ast::ObjectFaceType {
            object_types: vec![tokens::ObjectType::Sorcery],
            ..Default::default()
        }); "when input is a prepare // fight")]
    fn test_parse(
        raw_data: &str,
        name: &str,
        front: ast::ObjectFaceType,
        back: Option<ast::ObjectFaceType>,
    ) {
        Parser::init().ok();

        let raw_card: scryfall::Card = serde_json::from_str(raw_data).unwrap();

        let card = Card::parse(&raw_card).unwrap();

        assert_eq!(card.name, name);
        assert_eq!(card.face_types, (front, back));
    }
}

#[cfg(test)]
mod type_line_tests {
    use crate::{tokens, Parser};

    use super::*;

    fn init_parser() -> &'static Parser {
        Parser::init().ok();

        PARSER.get().unwrap()
    }

    #[test]
    fn test_single_type() {
        let parser = init_parser();

        let face_types = Card::parse_type_line("creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Creature],
                    ..Default::default()
                },
                None
            )
        );
    }

    #[test]
    fn test_token_type() {
        let parser = init_parser();

        let face_types = Card::parse_type_line("token creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Creature],
                    is_token: true,
                    ..Default::default()
                },
                None
            )
        );
    }

    #[test]
    fn test_supertype() {
        let parser = init_parser();

        let face_types = Card::parse_type_line("legendary creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Creature],
                    supertypes: vec![tokens::Supertype::Legendary],
                    ..Default::default()
                },
                None
            )
        );
    }

    #[test]
    fn test_subtype() {
        let parser = init_parser();

        let face_types = Card::parse_type_line("creature - human", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Creature],
                    subtypes: vec![tokens::Subtype::Human],
                    ..Default::default()
                },
                None
            )
        );
    }

    #[test]
    fn test_multiple_types() {
        let parser = init_parser();

        let face_types = Card::parse_type_line(
            "legendary artifact creature - blood vampire",
            &parser.type_line,
        )
        .unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Artifact, tokens::ObjectType::Creature],
                    supertypes: vec![tokens::Supertype::Legendary],
                    subtypes: vec![tokens::Subtype::Blood, tokens::Subtype::Vampire],
                    ..Default::default()
                },
                None
            )
        );
    }

    #[test]
    fn test_dual_face_type() {
        let parser = init_parser();

        let face_types = Card::parse_type_line(
            "legendary artifact creature - blood vampire // enchantment - aura",
            &parser.type_line,
        )
        .unwrap();

        assert_eq!(
            face_types,
            (
                ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Artifact, tokens::ObjectType::Creature],
                    supertypes: vec![tokens::Supertype::Legendary],
                    subtypes: vec![tokens::Subtype::Blood, tokens::Subtype::Vampire],
                    ..Default::default()
                },
                Some(ast::ObjectFaceType {
                    object_types: vec![tokens::ObjectType::Enchantment],
                    subtypes: vec![tokens::Subtype::Aura],
                    ..Default::default()
                })
            )
        );
    }
}
