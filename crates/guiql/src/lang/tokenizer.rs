use std::{iter::Peekable, str::Chars};
use crate::lang::ast::*;

pub struct Tokenizer<'a>
{
    itr: Peekable<Chars<'a>>,
    pending: Option<String>,
    current_idx: u32,
    full_idx_count: u32,
}

const MAX_IDX_VALUE: u32 = 4294967295;

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            itr: input.chars().peekable(),
            pending: None,
            current_idx: 0,
            full_idx_count: 0,
        }
    }

    fn lex_number_literal(&mut self) -> Token {
        let mut loc = TokenLoc {
            starts_at: 0,
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
        Token {
            loc,
            con: TokenContent::NumberLiteral(literal),
        }
    }

    fn lex_reserved(&mut self) -> Option<Token> {
        let mut word = String::new();
        while let Some(&c) = self.itr.peek() {
            if c.is_alphabetic() {
                word.push(c);
                if let Some(con) = TokenContent::from_str(word.as_str()) {
                    return Token {
                    };
                }
                self.next_char();
            } else {
            }
        };
        None
    }

    fn lex_identifier(&mut self) -> Token {
        todo!();
    }

    fn lex_alphabetical_chars(&mut self) -> Token {
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

    pub fn next(&mut self) -> Option<Token> {
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
            assert!(token == expected, "Unexpected result with a number literal.");
        };
    }

    #[test]
    fn multiple_tokens() {
        let expected = Token {
            loc: TokenLoc {
                starts_at: 1,
                len: 2,
            },
            con: TokenContent::NumberLiteral("91".to_string()),
        };

        let mut tokenizer = Tokenizer::new("x91");
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next() {
            tokens.push(token);
        };
        assert!(tokens[1] == expected, "Unexpected result with a number literal.");
    }
}
