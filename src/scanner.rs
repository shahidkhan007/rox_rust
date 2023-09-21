use crate::{
    error::Log,
    keywords::get_keywords,
    token::{self, Literal, Token, TokenType},
};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,

    logger: Log,
}

impl Scanner {
    pub fn new(source: String, logger: Log) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            logger,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line,
            Literal::Nil,
        ));
    }

    fn scan(&mut self) {
        let c = self.advance();

        let token_type = self.get_token_type(c.clone());

        match token_type {
            None => return,
            Some(token_type) => {
                let s = self.start as usize;
                let e = self.current as usize;

                let lexeme = &self.source[s..e].to_string();

                let new_token = Token::new(token_type, lexeme.to_owned(), self.line, Literal::Nil);

                self.tokens.push(new_token);
            }
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current as usize >= self.source.len();
    }

    fn advance(&mut self) -> String {
        let c = self
            .source
            .chars()
            .nth(self.current as usize)
            .unwrap()
            .to_string();

        self.current += 1;
        return c;
    }

    fn peek(&self) -> String {
        if self.is_at_end() {
            return "\0".to_string();
        }

        self.source
            .chars()
            .nth(self.current as usize)
            .unwrap()
            .to_string()
    }

    fn get_token_type(&mut self, character: String) -> Option<TokenType> {
        match character.as_str() {
            "(" => Some(TokenType::LEFT_PAREN),
            ")" => Some(TokenType::RIGHT_PAREN),
            "{" => Some(TokenType::LEFT_BRACE),
            "}" => Some(TokenType::RIGHT_BRACE),
            "," => Some(TokenType::COMMA),
            "." => Some(TokenType::DOT),
            "-" => Some(TokenType::MINUS),
            "+" => Some(TokenType::PLUS),
            ";" => Some(TokenType::SEMICOLON),
            "*" => Some(TokenType::STAR),
            "!" => {
                if self.match_char("=".to_string()) {
                    Some(TokenType::BANG_EQUAL)
                } else {
                    Some(TokenType::BANG)
                }
            }
            "=" => {
                if self.match_char("=".to_string()) {
                    Some(TokenType::EQUAL_EQUAL)
                } else {
                    Some(TokenType::EQUAL)
                }
            }
            "<" => {
                if self.match_char("=".to_string()) {
                    Some(TokenType::LESS_EQUAL)
                } else {
                    Some(TokenType::LESS)
                }
            }
            ">" => {
                if self.match_char("=".to_string()) {
                    Some(TokenType::GREATER_EQUAL)
                } else {
                    Some(TokenType::GREATER)
                }
            }
            "/" => {
                if self.match_char("/".to_string()) {
                    while self.peek() != "\n".to_string() && !self.is_at_end() {
                        self.advance();
                    }

                    None
                } else if self.match_char("*".to_string()) {
                    self.parse_block_comments();
                    None
                } else {
                    Some(TokenType::SLASH)
                }
            }
            " " => None,
            "\r" => None,
            "\t" => None,
            "\n" => {
                self.line += 1;
                None
            }
            "\"" => self.parse_string(),

            x => {
                if self.is_digit(x) {
                    self.parse_number();
                    return None;
                }

                if self.is_alphanumeric(x) {
                    self.parse_identifier();
                    return None;
                }

                self.logger.error(format!(
                    "Syntax Error: Unidentified character '{}' at line {}",
                    self.source
                        .chars()
                        .nth((self.current - 1) as usize)
                        .unwrap(),
                    self.line
                ));
                panic!();
            }
        }
    }

    fn match_char(&mut self, expected: String) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            let c = self.peek();

            if c != expected {
                return false;
            } else {
                self.current += 1;
                return true;
            }
        }
    }

    fn parse_string(&mut self) -> Option<TokenType> {
        while self.peek() != "\"".to_string() && !self.is_at_end() {
            if self.peek() == "\n".to_string() {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.logger
                .error("Syntax Error: Unterminated string".to_string());
            return None;
        }

        self.advance();

        let str_val = self.source[self.start + 1..self.current - 1].to_owned();

        let new_token = Token::new(
            TokenType::STRING,
            "".to_string(),
            self.line,
            Literal::String(str_val),
        );

        self.tokens.push(new_token);

        return None;
    }

    fn is_digit(&self, x: &str) -> bool {
        return x.to_string().parse::<i32>().is_ok();
    }

    fn peek_next(&self) -> String {
        if self.is_at_end() || self.current + 1 >= self.source.len() {
            return "\0".to_string();
        }

        self.source
            .chars()
            .nth(self.current + 1)
            .unwrap()
            .to_string()
    }

    fn parse_number(&mut self) {
        while self.is_digit(&self.peek()) {
            self.advance();
        }

        if self.peek() == ".".to_string() && self.is_digit(&self.peek_next()) {
            self.advance();

            while self.is_digit(&self.peek()) {
                self.advance();
            }
        }

        let number_val = self.source[self.start..self.current]
            .to_owned()
            .parse::<f64>()
            .unwrap();

        let new_token = Token::new(
            TokenType::NUMBER,
            "".to_string(),
            self.line,
            Literal::Number(number_val),
        );

        self.tokens.push(new_token);
    }

    fn is_alpha(&self, x: &str) -> bool {
        let utf8_code = x.bytes().next().unwrap();

        if (utf8_code > 64 && utf8_code < 65)
            || (utf8_code > 96 && utf8_code < 123)
            || utf8_code == 95
        {
            return true;
        } else {
            return false;
        }
    }

    fn is_alphanumeric(&self, x: &str) -> bool {
        self.is_alpha(x) || self.is_digit(x)
    }

    fn parse_identifier(&mut self) {
        let keywords = get_keywords();

        while self.is_alphanumeric(&self.peek()) {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];

        let mut token_type = keywords.get(lexeme);

        if token_type.is_none() {
            token_type = Some(&TokenType::IDENTIFIER);
        }

        let new_token = Token::new(
            token_type.unwrap().to_owned(),
            lexeme.to_string(),
            self.line,
            Literal::Nil,
        );

        self.tokens.push(new_token);
    }

    fn parse_block_comments(&mut self) {
        while self.peek() != "*" && self.peek_next() != "/" {
            let ch = self.advance();

            if ch == "\n" {
                self.line += 1;
            }
        }

        self.advance();
        self.advance();
    }
}
