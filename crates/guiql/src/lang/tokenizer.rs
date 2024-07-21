use std::{cell::RefCell, iter::Peekable, str::Chars};
use crate::lang::ast::*;

#[derive(Debug, Clone, Copy)]
pub enum TokenizerErr {
    UnterminatedStringLiteral,
    UnexpectedToken,
}

pub type TokenResult = Result<Token, TokenizerErr>;

pub struct Tokenizer<'a>
{
    itr: Peekable<Chars<'a>>,
    pending: RefCell<Option<Token>>,
    current_idx: u32,
    full_idx_count: u32,
}

const MAX_IDX_VALUE: u32 = 4294967295;

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            itr: input.chars().peekable(),
            pending: RefCell::new(None),
            current_idx: 0,
            full_idx_count: 0,
        }
    }

    fn lex_number_literal(&mut self) -> TokenResult {
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };

        let mut literal = String::new();
        let mut len = 0;
        while let Some(&c) = self.itr.peek() {
            if c.is_digit(10) {
                literal.push(c);
                len += 1;
                self.next_char();
            } else {
                break;
            }
        };

        loc.len = len;
        Ok(Token {
            loc,
            con: TokenContent::NumberLiteral(literal),
        })
    }

    fn lex_string_literal(&mut self) -> TokenResult {
        let mut literal = String::new();
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };
        let mut inner_quotation_mark = false;
        while let Some(&c) = self.itr.peek() {
            literal.push(c);
            self.next_char();
            loc.len += 1;
            if c == '\"' {
                if inner_quotation_mark {
                    break;
                } else {
                    inner_quotation_mark = true;
                }
            }
        };

        if inner_quotation_mark {
            Err(TokenizerErr::UnterminatedStringLiteral)
        } else {
            Ok(Token {
                loc,
                con: TokenContent::StringLiteral(literal),
            })
        }
    }

    fn lex_reserved(&mut self) -> Option<TokenResult> {
        let mut word = String::new();
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };
        while let Some(&c) = self.itr.peek() {
            if c.is_alphabetic() {
                word.push(c);
                if let Some(con) = TokenContent::from_str(word.as_str()) {
                    return Some(Ok(Token {
                        loc,
                        con
                    }));
                };
                self.next_char();
                loc.len += 1;
            } else {
                self.pending.replace(Some(Token {
                    loc,
                    con: TokenContent::Identifier(word),
                }));

                return None;
            }
        };

        self.pending.replace(Some(Token {
            loc,
            con: TokenContent::Identifier(word),
        }));

        None
    }

    fn lex_identifier(&mut self) -> TokenResult {
        let mut word = String::new();
        let mut loc = TokenLoc {
            starts_at: self.current_idx,
            len: 0,
        };

        if let Some(pending) = &*self.pending.borrow_mut() {
            loc = pending.loc;

            match &pending.con {
                TokenContent::Identifier(s) => {
                    word = s.to_string();
                },
                _ => {
                    return Err(TokenizerErr::UnexpectedToken);
                },
            }
        };

        while let Some(&c) = self.itr.peek() {
            if c.is_whitespace() {
                break;
            } else if c == ';' {
                break;
            }

            word.push(c);
            loc.len += 1;
            self.itr.next();
        };

        Ok(Token {
            loc,
            con: TokenContent::Identifier(word),
        })
    }

    fn lex_alphabetical_chars(&mut self) -> TokenResult {
        if let Some(token) = self.lex_reserved() {
            return token;
        } else {
            return self.lex_identifier();
        }
    }

    fn next_char(&mut self) {
        self.itr.next();

        self.current_idx += 1;

        if self.current_idx == MAX_IDX_VALUE {
            self.full_idx_count += 1;
            self.current_idx = 0;
        }
    }

    pub fn next(&mut self) -> Option<TokenResult> {
        while let Some(&c) = self.itr.peek() {
            if c.is_whitespace() {
                self.next_char();
                continue;
            } else if c.is_digit(10) {
                return Some(self.lex_number_literal());
            } else if c.is_alphabetic() {
                return Some(self.lex_alphabetical_chars());
            } else {
                break;
            }
        };
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_decimal_digits() {
        let expected = Token {
            loc: TokenLoc {
                starts_at: 0,
                len: 2,
            },
            con: TokenContent::NumberLiteral("91".to_string()),
        };

        let mut tokenizer = Tokenizer::new("91");
        while let Some(token) = tokenizer.next() {
            assert!(token.unwrap() == expected, "Unexpected result with a number literal.");
        };
    }

    #[test]
    fn multiple_tokens() {
        let expected = Token {
            loc: TokenLoc {
                starts_at: 2,
                len: 2,
            },
            con: TokenContent::NumberLiteral("91".to_string()),
        };

        let mut tokenizer = Tokenizer::new("x 91");
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next() {
            println!("{:?}", token.clone().unwrap());
            tokens.push(token.unwrap());
        };
        assert!(tokens[1] == expected, "Unexpected result with a number literal.");
    }
}
