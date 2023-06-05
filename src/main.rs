mod lexer;
mod token;

use lexer::Lexer;
use token::Token;

fn main() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y
        };
        
        let result = add(five, ten)";

        let mut lexer = Lexer::new(input.into());
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = lexer.next_token();
            match token {
                Token::Eof => break,
                _ => tokens.push(token),
        }
    }
}