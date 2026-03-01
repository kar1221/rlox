use crate::bytecode::chunk::{Chunk, OpType};
use crate::bytecode::value::Value;
use crate::compiler::error::{CompileError, CompileErrorKind};
use crate::compiler::precedence::Precendence;
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenKind};
use crate::debug::tracer::disassemble_chunk;

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    panic_mode: bool,
    errors: Vec<CompileError>,
    chunk: Chunk,
}

fn infix_prec(kind: TokenKind) -> Precendence {
    use TokenKind::*;
    match kind {
        Plus | Minus => Precendence::Term,
        Star | Slash => Precendence::Factor,
        _ => Precendence::None,
    }
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let scanner = Scanner::new(source);

        let dummy = Token {
            kind: TokenKind::Eof,
            lexeme: "",
            line: 1,
            error: None,
        };

        Self {
            scanner,
            current: dummy,
            previous: dummy,
            panic_mode: false,
            errors: Vec::new(),
            chunk: Chunk::new(),
        }
    }

    pub fn compile(mut self) -> Result<Chunk, Vec<CompileError>> {
        self.advance();
        self.expression();
        self.consume(TokenKind::Eof, CompileErrorKind::ExpectEndOfExpression);
        self.emit_return();

        disassemble_chunk(&self.chunk, "Before run");

        if self.errors.is_empty() {
            Ok(self.chunk)
        } else {
            Err(self.errors)
        }
    }

    // Actual parsing logic
    fn parse_precendence(&mut self, precendence: Precendence) {
        self.advance();

        match self.previous.kind {
            TokenKind::Number => self.number(),
            TokenKind::LeftParen => self.grouping(),
            TokenKind::Minus => self.unary(),
            TokenKind::StringPart => self.string(),
            TokenKind::False | TokenKind::True | TokenKind::Nil => self.literal(),
            _ => {
                self.error(CompileErrorKind::ExpectedExpression);
                return;
            }
        }

        while precendence <= infix_prec(self.current.kind) {
            self.advance();
            match self.previous.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    self.binary()
                }
                _ => break,
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precendence(Precendence::Assignment);
    }

    fn literal(&mut self) {
        match self.previous.kind {
            TokenKind::False => self.emit_op(OpType::False),
            TokenKind::True => self.emit_op(OpType::True),
            TokenKind::Nil => self.emit_op(OpType::Nil),
            _ => unreachable!(),
        }
    }

    fn emit_string_from_token(&mut self, token: Token<'a>) {
        let v = token.lexeme.to_string();
        self.emit_constant(Value::String(v));
    }

    fn string(&mut self) {
        let mut has_acc = !self.previous.lexeme.is_empty();

        if has_acc {
            self.emit_string_from_token(self.previous);
        }

        while self.current.kind == TokenKind::InterpStart {
            self.advance(); // Consume $

            self.expression();
            self.consume(TokenKind::InterpEnd, CompileErrorKind::ExpectRightBracket);

            self.emit_op(OpType::Stringify);

            if has_acc {
                self.emit_op(OpType::Add);
            } else {
                has_acc = true;
            }

            match self.current.kind {
                TokenKind::StringPart => {
                    self.advance();
                    if !self.previous.lexeme.is_empty() {
                        self.emit_string_from_token(self.previous);
                        self.emit_op(OpType::Add);
                    }
                }

                TokenKind::StringEnd => {
                    break;
                }

                _ => {
                    self.error_at_current(CompileErrorKind::Message(
                        "Expect string chunk or end of string after interpolation.",
                    ));
                    break;
                }
            }
        }

        // Empty String case
        if !has_acc {
            self.emit_constant(Value::String(String::new()));
        }

        self.consume(TokenKind::StringEnd, CompileErrorKind::UnterminatedString);
    }

    fn binary(&mut self) {
        let kind = self.previous.kind;
        let prec = infix_prec(kind);
        self.parse_precendence(prec.next());

        match kind {
            TokenKind::Plus => self.emit_op(OpType::Add),
            TokenKind::Minus => self.emit_op(OpType::Subtract),
            TokenKind::Star => self.emit_op(OpType::Multiply),
            TokenKind::Slash => self.emit_op(OpType::Divide),
            _ => unreachable!(),
        }
    }

    fn number(&mut self) {
        let value: f64 = self.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, CompileErrorKind::ExpectRightParen);
    }

    fn unary(&mut self) {
        let kind = self.previous.kind;

        self.parse_precendence(Precendence::Unary);

        match kind {
            TokenKind::Minus => self.emit_op(OpType::Negate),
            _ => unreachable!(),
        }
    }

    // Helpers

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            if self.current.kind != TokenKind::Error {
                break;
            }

            let msg = self.current.error.unwrap_or("Scan error.");
            let kind = CompileErrorKind::Message(msg);
            self.error_at_current(kind);
        }
    }

    fn error_at(&mut self, token: Token<'a>, kind: CompileErrorKind) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        self.errors.push(CompileError::at(&token, kind));
    }

    fn error(&mut self, kind: CompileErrorKind) {
        self.error_at(self.previous, kind);
    }

    fn error_at_current(&mut self, kind: CompileErrorKind) {
        self.error_at(self.current, kind);
    }

    fn consume(&mut self, kind: TokenKind, msg: CompileErrorKind) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn emit_op(&mut self, op: OpType) {
        self.chunk.write_op(op, self.previous.line);
    }

    fn emit_return(&mut self) {
        self.chunk.write_op(OpType::Return, self.previous.line);
    }

    fn emit_constant(&mut self, value: Value) {
        self.chunk.write_constant(value, self.previous.line);
    }
}

pub fn compile(source: &str) -> Result<Chunk, Vec<CompileError>> {
    Compiler::new(source).compile()
}
