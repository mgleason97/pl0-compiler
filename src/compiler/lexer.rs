use std::str;

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
            tokens.push(match c {
                '#' => Token(TokenType::Hash),
                '+' => Token(TokenType::Plus),
                '-' => Token(TokenType::Minus),
                '*' => Token(TokenType::Multiply),
                '/' => Token(TokenType::Divide),
                '(' => Token(TokenType::LParen),
                ')' => Token(TokenType::RParen),
                '.' => Token(TokenType::Dot),
                ',' => Token(TokenType::Comma),
                ';' => Token(TokenType::SemiColon),
                '?' => Token(TokenType::Question),
                '!' => Token(TokenType::Exclamation),
                '=' => Token(TokenType::Equal),
                ':' if idx + 1 < buf.len() && buf[idx+1] as char == '=' => {
                    idx += 1;
                    Token(TokenType::Assign)
                },
                '<' => if idx + 1 < buf.len() && buf[idx+1] as char == '=' {
                    idx += 1;
                    Token(TokenType::LessThanEqual)
                }  else {
                    Token(TokenType::LessThan)
                },
                '>' => if idx + 1 < buf.len() && buf[idx+1] as char == '=' {
                    idx += 1;
                    Token(TokenType::GreaterThanEqual)
                }  else {
                    Token(TokenType::GreaterThan)
                },
                _ => panic!("Invalid syntax {:?}", c)
            });

            idx+=1;
        } else if c.is_ascii_digit() {
            let start = idx;
            let end = loop {
                idx += 1;
                c = buf[idx] as char;
                if !c.is_ascii_digit() {
                    break idx;
                }
            };
            let slice = str::from_utf8(&buf[start..end]).expect("Invalid utf8");
            let num = u32::from_str_radix(slice, 10).expect("Could not parse number");
            tokens.push(Token(TokenType::Number(num)));
        } else if c.is_alphabetic() || c == '_' {
            let start = idx;
            let end = loop {
                idx += 1;
                c = buf[idx] as char;
                if !c.is_alphanumeric() && c != '_'  {
                    break idx;
                }
            };

            let slice = str::from_utf8(&buf[start..end]).expect("Invalid utf8");
            let word = String::from(slice);

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