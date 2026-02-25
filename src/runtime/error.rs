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
}

impl RuntimeError {
    pub fn ip(&self) -> usize {
        match *self {
            RuntimeError::StackUnderflow { ip, .. } => ip,
            RuntimeError::BadConstantIndex { ip, .. } => ip,
            RuntimeError::InvalidOpCode { ip, .. } => ip,
        }
    }
}
