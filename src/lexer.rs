use crate::token::*;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.into_bytes();

        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "false" => Token::False,
                    "true" => Token::True,
                    "return" => Token::Return,
                    "else" => Token::Else,
                    _ => Token::Ident(ident),
                }
            }
            b'0'..=b'9' => Token::Int(self.read_int()),
            b'0' => Token::Eof,
            _ => Token::Invalid,
        };

        self.read_char();

        token
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y
        };
        
        let result = add(five, ten)"#;

        let mut lexer = Lexer::new(input.into());
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = lexer.next_token();

            match token {
                Token::Eof | Token::Invalid => break,
                _ => tokens.push(token),
            }
        }

        let expected: Vec<Token> = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Eof,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_read_ident() {
        let input = "=+abc(){";
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(input.into());

        for _ in 0..8 {
            tokens.push(lexer.next_token());
        }

        assert_eq!(tokens[0], Token::Assign);
        assert_eq!(tokens[1], Token::Plus);
        assert_eq!(tokens[2], Token::Ident("abc".into()));
        assert_eq!(tokens[3], Token::Rparen);
        assert_eq!(tokens[4], Token::Lbrace);
    }

    #[test]
    fn test_read_int() {
        let input = "=+123(){";
        let mut tokens: Vec<Token> = vec![];
        let mut lexer = Lexer::new(input.into());

        for _ in 0..8 {
            tokens.push(lexer.next_token());
        }

        assert_eq!(tokens[0], Token::Assign);
        assert_eq!(tokens[1], Token::Plus);
        assert_eq!(tokens[2], Token::Int("123".into()));
        assert_eq!(tokens[3], Token::Rparen);
        assert_eq!(tokens[4], Token::Lbrace);
    }
}
