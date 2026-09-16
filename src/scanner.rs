pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    // TODO: remove source to save memory
    source: String,
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

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).is_none()
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
        self.current += 1;
        let character = self.source.chars().nth(self.current - 1).unwrap();
        character
    }

    pub fn match_character(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let character = self.source.chars().nth(self.current).unwrap();
        if character != expected {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    pub fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    pub fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }
        self.advance();
        self.make_token(TokenType::String)
    }

    pub fn number(&mut self) -> Token {
        while self.peek().is_digit(10) && !self.is_at_end() {
            self.advance();
        }
        // look for fractional part
        if self.peek() == '.' && !self.is_at_end() {
            self.advance();
            while self.peek().is_digit(10) && !self.is_at_end() {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn lexeme_is(&self, expected: &str) -> bool {
        let length = self.current - self.start;
        length == expected.chars().count()
            && self
                .source
                .chars()
                .skip(self.start)
                .take(length)
                .eq(expected.chars())
    }

    pub fn identifier_type(&self) -> TokenType {
        let length = self.current - self.start;
        let character = |offset: usize| self.source.chars().nth(self.start + offset);

        match character(0) {
            Some('a') if self.lexeme_is("and") => TokenType::And,
            Some('c') if self.lexeme_is("class") => TokenType::Class,
            Some('e') if self.lexeme_is("else") => TokenType::Else,
            Some('f') => match (length, character(1)) {
                (5, Some('a'))
                    if character(2) == Some('l')
                        && character(3) == Some('s')
                        && character(4) == Some('e') =>
                {
                    TokenType::False
                }
                (3, Some('o')) if character(2) == Some('r') => TokenType::For,
                (3, Some('u')) if character(2) == Some('n') => TokenType::Fun,
                _ => TokenType::Identifier,
            },
            Some('i') if self.lexeme_is("if") => TokenType::If,
            Some('n') if self.lexeme_is("nil") => TokenType::Nil,
            Some('o') if self.lexeme_is("or") => TokenType::Or,
            Some('p') if self.lexeme_is("print") => TokenType::Print,
            Some('r') if self.lexeme_is("return") => TokenType::Return,
            Some('s') if self.lexeme_is("super") => TokenType::Super,
            Some('t') => match (length, character(1)) {
                (4, Some('h')) if character(2) == Some('i') && character(3) == Some('s') => {
                    TokenType::This
                }
                (4, Some('r')) if character(2) == Some('u') && character(3) == Some('e') => {
                    TokenType::True
                }
                _ => TokenType::Identifier,
            },
            Some('v') if self.lexeme_is("var") => TokenType::Var,
            Some('w') if self.lexeme_is("while") => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    pub fn identifier(&mut self) -> Token {
        while !self.is_at_end() && self.peek().is_alphanumeric() {
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
                        while !self.is_at_end() && self.peek() != '\n' {
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
        let mut scanner = Scanner::new(source.to_string());
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
}
