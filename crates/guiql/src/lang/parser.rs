use crate::lang::tokenizer::{Tokenizer, TokenResult};

pub struct DefinitionParser<'a> {
    tokenizer: Tokenizer<'a>,
    pending: Option<TokenResult>,
}

impl<'a> DefinitionParser<'a> {
    pub fn new() -> Self {
        todo!()
    }
}

#[derive(Default)]
pub struct QueryParser {
    pending: Option<TokenResult>,
}

impl QueryParser {

}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    qparser: QueryParser,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            qparser: QueryParser::default(),
        }
    }

    pub fn parse_all(&mut self) {

    }

    fn advance(&mut self) {

    }
}
