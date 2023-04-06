use std::str;

use super::token::{self, Token, TokenType};

pub fn lex(buf: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut idx = 0;
    while idx < buf.len() {
        let mut c = buf[idx] as char;

        if c.is_whitespace() {
            idx += 1;
            continue;
        }

        if c.is_ascii_punctuation() && c != '_'{
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
                if idx >= buf.len() {
                    break idx;
                }

                c = buf[idx] as char;
                if !c.is_ascii_digit()
                {
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
                if idx >= buf.len() {
                    break idx;
                }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation_parsing() {
        lex_test("#", TokenType::Hash);
        lex_test("<", TokenType::LessThan);
        lex_test(">", TokenType::GreaterThan);
        lex_test(">=", TokenType::GreaterThanEqual);
        lex_test("<=", TokenType::LessThanEqual);
        lex_test(":=", TokenType::Assign);
    }
    
    #[test]
    fn digit_parsing() {
        lex_test("123456789", TokenType::Number(123456789));
        lex_test("0", TokenType::Number(0));
        lex_test(&u32::MAX.to_string(), TokenType::Number(u32::MAX));
    }

    #[test]
    fn unsupported_number_representations() {
        let tokens = lex("3.14".as_bytes());
        assert_eq!(tokens, vec![
            Token(TokenType::Number(3)), 
            Token(TokenType::Dot), 
            Token(TokenType::Number(14))
        ]);
        
        let tokens = lex("0x1234abcd".as_bytes());
        assert_eq!(tokens, vec![
            Token(TokenType::Number(0)),
            Token(TokenType::Ident("x1234abcd".to_string())),
        ]);
    }

    #[test]
    fn ident_parsing() {
        lex_test("foo", TokenType::Ident("foo".to_string()));
        lex_test("__dunder__", TokenType::Ident("__dunder__".to_string()));
        lex_test("sneaky_snake", TokenType::Ident("sneaky_snake".to_string()));
        lex_test("alphanum3r1c", TokenType::Ident("alphanum3r1c".to_string()));
        lex_test("var", TokenType::Ident("var".to_string()));
    }
    
    fn lex_test(test_str: &str, expected_token_type: TokenType) {
        assert_eq!(vec![Token(expected_token_type)], lex(test_str.as_bytes()));
    }

    #[test]
    #[should_panic]
    fn stray_colon_panics() {
        let _ = lex(": =".as_bytes());
    }
}