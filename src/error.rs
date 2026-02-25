#[derive(Debug)]
pub enum RuntimeError {
    StackUnderflow { needed: usize, found: usize, ip: usize },
    BadConstantIndex { index: usize, ip: usize },
    InvalidOpCode { opcode: u8, ip: usize }
}

pub enum CompilationError {
    Placeholder
}

pub enum Error {
    RuntimeError(RuntimeError),
    CompilationError(CompilationError)
}
