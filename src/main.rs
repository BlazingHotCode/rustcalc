use std::{io};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    DecimalPoint,
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    OpenParen,
    CloseParen,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Number(f64),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Exponentiation(Box<Expression>, Box<Expression>),
    Parenthesis(Box<Expression>),
}

fn main() {
    loop {
        let input = read_input();

        if input == "exit" {
            break;
        }

        let tokens: Vec<Token> = parse_input(&input);

        parse_expression(tokens);
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
            '.' => tokens.push(Token::DecimalPoint),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Mult),
            '/' => tokens.push(Token::Div),
            '^' => tokens.push(Token::Pow),
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

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn bump(&mut self) -> Token {
        let token = self.peek().clone();
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) {
        let got = self.bump();
        if got != expected {
            panic!("Expected {:?}, got {:?}", expected, got);
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Expression {
        let mut left = self.parse_mul_div();
        loop {
            match self.peek() {
                Token::Plus => {
                    self.bump();
                    let right = self.parse_mul_div();
                    left = Expression::Addition(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.bump();
                    let right = self.parse_mul_div();
                    left = Expression::Subtraction(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_mul_div(&mut self) -> Expression {
        let mut left = self.parse_unary();
        loop {
            match self.peek() {
                Token::Mult => {
                    self.bump();
                    let right = self.parse_unary();
                    left = Expression::Multiplication(Box::new(left), Box::new(right));
                }
                Token::Div => {
                    self.bump();
                    let right = self.parse_unary();
                    left = Expression::Division(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Expression {
        match self.peek() {
            Token::Plus => {
                self.bump();
                self.parse_unary()
            }
            Token::Minus => {
                self.bump();
                let expr = self.parse_unary();
                Expression::Subtraction(
                    Box::new(Expression::Number(0.0)),
                    Box::new(expr),
                )
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Expression {
        let left = self.parse_primary();
        if matches!(self.peek(), Token::Pow) {
            self.bump();
            let right = self.parse_unary(); // right-associative
            return Expression::Exponentiation(Box::new(left), Box::new(right));
        }
        left
    }

    fn parse_primary(&mut self) -> Expression {
        match self.peek() {
            Token::Number(_) => self.parse_number(),
            Token::OpenParen => {
                self.bump();
                let inner = self.parse_expression();
                self.expect(Token::CloseParen);
                Expression::Parenthesis(Box::new(inner))
            }
            other => panic!("Expected primary expression, got {:?}", other),
        }
    }

    fn parse_number(&mut self) -> Expression {
        let Token::Number(n) = self.bump() else {
            panic!("Expected number");
        };

        if matches!(self.peek(), Token::DecimalPoint) {
            self.bump();
            match self.bump() {
                Token::Number(frac) => {
                    let digits = frac.abs().to_string().len() as i32;
                    let decimal_part = (frac as f64) / 10f64.powi(digits);
                    Expression::Number(n as f64 + decimal_part)
                }
                other => panic!("Expected fractional digits after '.', got {:?}", other),
            }
        } else {
            Expression::Number(n as f64)
        }
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Expression {
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };

    let expr = parser.parse_expression();
    match parser.peek() {
        Token::EOF => expr,
        other => panic!("Unexpected token after expression: {:?}", other),
    }
}

fn evaluate_expression(expr: &Expression) -> f64 {
    match expr {
        Expression::Number(n) => *n,

        Expression::Addition(left, right) => evaluate_expression(left) + evaluate_expression(right),

        Expression::Subtraction(left, right) => evaluate_expression(left) - evaluate_expression(right),

        Expression::Multiplication(left, right) => evaluate_expression(left) * evaluate_expression(right),

        Expression::Division(left, right) => evaluate_expression(left) / evaluate_expression(right),

        Expression::Exponentiation(left, right) => evaluate_expression(left).powf(evaluate_expression(right)),
        
        Expression::Parenthesis(inner) => evaluate_expression(inner),
    }
}

fn parse_expression(tokens: Vec<Token>) {
    let expr = parse_tokens(tokens);
    println!("Parsed Expression: {:?}", expr);
    let expr = evaluate_expression(&expr);
    println!("Evaluated Expression: {}", expr);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str) -> f64 {
        let tokens = parse_input(input);
        let expr = parse_tokens(tokens);
        evaluate_expression(&expr)
    }

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

    #[test]
    fn test_parse_tokens() {
        let tokens = vec![
            Token::Number(12),
            Token::Plus,
            Token::Number(34),
            Token::Minus,
            Token::Number(5),
            Token::EOF,
        ];
        let expected_expression = Expression::Subtraction(
            Box::new(Expression::Addition(
                Box::new(Expression::Number(12.0)),
                Box::new(Expression::Number(34.0)),
            )),
            Box::new(Expression::Number(5.0)),
        );
        assert_eq!(parse_tokens(tokens), expected_expression);
    }

    #[test]
    fn test_parse_tokens_parentheses_after_plus() {
        let tokens = vec![
            Token::Number(1),
            Token::Plus,
            Token::OpenParen,
            Token::Number(1),
            Token::CloseParen,
            Token::EOF,
        ];
        let expected_expression = Expression::Addition(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Parenthesis(Box::new(Expression::Number(1.0)))),
        );
        assert_eq!(parse_tokens(tokens), expected_expression);
    }

    #[test]
    fn test_parse_tokens_unary_minus() {
        let tokens = vec![Token::Minus, Token::Number(1), Token::EOF];
        let expected_expression = Expression::Subtraction(
            Box::new(Expression::Number(0.0)),
            Box::new(Expression::Number(1.0)),
        );
        assert_eq!(parse_tokens(tokens), expected_expression);
    }

    #[test]
    fn test_parse_tokens_plus_then_unary_minus() {
        let tokens = vec![
            Token::Number(1),
            Token::Plus,
            Token::Minus,
            Token::Number(1),
            Token::EOF,
        ];
        let expected_expression = Expression::Addition(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Subtraction(
                Box::new(Expression::Number(0.0)),
                Box::new(Expression::Number(1.0)),
            )),
        );
        assert_eq!(parse_tokens(tokens), expected_expression);
    }

    #[test]
    fn test_pemdas_mul_before_add() {
        assert_eq!(eval("1+2*3"), 7.0);
    }

    #[test]
    fn test_pemdas_parentheses_override() {
        assert_eq!(eval("(1+2)*3"), 9.0);
    }

    #[test]
    fn test_pemdas_power_right_associative() {
        assert_eq!(eval("2^3^2"), 512.0);
    }

    #[test]
    fn test_pemdas_unary_minus_with_power() {
        assert_eq!(eval("-2^2"), -4.0);
        assert_eq!(eval("(-2)^2"), 4.0);
    }
}
