use crate::compiler::token::{Token, TokenKind};

#[derive(PartialEq)]
enum ScanMode {
    None,
    String,
    Interpolation,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    mode: ScanMode,
    interp_depth: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
            mode: ScanMode::None,
            interp_depth: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        match self.mode {
            ScanMode::None => {
                self.skip_whitespaces();
                self.start = self.current;

                if self.is_at_end() {
                    return self.make_token(TokenKind::Eof);
                }

                let current_char = self.advance_char();

                self.scan_token_from_char(current_char)
            }
            ScanMode::String => {
                self.start = self.current;
                self.string_part_or_end()
            }
            ScanMode::Interpolation => {
                self.start = self.current;
                self.interpolation_start_or_end()
            }
        }
    }

    fn scan_token_from_char(&mut self, c: char) -> Token<'a> {
        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => {
                if self.interp_depth > 0 {
                    self.interp_depth += 1;
                }

                self.make_token(TokenKind::LeftBrace)
            }
            '}' => {
                if self.interp_depth > 0 {
                    self.interp_depth -= 1;

                    if self.interp_depth == 0 {
                        self.mode = ScanMode::String;
                        return self.make_token(TokenKind::InterpEnd);
                    }
                }
                self.make_token(TokenKind::RightBrace)
            }
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            ';' => self.make_token(TokenKind::SemiColon),

            '+' => self.make_token(TokenKind::Plus),
            '-' => self.make_token(TokenKind::Minus),
            '*' => self.make_token(TokenKind::Star),
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
            c if is_alpha(c) => self.identifier(),

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

    fn string_part_or_end(&mut self) -> Token<'a> {
        if self.peek_char() == Some('"') {
            self.advance_char();
            self.mode = ScanMode::None;
            return self.make_token(TokenKind::StringEnd);
        }

        while self.peek_char() != Some('"') && !self.is_at_end() {
            if self.peek_char() == Some('\n') {
                self.line += 1;
            }

            if self.peek_char() == Some('$') {
                self.mode = ScanMode::Interpolation;
                return self.make_token(TokenKind::StringPart);
            }

            self.advance_char();
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated string.");
        }

        self.make_token(TokenKind::StringPart)
    }

    fn interpolation_start_or_end(&mut self) -> Token<'a> {
        if !self.consume('$') {
            self.mode = ScanMode::String;
            return self.make_error_token("Expect '$' to start interpolation.");
        }

        if !self.consume('{') {
            self.mode = ScanMode::String;
            return self.make_error_token("Expect '{' after '$' in interpolation.");
        }

        self.interp_depth = 1;
        self.mode = ScanMode::None;
        self.make_token(TokenKind::InterpStart)
    }

    fn string(&mut self) -> Token<'a> {
        self.mode = ScanMode::String;
        self.start = self.current;

        if self.peek_char() == Some('"') {
            return self.make_token(TokenKind::StringPart);
        }

        self.string_part_or_end()
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

    fn consume(&mut self, ch: char) -> bool {
        if self.peek_char() == Some(ch) {
            self.advance_char();
            return true;
        }

        false
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
