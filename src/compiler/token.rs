use phf::phf_map;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Ident(String),
    Number(u32),
    Const,
    Var,
    Procedure,
    Call,
    Begin,
    End,
    If,
    Then,
    While,
    Do,
    Odd,
    Assign,
    Equal,
    Hash,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Dot,
    Comma,
    SemiColon,
    Question,
    Exclamation
}

#[derive(PartialEq, Debug)]
pub struct Token(pub TokenType);

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "CONST" => TokenType::Const,
    "VAR" => TokenType::Var,
    "PROCEDURE" => TokenType::Procedure,
    "CALL" => TokenType::Call,
    "BEGIN" => TokenType::Begin,
    "END" => TokenType::End,
    "IF" => TokenType::If,
    "THEN" => TokenType::Then,
    "WHILE" => TokenType::While,
    "DO" => TokenType::Do,
    "ODD" => TokenType::Odd
};

pub fn parse_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).cloned()
}