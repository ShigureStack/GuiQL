use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::lang::tokenizer::{TokenResult, Tokenizer, TokenizerErr};
use crate::lang::ast::{ASTItem, Token, TokenContent, CreateQuery};

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

    #[allow(dead_code)]
    pub fn has_pending_token(&self) -> Option<&TokenResult> {
        match self {
            ParserState::PendingToken(token) => Some(token),
            _ => None,
        }
    }
}

struct PendingASTItem {
    pub item: ASTItem,
}

pub struct Parser<'a> {
    tokenizer: Rc<RefCell<Tokenizer<'a>>>,
    state: RefCell<ParserState>,
    result: Cell<Option<PendingASTItem>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    /// The token type is unexpected.
    UnexpectedToken,
    /// Incorrect syntax
    SyntaxError,
    TokenizeError(TokenizerErr),
}

pub type ParseResult = Result<(), ParseError>;
type TokenizeResult = Result<Token, ParseError>;

pub enum ParserResult {
    Continue,
    ParseError(ParseError),
    Done,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: RefCell<Tokenizer<'a>>) -> Self {
        Self {
            tokenizer: Rc::new(tokenizer),
            state: RefCell::new(ParserState::default()),
            result: None.into(),
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(RefCell::new(Tokenizer::new(input)))
    }

    pub fn parse_all(&self) {
        self.advance();
    }

    fn set_pending_err(&self, err: ParseError) {
        assert!(self.state.replace(ParserState::PendingParseError(err)).is_ready());
    }

    fn set_state_from_parse_result(&self, res: ParseResult) {
        if let Err(err) = res {
            self.set_pending_err(err);
        }
    }

    /// Consume token and handle tokenize error and returns it as [`ParseError`].
    /// If the inner tokenizer has no consumable token, it returns [`ParseError::SyntaxError`].
    fn consume_token_or_err(&self) -> TokenizeResult {
        match self.consume_token() {
            Some(res) => {
                match res {
                    Ok(tok) => Ok(tok),
                    Err(err) => Err(ParseError::TokenizeError(err)),
                }
            }
            None => {
                Err(ParseError::SyntaxError)
            }
        }
    }

    fn parse_create(&self) -> ParseResult {
        let tok = self.consume_token_or_err()?;

        let mut query = CreateQuery::default();

        if let TokenContent::Identifier(elm_name) = tok.con {
            query.elm_name = elm_name;
            let tok = self.consume_token_or_err()?;

            return Ok(());
        }

        Err(ParseError::UnexpectedToken)
    }

    fn parse_token(&self, res: TokenResult) {
        match res {
            Ok(token) => {
                match token.con {
                    TokenContent::Create => {
                        let res = self.parse_create();
                        self.set_state_from_parse_result(res);
                    }
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

    fn advance(&self) -> ParserResult {
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

    fn consume_token(&self) -> Option<TokenResult> {
        self.tokenizer.borrow_mut().next()
    }
}
