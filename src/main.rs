use std::{io};

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    OpenParen,
    CloseParen,
    EOF,
}

fn main() {
    loop {
        let input = read_input();

        if input == "exit" {
            break;
        }

        let tokens: Vec<Token> = parse_input(&input);

        println!("You entered: {}", input);
        println!("Tokens: {:?}", tokens);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_input(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '0'..='9' => {
                let mut num = 0;
                while i < chars.len() && chars[i].is_digit(10) {
                    num = num * 10 + chars[i].to_digit(10).unwrap() as i32;
                    i += 1;
                }
                tokens.push(Token::Number(num));
                continue; // Skip the increment at the end of the loop
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ' ' => {}, // Ignore whitespace
            _ => panic!("Unexpected character: {}", chars[i]),
        }
        i += 1;
    }

    tokens.push(Token::EOF);
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "12 + 34 - 5";
        let expected_tokens = vec![
            Token::Number(12),
            Token::Plus,
            Token::Number(34),
            Token::Minus,
            Token::Number(5),
            Token::EOF,
        ];
        assert_eq!(parse_input(input), expected_tokens);
    }
}