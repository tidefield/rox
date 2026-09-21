use rox::vm::interpret;

use std::{env, io};

fn repl() {
    loop {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).unwrap() == 0 {
            break;
        }
        interpret(&buffer);
    }
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
    if let Some(source) = env::args().nth(1) {
        interpret(&source);
    } else {
        repl();
    }
    // interpret(&mut vm);
}
