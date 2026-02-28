use crate::bytecode::chunk::{Chunk, OpType};
use crate::bytecode::value::Value;
use crate::compiler::error::{CompileError, CompileErrorKind};
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenKind};

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    panic_mode: bool,
    errors: Vec<CompileError>,
    chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut scanner = Scanner::new(source);
        let first = scanner.scan_token();
        let second = scanner.scan_token();
        Self {
            scanner,
            current: second,
            previous: first,
            panic_mode: false,
            errors: Vec::new(),
            chunk: Chunk::new(),
        }
    }

    pub fn compile(mut self) -> Result<Chunk, Vec<CompileError>> {
        self.advance();
        self.expression();
        self.consume(TokenKind::Eof, "Expect end of expression");

        if self.errors.is_empty() {
            Ok(self.chunk)
        } else {
            Err(self.errors)
        }
    }

    // Actual parsing logic
    fn expression(&self) {}

    fn number(&mut self) {
        let value: f64 = self.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let kind = self.previous.kind;

        self.expression();

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

            self.errors.push(CompileError::at(
                &self.current,
                CompileErrorKind::Message(self.current.error.unwrap().into()),
            ));
        }
    }

    fn error_at(&mut self, token: Token<'a>, msg: &'static str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        self.errors.push(CompileError::at(
            &token,
            CompileErrorKind::Message(msg.into()),
        ));
    }

    fn error(&mut self, msg: &'static str) {
        self.error_at(self.previous, msg);
    }

    fn error_at_current(&mut self, msg: &'static str) {
        self.error_at(self.current, msg);
    }

    fn consume(&mut self, kind: TokenKind, msg: &'static str) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.previous.line);
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
