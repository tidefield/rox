// VM implementation is currently intentionally left out while building up the codebase.
// Add VM execution/state logic here when you’re ready to continue.

// mod cchunk;

use crate::chunk::{Chunk, OpCode, Value};
use crate::compiler::compile;

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    // Vec<T> is a heap-backed dynamic array
    // Conceptually, we can use it as a logical runtime stack
    // given it provides push/pop operations
    pub stack: Vec<Value>,
}

impl VM {
    fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }
}

pub fn init_vm(chunk: Chunk) -> VM {
    VM::new(chunk)
}

// These macros are used by the VM dispatch loop when execution is enabled.
#[allow(unused_macros)]
macro_rules! read_byte {
    ($vm:expr) => {{
        let offset = $vm.ip;
        let byte = $vm.chunk.code[offset];
        $vm.ip += 1;
        (byte, offset)
    }};
}

#[allow(unused_macros)]
macro_rules! read_constant {
    ($vm:expr) => {{
        let (value_index, _offset) = read_byte!($vm);
        let constant = $vm.chunk.constants.get(value_index as usize).unwrap();
        constant
    }};
}

#[allow(unused_macros)]
macro_rules! binary_op {
    ($vm:expr, $op:tt) => {{
        let b = $vm.stack.pop().unwrap();
        let a = $vm.stack.pop().unwrap();
        $vm.stack.push(a $op b);
    }};
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub fn interpret(source: &str) -> InterpretResult {
    let chunk = match compile(source) {
        Some(chunk) => chunk,
        None => return InterpretResult::CompileError,
    };
    let mut vm = VM::new(chunk);

    #[cfg(debug_assertions)]
    crate::chunk::disassemble_chunk(&vm.chunk, "code");

    // InterpretResult::Ok

    loop {
        let (instruction, _offset) = read_byte!(vm);
        match OpCode::try_from(instruction) {
            Ok(opcode) => match opcode {
                OpCode::Constant => {
                    let constant = read_constant!(vm);
                    vm.stack.push(*constant);
                }
                OpCode::Return => {
                    println!("{:?}", vm.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Negate => {
                    let value = vm.stack.last_mut().unwrap();
                    *value = -*value;
                }
                OpCode::Add => {
                    binary_op!(vm, +);
                }
                OpCode::Subtract => {
                    binary_op!(vm, -);
                }
                OpCode::Multiply => {
                    binary_op!(vm, *);
                }
                OpCode::Divide => {
                    binary_op!(vm, /);
                }
            },
            Err(_) => {
                unimplemented!()
            }
        }
    }
}
