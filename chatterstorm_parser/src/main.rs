use lalrpop_util::lalrpop_mod;
use serde::{Deserialize, Serialize};

lalrpop_mod!(pub card);

use card::TypeLineParser;

mod ast;
mod tokens;

#[derive(Debug, Deserialize, Serialize)]
struct Card {
    name: String,
    type_line: String,
    set_name: String,
    set_type: String,
}

impl Card {
    pub fn should_skip(&self) -> bool {
        if self.set_type == "funny" {
            return true;
        }

        false
    }
}

fn main() {
    let cards = serde_json::from_str::<Vec<Card>>(include_str!("../data/cards.json")).unwrap();

    let parser = TypeLineParser::new();
    let mut count: u64 = 0;

    for c in &cards {
        if c.should_skip() {
            continue;
        }

        let type_line = c.type_line.to_owned().to_lowercase();
        let res = parser.parse(type_line.as_str());

        if res.is_err() {
            dbg!(c);
            dbg!(&res);
            break;
        }

        count += 1;
    }

    println!("Parsed {} cards out of {}", count, cards.len());
}
