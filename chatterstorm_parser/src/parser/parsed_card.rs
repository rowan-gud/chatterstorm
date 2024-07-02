use crate::ast;

use super::card_parser::PARSER;

#[derive(Debug)]
pub struct Card {
    pub face_types: (ast::ObjectFaceType, Option<ast::ObjectFaceType>),
}

impl Card {
    pub fn parse(raw_card: &scryfall::Card) -> Self {
        let face_types = PARSER
            .get()
            .unwrap()
            .type_line
            .parse(
                raw_card
                    .type_line
                    .to_owned()
                    .unwrap_or_default()
                    .to_lowercase()
                    .as_str(),
            )
            .unwrap();

        Self { face_types }
    }
}
