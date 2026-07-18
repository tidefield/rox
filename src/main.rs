struct Chunk {
    lines: Vec<u8>,
    code: Vec<u8>,
    constants: Vec<f64>,
}

// Guarantee the enum is only one byte in memory
// as long as we have less than 256 opcodes (aka variants)
#[repr(u8)]
enum OpCode {
    // followed by the index i of the constant (i.e. chunk.constants[i])
    Constant,
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

fn write_chunk(chunk: &mut Chunk, byte: u8, line: usize) {
    chunk.code.push(byte);
    chunk.lines.push(line as u8);
}

fn get_line(chunk: &Chunk, offset: usize) -> usize {
    chunk.lines[offset] as usize
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        let byte = chunk.code[offset];
        // NOTE: the book checks if the line is the same as
        // previous and print ` |` instead
        // we don't need to do that for now
        let line = get_line(chunk, offset);
        print!("{:04} {:4} ", offset, line);
        match OpCode::try_from(byte) {
            Ok(OpCode::Constant) => {
                let constant_index = chunk.code[offset + 1] as usize;
                // NOTE: This will panic when out of bounds
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
        lines: Vec::with_capacity(8),
        code: Vec::with_capacity(8),
        constants: Vec::with_capacity(8),
    };

    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OpCode::Constant as u8, 123);
    // TODO: this will panic when constant is greater than 255
    write_chunk(&mut chunk, constant as u8, 123);

    write_chunk(&mut chunk, OpCode::Return as u8, 123);

    disassemble_chunk(&chunk, "test chunk");
}
