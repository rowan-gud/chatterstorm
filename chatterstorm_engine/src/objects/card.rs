use scryfall::card::Layout;

use super::card_face::CardFace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: String,
    pub name: String,

    pub layout: Layout,
    pub front: CardFace,
    pub back: Option<CardFace>,
}
