use crate::bytecode::chunk::Chunk;
use crate::bytecode::chunk::OpType;
use crate::bytecode::value::Value;
use crate::debug::tracer::disassemble_chunk;
use crate::runtime::machine::Vm;

mod bytecode;
mod debug;
mod error;
mod runtime;

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
    let mut vm = Vm::new(&chunk);
    vm.interpret();
}
