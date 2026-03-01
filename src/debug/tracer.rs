#[cfg(debug_assertions)]
use crate::bytecode::chunk::Chunk;
#[cfg(debug_assertions)]
use crate::bytecode::chunk::OpType;
#[cfg(debug_assertions)]
use crate::bytecode::value::Value;

const HEADER_LENGTH: usize = 20;
const LINE_PADDING: usize = 4;
const OFFSET_PADDING: usize = 4;
const OP_NAME_PADDING: usize = 20;
const CONSTANT_PADDING: usize = 0;
const STACK_PADDING: usize = 11;

#[cfg(debug_assertions)]
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    let left = "=".repeat(HEADER_LENGTH / 2);
    let right = "=".repeat(HEADER_LENGTH / 2);
    let header = format!("{left} {name} {right}");

    println!("{}", header);

    let mut offset = 0;

    while offset < chunk.code().len() {
        offset = disassemble_instruction(chunk, offset);
    }

    let footer = "=".repeat(header.len());

    println!("{}", footer);
}

#[cfg(debug_assertions)]
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:0width$} ", offset, width = OFFSET_PADDING);

    let line = chunk.line_at(offset);
    let prev_line = if offset > 0 {
        chunk.line_at(offset - 1)
    } else {
        None
    };

    match (line, prev_line) {
        (Some(l), Some(pl)) if l == pl => print!("{:>width$}", "|", width = LINE_PADDING),
        (Some(l), _) => print!("{:width$}", l, width = LINE_PADDING),
        (None, _) => print!("{:width$}", '?', width = LINE_PADDING),
    }

    let instruction: u8 = *chunk.code().get(offset).unwrap();
    match instruction {
        x if x == OpType::Return as u8 => simple_instruction("OP_RETURN", offset),
        x if x == OpType::Constant as u8 => constant_instruction("OP_CONSTANT", chunk, offset),
        x if x == OpType::ConstantLong as u8 => {
            constant_long_instruction("OP_CONSTANT_LONG", chunk, offset)
        }
        x if x == OpType::Divide as u8 => simple_instruction("OP_DIVIDE", offset),
        x if x == OpType::Multiply as u8 => simple_instruction("OP_MULTIPLY", offset),
        x if x == OpType::Negate as u8 => simple_instruction("OP_NEGATE", offset),
        x if x == OpType::Subtract as u8 => simple_instruction("OP_SUBTRACT", offset),
        x if x == OpType::Add as u8 => simple_instruction("OP_ADD", offset),
        x if x == OpType::Stringify as u8 => simple_instruction("OP_STRINGIFY", offset),
        _ => {
            print!("Unknown opcode {instruction}");
            offset + 1
        }
    }
}

#[cfg(debug_assertions)]
pub fn stack_tracing(stack: &Vec<Value>) {
    if stack.is_empty() {
        return;
    }

    print!("{:>width$} ", '>', width = STACK_PADDING);
    for v in stack {
        print!("[ ");
        print!("{v}");
        print!(" ]");
    }
    println!()
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!(" {name}");
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    if let Some(constant) = chunk.code().get(offset + 1) {
        print!(
            " {:<width$} {:constant_width$} | ",
            name,
            constant,
            width = OP_NAME_PADDING,
            constant_width = CONSTANT_PADDING
        );
        let value = chunk.get_constant().get(*constant as usize);
        print_value(value.unwrap());
        println!();
    }

    offset + 2
}

fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let mut constant_index: u32 = 0;
    let code = chunk.code();

    constant_index |= u32::from(code[offset + 1]);
    constant_index |= u32::from(code[offset + 2]) << 8;
    constant_index |= u32::from(code[offset + 3]) << 16;

    print!(
        " {:<width$} {:constant_width$} | ",
        name,
        constant_index,
        width = OP_NAME_PADDING,
        constant_width = CONSTANT_PADDING
    );

    if let Some(c) = chunk.get_constant().get(constant_index as usize) {
        print_value(c);
    }

    println!();

    offset + 4
}

fn print_value(v: &Value) {
    print!("{v}");
}
