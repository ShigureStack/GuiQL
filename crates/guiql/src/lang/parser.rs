use std::cell::RefCell;

use crate::lang::tokenizer::{Tokenizer, TokenResult};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum ParserState {
    PendingToken(TokenResult),
    EOF,
    #[default]
    Ready,
}

impl ParserState {
    pub fn is_ready(&self) -> bool {
        match self {
            ParserState::Ready => true,
            _ => false,
        }
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    state: RefCell<ParserState>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            state: RefCell::new(ParserState::default()),
        }
    }

    pub fn parse_all(&mut self) {
        self.advance();
    }

    fn advance(&mut self) {
        match self.tokenizer.next() {
            Some(res) => {
                assert!(self.state.replace(ParserState::PendingToken(res)).is_ready());
            }
            None => {
                assert!(self.state.replace(ParserState::EOF).is_ready());
            }
        }
    }

    pub fn next() {

    }
}
