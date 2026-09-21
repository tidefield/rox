use crate::{
    chunk::{Chunk, OpCode},
    parser::Parser,
    scanner::Scanner,
};

// `'s` is the lifetime of the borrowed input. The scanner cannot outlive
// this borrow, so `source` must remain valid while the scanner is in use.
pub fn compile<'s>(source: &'s str) -> Option<Chunk> {
    let scanner = Scanner::new(source);
    let mut parser = Parser::new(scanner);
    let mut chunk = Chunk::new();

    parser.parse(source, &mut chunk)?;
    chunk.code.push(OpCode::Return as u8);

    Some(chunk)
}

#[cfg(test)]
mod tests {
    use super::compile;
    use crate::chunk::OpCode;

    #[test]
    fn compiles_operator_precedence() {
        let chunk = compile("1 + 2 * 3").expect("expression should compile");

        assert_eq!(chunk.constants, vec![1.0, 2.0, 3.0]);
        assert_eq!(
            chunk.code,
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Constant as u8,
                2,
                OpCode::Multiply as u8,
                OpCode::Add as u8,
                OpCode::Return as u8,
            ]
        );
    }

    #[test]
    fn compiles_grouping_and_unary_minus() {
        let chunk = compile("-(1 + 2)").expect("expression should compile");

        assert_eq!(chunk.constants, vec![1.0, 2.0]);
        assert_eq!(
            chunk.code,
            vec![
                OpCode::Constant as u8,
                0,
                OpCode::Constant as u8,
                1,
                OpCode::Add as u8,
                OpCode::Negate as u8,
                OpCode::Return as u8,
            ]
        );
    }
}
