use std::cell::RefCell;

use crate::lang::tokenizer::{TokenResult, Tokenizer, TokenizerErr};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum ParserState {
    PendingToken(TokenResult),
    PendingTokenizeError(TokenizerErr),
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
    pub fn has_pending_token(&self) -> Option<&TokenResult> {
        match self {
            ParserState::PendingToken(token) => Some(token),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    state: RefCell<ParserState>,
}

pub enum ParseError {}

pub enum ParserResult {
    Continue,
    TokenizeError(TokenizerErr),
    ParseError(ParseError),
    Done,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            state: RefCell::new(ParserState::default()),
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(Tokenizer::new(input))
    }

    pub fn parse_all(&mut self) {
        self.advance();
    }

    fn parse_token(&mut self, res: TokenResult) {
        match res {
            Ok(token) => {
                match token.con {
                    _ => {}
                };
            }
            Err(err) => {
                assert!(self
                    .state
                    .replace(ParserState::PendingTokenizeError(err))
                    .is_ready());
            }
        }
    }

    fn advance(&mut self) -> ParserResult {
        match self.state.take() {
            ParserState::Ready => {
                self.consume_token();
                ParserResult::Continue
            }
            ParserState::PendingToken(token) => {
                self.parse_token(token);
                ParserResult::Continue
            }
            ParserState::PendingTokenizeError(err) => ParserResult::TokenizeError(err.clone()),
            ParserState::EOF => ParserResult::Done,
        }
    }

    fn consume_token(&mut self) {
        match self.tokenizer.next() {
            Some(res) => {
                assert!(self
                    .state
                    .replace(ParserState::PendingToken(res))
                    .is_ready());
            }
            None => {
                assert!(self.state.replace(ParserState::EOF).is_ready());
            }
        }
    }

    pub fn next() {}
}
