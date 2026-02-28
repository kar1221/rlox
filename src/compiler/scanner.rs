use crate::compiler::token::{Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespaces();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let current_char = self.advance_char();

        self.scan_token_from_char(current_char)
    }

    fn scan_token_from_char(&mut self, c: char) -> Token<'a> {
        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            ';' => self.make_token(TokenKind::SemiColon),

            // TODO: Add assignment operator later
            '+' => self.make_token(TokenKind::Plus),
            '-' => self.make_token(TokenKind::Minus),
            '*' => self.make_token(TokenKind::Star),
            // TODO: Add comments
            '/' => self.make_token(TokenKind::Slash),

            '!' => {
                let kind = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };

                self.make_token(kind)
            }

            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };

                self.make_token(kind)
            }

            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };

                self.make_token(kind)
            }

            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };

                self.make_token(kind)
            }

            '"' => self.string(),

            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() => self.identifier(),

            _ => self.make_error_token("Unexpected Character"),
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        while matches!(self.peek_char(), Some(c) if is_alnum(c)) {
            self.advance_char();
        }

        let text = &self.source[self.start..self.current];
        let kind = keyword_kind(text).unwrap_or(TokenKind::Identifier);
        self.make_token(kind)
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek_char() != Some('"') && !self.is_at_end() {
            if self.peek_char() == Some('\n') {
                self.line += 1;
            }
            self.advance_char();
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated String.");
        }

        self.advance_char();
        self.make_token(TokenKind::String)
    }

    fn number(&mut self) -> Token<'a> {
        while matches!(self.peek_char(), Some(ch) if ch.is_ascii_digit()) {
            self.advance_char();
        }

        if matches!(self.peek_char(), Some('.'))
            && matches!(self.peek_next_char(), Some(ch) if ch.is_ascii_digit())
        {
            self.advance_char();

            while matches!(self.peek_char(), Some(ch) if ch.is_ascii_digit()) {
                self.advance_char();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek_char() == Some(expected) {
            self.advance_char();
            true
        } else {
            false
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            match self.peek_char() {
                Some(' ' | '\r' | '\t') => {
                    self.advance_char();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance_char();
                }
                Some('/') => {
                    if self.peek_next_char() == Some('/') {
                        while self.peek_char() != Some('\n') && !self.is_at_end() {
                            self.advance_char();
                        }
                    } else {
                        return;
                    }
                }
                _ => break,
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut it = self.source[self.current..].chars();
        it.next()?;
        it.next()
    }

    fn advance_char(&mut self) -> char {
        let ch = self.peek_char().expect("advance_char called at end.");

        self.current += ch.len_utf8();
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
            error: None,
        }
    }

    fn make_error_token(&self, msg: &'static str) -> Token<'a> {
        Token {
            kind: TokenKind::Error,
            lexeme: &self.source[self.start..self.current], // can be "" or the bad char slice
            line: self.line,
            error: Some(msg),
        }
    }
}

fn keyword_kind(text: &str) -> Option<TokenKind> {
    Some(match text {
        "and" => TokenKind::And,
        "class" => TokenKind::Class,
        "else" => TokenKind::Else,
        "false" => TokenKind::False,
        "for" => TokenKind::For,
        "fun" => TokenKind::Fun,
        "if" => TokenKind::If,
        "nil" => TokenKind::Nil,
        "or" => TokenKind::Or,
        "print" => TokenKind::Print,
        "return" => TokenKind::Return,
        "super" => TokenKind::Super,
        "this" => TokenKind::This,
        "true" => TokenKind::True,
        "var" => TokenKind::Var,
        "while" => TokenKind::While,
        _ => return None,
    })
}

fn is_alpha(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_alnum(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}
