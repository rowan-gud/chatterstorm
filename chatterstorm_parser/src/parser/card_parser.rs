use lalrpop_util::lalrpop_mod;
use std::sync::OnceLock;

lalrpop_mod!(card);

pub use card::*;

pub static PARSER: OnceLock<Parser> = OnceLock::new();

pub struct Parser {
    pub type_line: card::TypeLineParser,
}

#[derive(Debug, Clone)]
pub enum ParserInitError {
    SetCellError,
}

impl Parser {
    pub fn init() -> Result<(), ParserInitError> {
        PARSER
            .set(Parser {
                type_line: card::TypeLineParser::new(),
            })
            .map_err(|_| ParserInitError::SetCellError)
    }
}
