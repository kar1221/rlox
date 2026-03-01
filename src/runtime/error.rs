use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Stack underflow at ip {ip} (needed {needed}, found {found})")]
    StackUnderflow {
        needed: usize,
        found: usize,
        ip: usize,
    },
    #[error("Constant index out of range at ip {ip} (index {index})")]
    BadConstantIndex { index: usize, ip: usize },
    #[error("Invalid opcode {opcode} at ip {ip}")]
    InvalidOpCode { opcode: u8, ip: usize },

    #[error("Type error at ip {ip}: {op} expected number(s), got {a} and {b}")]
    BinaryTypeError {
        op: &'static str,
        a: &'static str,
        b: &'static str,
        ip: usize,
    },

    #[error("Type error at ip {ip}: {op} expected number, got {got}")]
    UnaryTypeError {
        op: &'static str,
        got: &'static str,
        ip: usize,
    },
}

impl RuntimeError {
    pub fn ip(&self) -> usize {
        match *self {
            RuntimeError::StackUnderflow { ip, .. } => ip,
            RuntimeError::BadConstantIndex { ip, .. } => ip,
            RuntimeError::InvalidOpCode { ip, .. } => ip,
            RuntimeError::BinaryTypeError { ip, .. } => ip,
            RuntimeError::UnaryTypeError { ip, .. } => ip,
        }
    }
}
