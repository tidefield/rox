use crate::scanner::{Scanner, Token, TokenType};

// `'s` is the lifetime of the borrowed input. The scanner cannot outlive
// this borrow, so `source` must remain valid while the scanner is in use.
pub fn compile<'s>(source: &'s str) {
    let mut scanner: Scanner<'s> = Scanner::new(source);
    let mut line: usize = 0;
    // Scan tokens until EOF
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            println!("{}", token.line);
            line = token.line;
        } else {
            print!(" |");
        }

        println!("{:?} {} {}", token.kind, token.length, token.start);

        if token.kind == TokenType::EOF {
            break;
        }
    }
}
