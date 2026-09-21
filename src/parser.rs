use crate::{
    chunk::{add_constant, write_chunk, Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Term,
    Factor,
    Unary,
}

pub struct Parser<'s> {
    scanner: Scanner<'s>,
    current: Token,
    previous: Token,
}

impl<'s> Parser<'s> {
    pub fn new(scanner: Scanner<'s>) -> Self {
        Self {
            scanner,
            current: Token::default(),
            previous: Token::default(),
        }
    }

    pub fn parse(&mut self, source: &str, chunk: &mut Chunk) -> Option<()> {
        self.advance()?;
        self.expression(source, chunk)?;
        self.consume(TokenType::EOF)
    }

    fn advance(&mut self) -> Option<()> {
        let token = self.scanner.scan_token();
        if token.kind == TokenType::Error {
            return None;
        }

        self.previous = std::mem::replace(&mut self.current, token);
        Some(())
    }

    fn consume(&mut self, kind: TokenType) -> Option<()> {
        if self.current.kind == kind {
            self.advance()
        } else {
            None
        }
    }

    fn expression(&mut self, source: &str, chunk: &mut Chunk) -> Option<()> {
        self.parse_precedence(Precedence::Assignment, source, chunk)
    }

    fn parse_precedence(
        &mut self,
        precedence: Precedence,
        source: &str,
        chunk: &mut Chunk,
    ) -> Option<()> {
        self.advance()?;

        let prefix = get_rule(&self.previous.kind).prefix?;
        prefix(self, source, chunk)?;

        while precedence <= get_rule(&self.current.kind).precedence {
            self.advance()?;
            let infix = get_rule(&self.previous.kind).infix?;
            infix(self, source, chunk)?;
        }

        Some(())
    }
}

type ParseFn = for<'s> fn(&mut Parser<'s>, &str, &mut Chunk) -> Option<()>;

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

fn get_rule(kind: &TokenType) -> ParseRule {
    match kind {
        TokenType::LeftParen => ParseRule {
            prefix: Some(grouping),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::Minus => ParseRule {
            prefix: Some(unary),
            infix: Some(binary),
            precedence: Precedence::Term,
        },
        TokenType::Plus => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Term,
        },
        TokenType::Slash | TokenType::Star => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Factor,
        },
        TokenType::Number => ParseRule {
            prefix: Some(number),
            infix: None,
            precedence: Precedence::None,
        },
        _ => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    }
}

fn number(parser: &mut Parser<'_>, source: &str, chunk: &mut Chunk) -> Option<()> {
    let token = &parser.previous;
    let end = token.start + token.length;
    let lexeme = source.get(token.start..end)?;
    let value = lexeme.parse::<f64>().ok()?;
    let constant_index = add_constant(chunk, value);

    write_chunk(chunk, OpCode::Constant.into(), token.line);
    write_chunk(chunk, constant_index as u8, token.line);
    Some(())
}

fn grouping(parser: &mut Parser<'_>, source: &str, chunk: &mut Chunk) -> Option<()> {
    parser.expression(source, chunk)?;
    parser.consume(TokenType::RightParen)
}

fn unary(parser: &mut Parser<'_>, source: &str, chunk: &mut Chunk) -> Option<()> {
    let operator = parser.previous.kind.clone();
    let line = parser.previous.line;

    parser.parse_precedence(Precedence::Unary, source, chunk)?;

    if operator == TokenType::Minus {
        write_chunk(chunk, OpCode::Negate.into(), line);
        Some(())
    } else {
        None
    }
}

fn binary(parser: &mut Parser<'_>, source: &str, chunk: &mut Chunk) -> Option<()> {
    let operator = parser.previous.kind.clone();
    let line = parser.previous.line;
    let precedence = get_rule(&operator).precedence;
    let next_precedence = match precedence {
        Precedence::Term => Precedence::Factor,
        Precedence::Factor => Precedence::Unary,
        _ => return None,
    };

    parser.parse_precedence(next_precedence, source, chunk)?;

    let opcode = match operator {
        TokenType::Plus => OpCode::Add,
        TokenType::Minus => OpCode::Subtract,
        TokenType::Star => OpCode::Multiply,
        TokenType::Slash => OpCode::Divide,
        _ => return None,
    };
    write_chunk(chunk, opcode.into(), line);
    Some(())
}
