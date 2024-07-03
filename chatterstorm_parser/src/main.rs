use chatterstorm_engine::objects::Card;
use chatterstorm_parser::{Parseable, Parser};
use scryfall::set::SetType;

#[tokio::main]
async fn main() {
    match Parser::init() {
        Ok(_) => (),
        Err(_) => panic!("Failed to initialize parser"),
    };

    println!("Initialized parser");

    let mut count: usize = 0;

    for card in scryfall::bulk::oracle_cards().await.unwrap() {
        let card = card.unwrap();

        if card.set_type == SetType::Funny {
            continue;
        }

        let parsed_card = Card::parse(&card).unwrap();

        if count % 1000 == 0 {
            println!("{:?}", parsed_card);
        }

        count += 1;
    }

    println!("Parsed {} cards", count);
}
