use std::cell::RefCell;

use crate::lang::tokenizer::{TokenResult, Tokenizer, TokenizerErr};
use crate::lang::ast::{ASTItem, TokenContent};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum ParserState {
    PendingToken(TokenResult),
    PendingParseError(ParseError),
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

pub struct PendingASTItem {
    item: ASTItem,
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    state: RefCell<ParserState>,
    pending_item: Option<PendingASTItem>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedToken,
    TokenizeError(TokenizerErr),
}

pub enum ParserResult {
    Continue,
    ParseError(ParseError),
    Done,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            state: RefCell::new(ParserState::default()),
            pending_item: None,
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(Tokenizer::new(input))
    }

    pub fn parse_all(&mut self) {
        self.advance();
    }

    fn set_pending_err(&mut self, err: ParseError) {
        assert!(self.state.replace(ParserState::PendingParseError(err)).is_ready());
    }

    fn parse_token(&mut self, res: TokenResult) {
        match res {
            Ok(token) => {
                match token.con {
                    _ => {
                        self.set_pending_err(ParseError::UnexpectedToken);
                    }
                }
            }
            Err(err) => {
                self.set_pending_err(ParseError::TokenizeError(err));
            }
        }
    }

    fn advance(&mut self) -> ParserResult {
        match self.state.take() {
            ParserState::Ready => {
                match self.consume_token() {
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
                ParserResult::Continue
            }
            ParserState::PendingToken(token) => {
                self.parse_token(token);
                ParserResult::Continue
            }
            ParserState::PendingParseError(err) => ParserResult::ParseError(err.clone()),
            ParserState::EOF => ParserResult::Done,
        }
    }

    fn consume_token(&mut self) -> Option<TokenResult> {
        self.tokenizer.next()
    }
}
