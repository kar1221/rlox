use crate::compiler::token::Token;
use crate::compiler::token::TokenKind;
use thiserror::Error;

#[derive(Debug)]
pub struct CompileError {
    pub line: usize,
    pub where_: Where,
    pub kind: CompileErrorKind,
}

#[derive(Debug, Clone)]
pub enum Where {
    AtEnd,
    AtLexeme(String),
}

#[derive(Debug, Clone, Error)]
pub enum CompileErrorKind {
    #[error("Unexpected character.")]
    UnexpectedCharacter,

    #[error("Expected expression.")]
    ExpectedExpression,

    #[error("Expect ')' after expression.")]
    ExpectRightParen,

    #[error("Expect ';' after value.")]
    ExpectSemicolon,

    #[error("{0}")]
    Message(String),
}

impl CompileError {
    pub fn at(token: &Token<'_>, kind: CompileErrorKind) -> Self {
        let where_ = match token.kind {
            TokenKind::Eof => Where::AtEnd,
            _ => Where::AtLexeme(token.lexeme.to_string()),
        };

        Self {
            line: token.line,
            where_,
            kind,
        }
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.where_ {
            Where::AtEnd => write!(f, "[line {}] Error at end: {}", self.line, self.kind),
            Where::AtLexeme(lex) => {
                write!(f, "[line {}] Error at '{}': {}", self.line, lex, self.kind)
            }
        }
    }
}

impl std::error::Error for CompileError {}
