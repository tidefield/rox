pub struct Scanner<'s> {
    start: usize,
    current: usize,
    line: usize,
    // This remains valid for as long as the scanner exists.
    source: &'s str,
}

#[derive(Debug, Clone, PartialEq)]
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
    EOF,
}

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
            source,
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

    pub fn advance(&mut self) -> char {
        let character = self.peek().expect("advance called at end of input");
        self.current += character.len_utf8();
        character
    }

    pub fn match_character(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != Some(expected) {
            return false;
        }

        self.advance();
        true
    }

    pub fn peek(&self) -> Option<char> {
        self.source
            .get(self.current..)
            .and_then(|remaining| remaining.chars().next())
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek() {
            if !character.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    pub fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != Some('"') {
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
        while matches!(self.peek(), Some(character) if character.is_digit(10)) {
            self.advance();
        }
        // look for fractional part
        if self.peek() == Some('.') {
            self.advance();
            while matches!(self.peek(), Some(character) if character.is_digit(10)) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn lexeme_is(&self, expected: &str) -> bool {
        self.source.get(self.start..self.current) == Some(expected)
    }

    pub fn identifier_type(&self) -> TokenType {
        let lexeme = self.source.get(self.start..self.current).unwrap_or("");

        match lexeme.chars().next() {
            Some('a') if self.lexeme_is("and") => TokenType::And,
            Some('c') if self.lexeme_is("class") => TokenType::Class,
            Some('e') if self.lexeme_is("else") => TokenType::Else,
            Some('f') => match lexeme {
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                _ => TokenType::Identifier,
            },
            Some('i') if self.lexeme_is("if") => TokenType::If,
            Some('n') if self.lexeme_is("nil") => TokenType::Nil,
            Some('o') if self.lexeme_is("or") => TokenType::Or,
            Some('p') if self.lexeme_is("print") => TokenType::Print,
            Some('r') if self.lexeme_is("return") => TokenType::Return,
            Some('s') if self.lexeme_is("super") => TokenType::Super,
            Some('t') => match lexeme {
                "this" => TokenType::This,
                "true" => TokenType::True,
                _ => TokenType::Identifier,
            },
            Some('v') if self.lexeme_is("var") => TokenType::Var,
            Some('w') if self.lexeme_is("while") => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    pub fn identifier(&mut self) -> Token {
        while let Some(character) = self.peek() {
            if !character.is_alphanumeric() {
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
            let character = self.advance();

            if character.is_digit(10) {
                return self.number();
            }

            if character.is_alphabetic() {
                return self.identifier();
            }

            let token = match character {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '-' => self.make_token(TokenType::Minus),
                '+' => self.make_token(TokenType::Plus),
                ';' => self.make_token(TokenType::Semicolon),
                '/' => {
                    if self.match_character('/') {
                        while !self.is_at_end() && self.peek() != Some('\n') {
                            self.advance();
                        }
                        continue;
                    }
                    self.make_token(TokenType::Slash)
                }
                '*' => self.make_token(TokenType::Star),
                '!' => {
                    if self.match_character('=') {
                        self.make_token(TokenType::BangEqual)
                    } else {
                        self.make_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.match_character('=') {
                        self.make_token(TokenType::EqualEqual)
                    } else {
                        self.make_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.match_character('=') {
                        self.make_token(TokenType::LessEqual)
                    } else {
                        self.make_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.match_character('=') {
                        self.make_token(TokenType::GreaterEqual)
                    } else {
                        self.make_token(TokenType::Greater)
                    }
                }
                '"' => self.string(),
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
    fn scans_unicode_identifiers_with_byte_offsets() {
        let source = "é + 中";
        let mut scanner = Scanner::new(source);

        let identifier = scanner.scan_token();
        assert_eq!(identifier.kind, TokenType::Identifier);
        assert_eq!(
            &source[identifier.start..identifier.start + identifier.length],
            "é"
        );
        assert_eq!(identifier.length, "é".len());

        let plus = scanner.scan_token();
        assert_eq!(plus.kind, TokenType::Plus);

        let second_identifier = scanner.scan_token();
        assert_eq!(second_identifier.kind, TokenType::Identifier);
        assert_eq!(
            &source[second_identifier.start..second_identifier.start + second_identifier.length],
            "中"
        );
    }
}
