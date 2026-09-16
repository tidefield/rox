use rox::scanner::{Scanner, TokenType};
use std::hint::black_box;
use std::time::Instant;

const REPETITIONS: usize = 10_000;
const PROGRAM: &str = "var total = 1 + 2 * 3; // scanner benchmark\nprint total;\n";

fn make_source() -> String {
    let mut source = String::with_capacity(PROGRAM.len() * REPETITIONS);

    for _ in 0..REPETITIONS {
        source.push_str(PROGRAM);
    }

    source
}

fn scan_source(source: &str) -> (u128, usize) {
    let started = Instant::now();
    let mut scanner = Scanner::new(source);
    let mut token_count = 0;

    loop {
        let token = scanner.scan_token();
        token_count += 1;

        if token.kind == TokenType::EOF {
            break;
        }
    }

    (started.elapsed().as_nanos(), black_box(token_count))
}

fn main() {
    let source = make_source();
    let (elapsed_nanos, token_count) = scan_source(&source);

    println!("source bytes: {0}", source.len());
    println!("tokens:       {token_count}");
    println!("scan time:    {elapsed_nanos} ns");
}
