use chatterstorm_engine::objects;
use derive_more::Display;

use super::card_parser::{TypeLineParser, PARSER};

#[derive(Debug, Display)]
pub enum ParseError {
    #[display(fmt = "Failed to lock parser")]
    ParserLock,

    #[display(fmt = "Raw data missing field {}", _0)]
    MissingField(String),

    #[display(fmt = "Syntax error: {}", _0)]
    SyntaxError(String),
}

pub trait Parseable {
    fn parse(raw_data: &scryfall::Card) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn parse_type_line(
        type_line: &str,
        parser: &TypeLineParser,
    ) -> Result<(objects::CardType, Option<objects::CardType>), ParseError>;
}

fn to_owned_string<S>(s: &Option<String>, field_name: S) -> Result<String, ParseError>
where
    S: Into<String>,
{
    s.to_owned()
        .ok_or(ParseError::MissingField(field_name.into()))
}

impl Parseable for objects::Card {
    fn parse(raw_card: &scryfall::Card) -> Result<Self, ParseError> {
        let parser = PARSER.get().ok_or(ParseError::ParserLock)?;

        let type_line = to_owned_string(&raw_card.type_line, "type_line")?;
        let face_types = Self::parse_type_line(&type_line, &parser.type_line)?;

        let card = Self {
            id: raw_card.id.to_string(),
            name: raw_card.name.to_owned(),

            layout: raw_card.layout.to_owned(),
            front: objects::CardFace {
                card_face_type: face_types.0,
            },
            back: face_types.1.map(|face_type| objects::CardFace {
                card_face_type: face_type,
            }),
        };

        Ok(card)
    }

    fn parse_type_line(
        type_line: &str,
        parser: &TypeLineParser,
    ) -> Result<(objects::CardType, Option<objects::CardType>), ParseError> {
        let face_types = parser
            .parse(&type_line.to_lowercase())
            .map_err(|e| ParseError::SyntaxError(format!("{}", e)))?;

        Ok(face_types)
    }
}

#[cfg(test)]
mod type_line_tests {
    use chatterstorm_engine::{objects, tokens};

    use crate::Parser;

    use super::*;

    fn init_parser() -> &'static Parser {
        Parser::init().ok();

        PARSER.get().unwrap()
    }

    #[test]
    fn test_single_type() {
        let parser = init_parser();

        let face_types = objects::Card::parse_type_line("creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(vec![], vec![tokens::ObjectType::Creature], vec![], false),
                None
            )
        );
    }

    #[test]
    fn test_token_type() {
        let parser = init_parser();

        let face_types =
            objects::Card::parse_type_line("token creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(vec![], vec![tokens::ObjectType::Creature], vec![], true),
                None
            )
        );
    }

    #[test]
    fn test_supertype() {
        let parser = init_parser();

        let face_types =
            objects::Card::parse_type_line("legendary creature", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(
                    vec![tokens::Supertype::Legendary],
                    vec![tokens::ObjectType::Creature],
                    vec![],
                    false
                ),
                None
            )
        );
    }

    #[test]
    fn test_subtype() {
        let parser = init_parser();

        let face_types =
            objects::Card::parse_type_line("creature - human", &parser.type_line).unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(
                    vec![],
                    vec![tokens::ObjectType::Creature],
                    vec![tokens::Subtype::Human],
                    false
                ),
                None
            )
        );
    }

    #[test]
    fn test_multiple_types() {
        let parser = init_parser();

        let face_types = objects::Card::parse_type_line(
            "legendary artifact creature - vampire blood",
            &parser.type_line,
        )
        .unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(
                    vec![tokens::Supertype::Legendary],
                    vec![tokens::ObjectType::Artifact, tokens::ObjectType::Creature],
                    vec![tokens::Subtype::Vampire, tokens::Subtype::Blood],
                    false,
                ),
                None
            )
        );
    }

    #[test]
    fn test_dual_face_type() {
        let parser = init_parser();

        let face_types = objects::Card::parse_type_line(
            "legendary artifact creature - blood vampire // enchantment - aura",
            &parser.type_line,
        )
        .unwrap();

        assert_eq!(
            face_types,
            (
                objects::CardType::new(
                    vec![tokens::Supertype::Legendary],
                    vec![tokens::ObjectType::Artifact, tokens::ObjectType::Creature],
                    vec![tokens::Subtype::Vampire, tokens::Subtype::Blood],
                    false,
                ),
                Some(objects::CardType::new(
                    vec![],
                    vec![tokens::ObjectType::Enchantment],
                    vec![tokens::Subtype::Aura],
                    false,
                ))
            )
        );
    }
}
