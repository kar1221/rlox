use crate::Chunk;
use crate::OpType;
use crate::Value;
use crate::compiler::compile;
use crate::debug::tracer::disassemble_instruction;
use crate::debug::tracer::stack_tracing;
use crate::runtime::error::RuntimeError;

pub struct Vm {
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) {
        match compile(source) {
            Ok(chunk) => {
                if let Err(e) = self.run(&chunk) {
                    report_runtime_error(&chunk, &e);
                }
            }
            Err(errs) => {
                for e in errs {
                    eprintln!("{e}");
                }
            }
        }
    }

    fn run(&mut self, chunk: &Chunk) -> Result<(), RuntimeError> {
        self.ip = 0;
        self.stack.clear();

        loop {
            stack_tracing(&self.stack);
            disassemble_instruction(chunk, self.ip);

            let instruction_ip = self.ip;
            let instruction = self.read_byte(chunk);

            match instruction {
                x if x == OpType::Return as u8 => {
                    let value = self.pop()?;
                    println!("{value}");
                    return Ok(());
                }
                x if x == OpType::Constant as u8 => {
                    let offset = self.read_byte(chunk);
                    let constant =
                        chunk
                            .value_at(offset as usize)
                            .ok_or(RuntimeError::BadConstantIndex {
                                index: offset as usize,
                                ip: instruction_ip,
                            })?;
                    self.push(constant);
                }
                x if x == OpType::Negate as u8 => {
                    let v = self.pop()?;
                    self.push(v.negate(instruction_ip)?);
                }
                x if x == OpType::Add as u8 => self.binary_op(instruction_ip, Value::add)?,
                x if x == OpType::Subtract as u8 => self.binary_op(instruction_ip, Value::sub)?,
                x if x == OpType::Multiply as u8 => self.binary_op(instruction_ip, Value::mul)?,
                x if x == OpType::Divide as u8 => self.binary_op(instruction_ip, Value::div)?,
                x if x == OpType::Nil as u8 => self.push(Value::Nil),
                x if x == OpType::True as u8 => self.push(Value::Boolean(true)),
                x if x == OpType::False as u8 => self.push(Value::Boolean(false)),
                x if x == OpType::Stringify as u8 => self.stringify()?,
                _ => {
                    return Err(RuntimeError::InvalidOpCode {
                        opcode: instruction,
                        ip: instruction_ip,
                    });
                }
            };
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let b = chunk.code()[self.ip];
        self.ip += 1;
        b
    }

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow {
            needed: 1,
            found: self.stack.len(),
            ip: self.ip,
        })
    }

    fn pop2(&mut self) -> Result<(Value, Value), RuntimeError> {
        let b = self.pop()?;
        let a = self.pop()?;

        Ok((a, b))
    }

    fn binary_op<F>(&mut self, ip: usize, f: F) -> Result<(), RuntimeError>
    where
        F: FnOnce(Value, Value, usize) -> Result<Value, RuntimeError>,
    {
        let (a, b) = self.pop2()?;
        self.push(f(a, b, ip)?);
        Ok(())
    }

    fn stringify(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop()?;
        let str = match value {
            Value::Number(n) => format!("{}", n),
            Value::Boolean(b) => format!("{}", b),
            Value::String(s) => s,
            Value::Nil => "nil".to_string(),
        };
        self.push(Value::String(str));
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
