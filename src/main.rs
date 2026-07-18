struct Chunk {
    count: usize,
    capacity: usize,
    code: Vec<u8>,
    constants: Vec<f64>,
}

// Guarantee the enum is only one byte in memory
// as long as we have less than 256 opcodes (aka variants)
#[repr(u8)]
enum OpCode {
    Constant, // followed by the index i of the constant (i.e. chunk.constants[i])
    Return,
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> Self {
        opcode as u8
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            byte if byte == OpCode::Constant as u8 => Ok(OpCode::Constant),
            byte if byte == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

fn add_constant(chunk: &mut Chunk, value: f64) -> usize {
    chunk.constants.push(value);
    chunk.constants.len() - 1
}

fn write_chunk(chunk: &mut Chunk, byte: u8, _line: usize) {
    chunk.code.push(byte);
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        let byte = chunk.code[offset];
        print!("{:04} ", offset);
        match OpCode::try_from(byte) {
            Ok(OpCode::Constant) => {
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = chunk.constants[constant_index];
                println!("OP_CONSTANT {} '{}'", constant_index, constant_value);
                offset += 2;
            }
            Ok(OpCode::Return) => {
                println!("OP_RETURN");
                offset += 1;
            }
            Err(()) => {
                unreachable!();
            }
        }
    }
}

fn main() {
    let mut chunk = Chunk {
        count: 0,
        capacity: 8,
        code: Vec::with_capacity(8),
        constants: Vec::with_capacity(8),
    };

    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OpCode::Constant as u8, constant);
    // TODO: this will panic when constant is greater than 255
    write_chunk(&mut chunk, constant as u8, constant);

    write_chunk(&mut chunk, OpCode::Return as u8, constant);

    disassemble_chunk(&chunk, "test chunk");
}
