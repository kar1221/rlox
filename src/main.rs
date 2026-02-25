mod chunk;
mod debug;
mod error;
mod line;
mod value;
mod vm;

use crate::chunk::{Chunk, OpType};
use crate::debug::disassemble_chunk;
use crate::value::Value;
use crate::vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(Value::number(1.0), 1);
    chunk.write_constant(Value::number(5.0), 1);
    chunk.write_op(OpType::Add, 1);
    chunk.write_op(OpType::Negate, 1);
    chunk.write_constant(Value::number(2.0), 1);
    chunk.write_op(OpType::Multiply, 1);
    chunk.write_op(OpType::Return, 1);

    disassemble_chunk(&chunk, "Before interpreter");
    println!();
    let mut vm = VM::new(&chunk);
    vm.interpret();
}
