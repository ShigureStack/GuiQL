use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    ast::ViewRoot,
    parser::{ParseError, ParseResult},
    tokenizer::{TokenResult, Tokenizer},
};

use super::Parser;

pub enum ViewParserResult {
    Continue,
    ParseError(ParseError),
    Done,
}

#[derive(Default)]
enum ViewParserState {
    #[default]
    Ready,
    PendingToken(TokenResult),
    PendingParseError(ParseError),
    EOF,
}

impl ViewParserState {
    fn is_ready(&self) -> bool {
        match self {
            ViewParserState::Ready => true,
            _ => false,
        }
    }
}

pub struct ViewParser<'a> {
    tokenizer: Rc<RefCell<Tokenizer<'a>>>,
    state: RefCell<ViewParserState>,
    pending: RefCell<Option<ViewRoot>>,
}

impl<'a> Parser<ViewRoot> for ViewParser<'a> {
    fn parse_all(&self) -> ParseResult<ViewRoot> {
        loop {
            match self.advance() {
                ViewParserResult::ParseError(err) => return Err(err),
                ViewParserResult::Done => {
                    return Ok(self.pending.take().expect("No pending result"))
                }
                _ => {}
            }
        }
    }
}

impl<'a> ViewParser<'a> {
    pub fn new(tokenizer: Rc<RefCell<Tokenizer<'a>>>) -> Self {
        ViewParser {
            tokenizer,
            state: RefCell::new(ViewParserState::default()),
            pending: None.into(),
        }
    }

    fn parse_token(&self, res: TokenResult) {}

    fn advance(&self) -> ViewParserResult {
        type State = ViewParserState;

        match self.state.take() {
            State::Ready => {
                match self.consume_token() {
                    Some(tok) => assert!(self.state.replace(State::PendingToken(tok)).is_ready()),
                    None => assert!(self.state.replace(State::EOF).is_ready()),
                };

                ViewParserResult::Continue
            }
            State::PendingToken(tok) => {
                self.parse_token(tok);
                ViewParserResult::Continue
            }
            State::PendingParseError(err) => ViewParserResult::ParseError(err),
            State::EOF => ViewParserResult::Done,
        }
    }

    fn consume_token(&self) -> Option<TokenResult> {
        self.tokenizer.borrow_mut().next()
    }
}
