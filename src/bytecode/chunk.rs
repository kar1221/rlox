use crate::Value;
use crate::bytecode::line::LineTracker;

#[repr(u8)]
pub enum OpType {
    Return = 0,
    Constant = 1,
    ConstantLong = 2,
    Negate = 3,
    Add = 4,
    Subtract = 5,
    Multiply = 6,
    Divide = 7,
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: LineTracker,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: LineTracker::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_op(&mut self, op_code: OpType, line: usize) {
        self.write_byte(op_code as u8, line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        self.constants.push(value);
        let index = (self.constants.len() - 1) as u32;

        if index <= 0xFF {
            self.write_op(OpType::Constant, line);
            self.write_byte(index as u8, line);
        } else if index <= 0xFF_FFFF {
            self.write_op(OpType::ConstantLong, line);
            self.write_byte((index & 0xFF) as u8, line);
            self.write_byte(((index >> 8) & 0xFF) as u8, line);
            self.write_byte(((index >> 16) & 0xFF) as u8, line);
        } else {
            panic!("Too many constants in chunk (max 16,777,216).");
        }
    }

    pub fn value_at(&self, index: usize) -> Option<Value> {
        self.constants.get(index).cloned()
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn get_constant(&self) -> &[Value] {
        &self.constants
    }

    pub fn line_at(&self, offset: usize) -> Option<usize> {
        self.lines.line_at(offset)
    }
}
