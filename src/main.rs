use crate::bytecode::chunk::Chunk;
use crate::bytecode::chunk::OpType;
use crate::bytecode::value::Value;
use crate::debug::tracer::disassemble_chunk;
use crate::runtime::machine::Vm;

use std::env;
use std::io;
use std::process::exit;

mod bytecode;
mod compiler;
mod debug;
mod error;
mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            exit(64);
        }
    }
}

fn repl() {
    let mut input = String::new();
    print!("> ");

    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
    }
}

fn run_file(source_path: &str) {}
