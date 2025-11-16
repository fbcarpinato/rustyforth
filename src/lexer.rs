pub enum Token {
    StringLiteral(String), // e.g., "hello"
    NumberLiteral(f64),    // e.g., 123, 45.67
    BooleanLiteral(bool),  // e.g., true, false

    Identifier(String), // e.g., variable names, function names

    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /

    EOF, // End of file/input
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input.as_bytes()[self.position].is_ascii_whitespace()
        {
            self.position += 1;
        }
    }

    fn read_string(&mut self) -> Token {
        self.position += 1;
        let start = self.position;

        while self.position < self.input.len()
            && self.input.as_bytes()[self.position] as char != '"'
        {
            self.position += 1;
        }

        let string_literal = &self.input[start..self.position];
        self.position += 1;

        Token::StringLiteral(string_literal.to_string())
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input.as_bytes()[self.position] as char).is_ascii_digit()
        {
            self.position += 1;
        }

        if self.position < self.input.len() && self.input.as_bytes()[self.position] as char == '.' {
            self.position += 1;

            while self.position < self.input.len()
                && (self.input.as_bytes()[self.position] as char).is_ascii_digit()
            {
                self.position += 1;
            }
        }

        let number_literal = &self.input[start..self.position];
        let number_value: f64 = number_literal.parse().unwrap();

        Token::NumberLiteral(number_value)
    }

    fn read_boolean(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input.as_bytes()[self.position] as char).is_ascii_alphabetic()
        {
            self.position += 1;
        }

        let boolean_literal = &self.input[start..self.position];

        match boolean_literal {
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),
            _ => self.next_token(),
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input.as_bytes()[self.position] as char).is_ascii_alphanumeric()
            || (self.input.as_bytes()[self.position] as char) == '_'
        {
            self.position += 1;
        }

        let identifier = &self.input[start..self.position];

        Token::Identifier(identifier.to_string())
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.input.as_bytes()[self.position] as char;

        match current_char {
            '"' => self.read_string(),
            '0'..='9' => self.read_number(),
            't' | 'f' => self.read_boolean(),
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                if self.position + 1 < self.input.len() {
                    let next_char = self.input.as_bytes()[self.position + 1] as char;
                    if next_char.is_ascii_digit() {
                        self.position += 1;
                        let number_token = self.read_number();
                        if let Token::NumberLiteral(num) = number_token {
                            return Token::NumberLiteral(-num);
                        }
                    }
                }

                self.position += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                Token::Asterisk
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            c if c.is_alphabetic() || c == '_' => self.read_identifier(),
            _ => {
                self.position += 1;
                self.next_token()
            }
        }
    }
}
