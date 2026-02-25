use crate::Chunk;
use crate::OpType;
use crate::Value;
use crate::debug::tracer::disassemble_instruction;
use crate::debug::tracer::stack_tracing;
use crate::runtime::error::RuntimeError;

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) {
        if let Err(e) = self.run() {
            report_runtime_error(self.chunk, &e);
        }
    }

    fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            stack_tracing(&self.stack);
            disassemble_instruction(self.chunk, self.ip);

            let instruction = self.read_byte();
            let instruction_ip = self.ip;

            match instruction {
                x if x == OpType::Return as u8 => {
                    let value = self.pop()?;
                    println!("{value}");
                    return Ok(());
                }
                x if x == OpType::Constant as u8 => {
                    let offset = self.read_byte();
                    let constant = self.chunk.value_at(offset as usize).ok_or(
                        RuntimeError::BadConstantIndex {
                            index: offset as usize,
                            ip: instruction_ip,
                        },
                    )?;
                    self.push(constant);
                }
                x if x == OpType::Negate as u8 => {
                    let v = self.pop()?;
                    self.push(-v);
                }
                x if x == OpType::Add as u8 => self.binary_op(|a, b| a + b)?,
                x if x == OpType::Subtract as u8 => self.binary_op(|a, b| a - b)?,
                x if x == OpType::Multiply as u8 => self.binary_op(|a, b| a * b)?,
                x if x == OpType::Divide as u8 => self.binary_op(|a, b| a / b)?,
                _ => {
                    return Err(RuntimeError::InvalidOpCode {
                        opcode: instruction,
                        ip: instruction_ip,
                    });
                }
            };
        }
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code()[self.ip];
        self.ip += 1;
        b
    }

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow {
            needed: 1,
            found: 0,
            ip: self.ip,
        })
    }

    fn pop2(&mut self) -> Result<(Value, Value), RuntimeError> {
        let b = self.pop()?;
        let a = self.pop()?;

        Ok((a, b))
    }

    fn binary_op<F>(&mut self, f: F) -> Result<(), RuntimeError>
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let (a, b) = self.pop2()?;
        self.push(f(a, b));
        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}

fn report_runtime_error(chunk: &Chunk, err: &RuntimeError) {
    let ip = err.ip();
    let line = chunk.line_at(ip).unwrap_or(0);
    eprintln!("[runtime error] line {line} @ ip {ip}: {err}");
}
