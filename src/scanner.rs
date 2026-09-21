pub struct Scanner<'s> {
    start: usize,
    current: usize,
    line: usize,
    // This remains valid for as long as the scanner exists.
    source: &'s [u8],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One- or two-character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    #[default]
    EOF,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token {
    pub kind: TokenType,
    // pub text: String,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

pub enum TokenKind {}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source: source.as_bytes(),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn make_token(&self, kind: TokenType) -> Token {
        Token {
            kind,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    pub fn error_token(&self, message: String) -> Token {
        Token {
            kind: TokenType::Error,
            start: self.start,
            length: message.len(),
            line: self.line,
        }
    }

    pub fn advance(&mut self) -> u8 {
        let byte = self.peek().expect("advance called at end of input");
        self.current += 1;
        byte
    }

    pub fn match_character(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != Some(expected) {
            return false;
        }

        self.advance();
        true
    }

    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }

    pub fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != Some(b'"') {
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string".to_string())
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    pub fn number(&mut self) -> Token {
        while matches!(self.peek(), Some(byte) if byte.is_ascii_digit()) {
            self.advance();
        }
        // look for fractional part
        if self.peek() == Some(b'.') {
            self.advance();
            while matches!(self.peek(), Some(byte) if byte.is_ascii_digit()) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn lexeme_is(&self, expected: &[u8]) -> bool {
        self.source.get(self.start..self.current) == Some(expected)
    }

    pub fn identifier_type(&self) -> TokenType {
        let first = self.source.get(self.start).copied();

        match first {
            Some(b'a') if self.lexeme_is(b"and") => TokenType::And,
            Some(b'c') if self.lexeme_is(b"class") => TokenType::Class,
            Some(b'e') if self.lexeme_is(b"else") => TokenType::Else,
            Some(b'f') if self.lexeme_is(b"false") => TokenType::False,
            Some(b'f') if self.lexeme_is(b"for") => TokenType::For,
            Some(b'f') if self.lexeme_is(b"fun") => TokenType::Fun,
            Some(b'i') if self.lexeme_is(b"if") => TokenType::If,
            Some(b'n') if self.lexeme_is(b"nil") => TokenType::Nil,
            Some(b'o') if self.lexeme_is(b"or") => TokenType::Or,
            Some(b'p') if self.lexeme_is(b"print") => TokenType::Print,
            Some(b'r') if self.lexeme_is(b"return") => TokenType::Return,
            Some(b's') if self.lexeme_is(b"super") => TokenType::Super,
            Some(b't') if self.lexeme_is(b"this") => TokenType::This,
            Some(b't') if self.lexeme_is(b"true") => TokenType::True,
            Some(b'v') if self.lexeme_is(b"var") => TokenType::Var,
            Some(b'w') if self.lexeme_is(b"while") => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    pub fn identifier(&mut self) -> Token {
        while let Some(byte) = self.peek() {
            if !byte.is_ascii_alphanumeric() {
                break;
            }
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    pub fn scan_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            self.start = self.current;

            if self.is_at_end() {
                return self.make_token(TokenType::EOF);
            }
            let byte = self.advance();

            if byte.is_ascii_digit() {
                return self.number();
            }

            if byte.is_ascii_alphabetic() {
                return self.identifier();
            }

            let token = match byte {
                b'(' => self.make_token(TokenType::LeftParen),
                b')' => self.make_token(TokenType::RightParen),
                b'{' => self.make_token(TokenType::LeftBrace),
                b'}' => self.make_token(TokenType::RightBrace),
                b',' => self.make_token(TokenType::Comma),
                b'.' => self.make_token(TokenType::Dot),
                b'-' => self.make_token(TokenType::Minus),
                b'+' => self.make_token(TokenType::Plus),
                b';' => self.make_token(TokenType::Semicolon),
                b'/' => {
                    if self.match_character(b'/') {
                        while !self.is_at_end() && self.peek() != Some(b'\n') {
                            self.advance();
                        }
                        continue;
                    }
                    self.make_token(TokenType::Slash)
                }
                b'*' => self.make_token(TokenType::Star),
                b'!' => {
                    if self.match_character(b'=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                b'=' => {
                    if self.match_character(b'=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                b'<' => {
                    if self.match_character(b'=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                b'>' => {
                    if self.match_character(b'=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                b'"' => self.string(),
                _ => self.error_token("Unexpected character".to_string()),
            };

            return token;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, TokenType};

    fn scan_kinds(source: &str) -> Vec<TokenType> {
        let mut scanner = Scanner::new(source);
        let mut kinds = Vec::new();

        loop {
            let token = scanner.scan_token();
            let is_eof = token.kind == TokenType::EOF;
            kinds.push(token.kind);

            if is_eof {
                return kinds;
            }
        }
    }

    #[test]
    fn recognizes_keywords_and_identifier_fallback() {
        let source = "and class else false for fun if nil or print return super this true var while andrew classy";

        assert_eq!(
            scan_kinds(source),
            vec![
                TokenType::And,
                TokenType::Class,
                TokenType::Else,
                TokenType::False,
                TokenType::For,
                TokenType::Fun,
                TokenType::If,
                TokenType::Nil,
                TokenType::Or,
                TokenType::Print,
                TokenType::Return,
                TokenType::Super,
                TokenType::This,
                TokenType::True,
                TokenType::Var,
                TokenType::While,
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn recognizes_compound_operators_and_skips_comments() {
        let source = "!= == <= >= / // ignored\n!";

        assert_eq!(
            scan_kinds(source),
            vec![
                TokenType::BangEqual,
                TokenType::EqualEqual,
                TokenType::LessEqual,
                TokenType::GreaterEqual,
                TokenType::Slash,
                TokenType::Bang,
                TokenType::EOF,
            ]
        );
    }

    #[test]
    fn rejects_non_ascii_input() {
        let source = "é";
        let mut scanner = Scanner::new(source);

        assert_eq!(scanner.scan_token().kind, TokenType::Error);
        assert_eq!(scanner.scan_token().kind, TokenType::Error);
        assert_eq!(scanner.scan_token().kind, TokenType::EOF);
    }
}
