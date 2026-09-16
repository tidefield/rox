mod chunk;
mod vm;

use chunk::{Chunk, OpCode, add_constant, write_chunk};

use vm::{init_vm, interpret};

use std::io;

fn repl() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
    }
    interpret(buffer);
}

fn main() {
    // const NEGATIONS: usize = 50_000_000;

    // let mut chunk = Chunk {
    //     line_tuples: Vec::with_capacity(2),
    //     code: Vec::with_capacity(NEGATIONS + 3),
    //     constants: Vec::with_capacity(1),
    // };

    // let constant = add_constant(&mut chunk, 1.0);

    // write_chunk(&mut chunk, OpCode::Constant as u8, 123);
    // write_chunk(&mut chunk, constant as u8, 123);
    // for _ in 0..NEGATIONS {
    //     write_chunk(&mut chunk, OpCode::Negate as u8, 123);
    // }
    // write_chunk(&mut chunk, OpCode::Return as u8, 123);

    // let mut vm = init_vm(chunk);
    repl();
    // interpret(&mut vm);
}
