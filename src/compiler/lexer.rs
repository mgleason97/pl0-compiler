use super::token::{self, Token, TokenType};

pub fn lex(buf: &mut [u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut idx = 0;
    while idx < buf.len() {
        let mut c = buf[idx] as char;

        if c.is_whitespace() {
            idx += 1;
            continue;
        }

        if c.is_ascii_punctuation() {
            if c == '#' {
                tokens.push(Token(TokenType::Hash));
            } else if c == '+' {
                tokens.push(Token(TokenType::Plus));
            } else if c == '-' {
                tokens.push(Token(TokenType::Minus));
            } else if c == '*' {
                tokens.push(Token(TokenType::Multiply));
            } else if c == '/' {
                tokens.push(Token(TokenType::Divide));
            } else if c == '(' {
                tokens.push(Token(TokenType::LParen));
            } else if c == ')' {
                tokens.push(Token(TokenType::RParen));
            } else if c == '.' {
                tokens.push(Token(TokenType::Dot));
            } else if c == ',' {
                tokens.push(Token(TokenType::Comma));
            } else if c == ';' {
                tokens.push(Token(TokenType::SemiColon));
            } else if c == '?' {
                tokens.push(Token(TokenType::Question));
            } else if c == '!' {
                tokens.push(Token(TokenType::Exclamation));
            } else if c == '=' {
                tokens.push(Token(TokenType::Equal));
            } else if c == ':' {
                if idx + 1 < buf.len() && buf[idx+1] as char == '=' {
                    tokens.push(Token(TokenType::Assign));
                    idx += 1;
                } else {
                    panic!("Invalid syntax");
                }
            } else if c == '<' {
                if idx + 1 < buf.len() && buf[idx+1] as char == '=' {
                    tokens.push(Token(TokenType::LessThanEqual));
                    idx += 1;
                } else {
                    tokens.push(Token(TokenType::LessThan));
                }
            } else if c == '>' {
                if idx + 1 < buf.len() && buf[idx+1] as char == '=' {
                    tokens.push(Token(TokenType::GreaterThanEqual));
                    idx += 1;
                } else {
                    tokens.push(Token(TokenType::GreaterThan));
                }
            }

            idx+=1;
        } else if c.is_ascii_digit() {
            let mut digit_buf = Vec::<u32>::new();
            while c.is_ascii_digit() {
                digit_buf.push(c.to_digit(10).expect("Safe to convert ascii digit to base 10 digit"));
                idx += 1;
                c = buf[idx] as char;
            }
            let num = digit_buf.iter().fold(0, |acc, &x| acc * 10 + x);
            tokens.push(Token(TokenType::Number(num)));
        } else if c.is_alphabetic() || c == '_' {
            let mut char_buf = Vec::<char>::new();
            while c.is_alphanumeric() || c == '_' {
                char_buf.push(c);
                idx += 1;
                c = buf[idx] as char;
            }
            let word = String::from_iter(char_buf.iter());

            if let Some(reserved_token) = token::parse_keyword(&word) {
                tokens.push(Token(reserved_token));
            } else {
                tokens.push(Token(TokenType::Ident(word)));
            }
        } else {
            panic!("Ahh! How do I handle this? {}", c);
        }
    }

    tokens
}