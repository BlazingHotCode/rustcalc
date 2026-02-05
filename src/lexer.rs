use crate::error::CalcError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Ident(String),
    DecimalPoint,
    Comma,
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    OpenParen,
    CloseParen,
    EOF,
}

pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    ident.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Ident(ident));
                continue;
            }
            '0'..='9' => {
                let mut num = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num = num * 10 + chars[i].to_digit(10).unwrap() as i32;
                    i += 1;
                }
                tokens.push(Token::Number(num));
                continue;
            }
            '.' => tokens.push(Token::DecimalPoint),
            ',' => tokens.push(Token::Comma),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Mult),
            '/' => tokens.push(Token::Div),
            '^' => tokens.push(Token::Pow),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ' ' => {} // Ignore whitespace
            other => return Err(CalcError::UnexpectedChar(other)),
        }
        i += 1;
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}
