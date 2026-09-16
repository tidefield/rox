use crate::scanner::{Scanner, Token, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
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
