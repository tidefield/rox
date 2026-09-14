pub type Value = f64;

#[derive(Debug, Default)]
pub struct Chunk {
    // (usize, u8) is a tuple of (offset, line)
    // where `offset` is the first index of the instruction (aka code)
    // that is on the line `line`
    pub line_tuples: Vec<(usize, u8)>,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

// Guarantee the enum is only one byte in memory
// as long as we have less than 256 opcodes (aka variants)
#[repr(u8)]
pub enum OpCode {
    // followed by the index i of the constant (i.e. chunk.constants[i])
    Constant,
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
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
            byte if byte == OpCode::Negate as u8 => Ok(OpCode::Negate),
            byte if byte == OpCode::Add as u8 => Ok(OpCode::Add),
            byte if byte == OpCode::Subtract as u8 => Ok(OpCode::Subtract),
            byte if byte == OpCode::Multiply as u8 => Ok(OpCode::Multiply),
            byte if byte == OpCode::Divide as u8 => Ok(OpCode::Divide),
            _ => Err(()),
        }
    }
}

pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
    chunk.constants.push(value);
    chunk.constants.len() - 1
}

pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: usize) {
    chunk.code.push(byte);

    // Given the follow instruction sequence:
    // offset: 0  1  2  3  4  5
    // code   [ ][ ][ ][ ][ ][ ]
    // line    1  1  1  2  2  3
    // We want to store
    // (0, 1), (3, 2), (5, 3)
    // NOTE: The book uses a different approach to store line information, but this is more
    // efficient for our use case
    // Invariant: line_tuples is always sorted by offset, and the last tuple's line is always
    // less than or equal to the current line
    let last_tuple = chunk.line_tuples.last();
    match last_tuple {
        Some((_, last_line)) if *last_line == line as u8 => {
            // do nothing, the last tuple already has the same line
        }
        _ => {
            chunk.line_tuples.push((chunk.code.len() - 1, line as u8));
        }
    }
}

// Peform binary search to find the line number for a given offset
pub fn get_line(chunk: &Chunk, offset: usize) -> usize {
    let mut left = 0;
    let mut right = chunk.line_tuples.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let (mid_offset, mid_line) = chunk.line_tuples[mid];

        if offset < mid_offset {
            if mid == 0 {
                break; // Prevent underflow
            }
            right = mid - 1;
        } else if offset > mid_offset {
            left = mid + 1;
        } else {
            return mid_line as usize;
        }
    }

    // At this point, we have
    // offsets: [below offset] [above offset]
    //                   right  left
    // line_tuples[right].0 < offset
    // line_tuples[left].0 > offset
    return chunk.line_tuples[right].1 as usize;
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
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
            Ok(OpCode::Negate) => {
                println!("OP_NEGATE");
                offset += 1;
            }
            Ok(OpCode::Add) => {
                println!("OP_ADD");
                offset += 1;
            }
            Ok(OpCode::Subtract) => {
                println!("OP_SUBTRACT");
                offset += 1;
            }
            Ok(OpCode::Multiply) => {
                println!("OP_MULTIPLY");
                offset += 1;
            }
            Ok(OpCode::Divide) => {
                println!("OP_DIVIDE");
                offset += 1;
            }
            Err(()) => {
                unreachable!();
            }
        }
    }
}
