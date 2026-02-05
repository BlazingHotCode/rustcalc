use std::{
    fmt,
    io,
};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Ident(String),
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
    Identifier(String),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Exponentiation(Box<Expression>, Box<Expression>),
    FunctionCall { name: String, arg: Box<Expression> },
    Parenthesis(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
enum CalcError {
    UnexpectedChar(char),
    ExpectedToken { expected: Token, got: Token },
    ExpectedPrimary(Token),
    ExpectedNumber(Token),
    ExpectedFractionDigits(Token),
    UnexpectedTokenAfterExpression(Token),
    UnknownIdentifier(String),
    UnknownFunction(String),
    DivideByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::UnexpectedChar(ch) => write!(f, "unexpected character: {ch}"),
            CalcError::ExpectedToken { expected, got } => {
                write!(f, "expected token {expected:?}, got {got:?}")
            }
            CalcError::ExpectedPrimary(got) => write!(f, "expected expression, got {got:?}"),
            CalcError::ExpectedNumber(got) => write!(f, "expected number, got {got:?}"),
            CalcError::ExpectedFractionDigits(got) => {
                write!(f, "expected digits after '.', got {got:?}")
            }
            CalcError::UnexpectedTokenAfterExpression(got) => {
                write!(f, "unexpected token after expression: {got:?}")
            }
            CalcError::UnknownIdentifier(name) => write!(f, "unknown identifier: {name}"),
            CalcError::UnknownFunction(name) => write!(f, "unknown function: {name}"),
            CalcError::DivideByZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

fn main() {
    loop {
        let input = read_input();

        if input == "exit" {
            break;
        }

        match parse_input(&input) {
            Ok(tokens) => parse_expression(tokens),
            Err(err) => eprintln!("Error: {err}"),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_input(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while i < chars.len()
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_')
                {
                    ident.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Ident(ident));
                continue;
            }
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
            other => return Err(CalcError::UnexpectedChar(other)),
        }
        i += 1;
    }

    tokens.push(Token::EOF);
    Ok(tokens)
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
        if self.pos >= self.tokens.len() {
            return Token::EOF;
        }
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), CalcError> {
        let got = self.bump();
        if got != expected {
            return Err(CalcError::ExpectedToken { expected, got });
        }
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<Expression, CalcError> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expression, CalcError> {
        let mut left = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.bump();
                    let right = self.parse_mul_div()?;
                    left = Expression::Addition(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.bump();
                    let right = self.parse_mul_div()?;
                    left = Expression::Subtraction(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expression, CalcError> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Mult => {
                    self.bump();
                    let right = self.parse_unary()?;
                    left = Expression::Multiplication(Box::new(left), Box::new(right));
                }
                Token::Div => {
                    self.bump();
                    let right = self.parse_unary()?;
                    left = Expression::Division(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, CalcError> {
        match self.peek() {
            Token::Plus => {
                self.bump();
                self.parse_unary()
            }
            Token::Minus => {
                self.bump();
                let expr = self.parse_unary()?;
                Ok(Expression::Subtraction(
                    Box::new(Expression::Number(0.0)),
                    Box::new(expr),
                ))
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Result<Expression, CalcError> {
        let left = self.parse_primary()?;
        if matches!(self.peek(), Token::Pow) {
            self.bump();
            let right = self.parse_unary()?; // right-associative
            return Ok(Expression::Exponentiation(Box::new(left), Box::new(right)));
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, CalcError> {
        match self.peek() {
            Token::Number(_) => self.parse_number(),
            Token::Ident(_) => {
                let token = self.bump();
                let Token::Ident(name) = token else {
                    return Err(CalcError::ExpectedPrimary(token));
                };

                if matches!(self.peek(), Token::OpenParen) {
                    self.bump();
                    let arg = self.parse_expression()?;
                    self.expect(Token::CloseParen)?;
                    Ok(Expression::FunctionCall {
                        name,
                        arg: Box::new(arg),
                    })
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            Token::OpenParen => {
                self.bump();
                let inner = self.parse_expression()?;
                self.expect(Token::CloseParen)?;
                Ok(Expression::Parenthesis(Box::new(inner)))
            }
            other => Err(CalcError::ExpectedPrimary(other.clone())),
        }
    }

    fn parse_number(&mut self) -> Result<Expression, CalcError> {
        let token = self.bump();
        let Token::Number(n) = token else {
            return Err(CalcError::ExpectedNumber(token));
        };

        if matches!(self.peek(), Token::DecimalPoint) {
            self.bump();
            match self.bump() {
                Token::Number(frac) => {
                    let digits = frac.abs().to_string().len() as i32;
                    let decimal_part = (frac as f64) / 10f64.powi(digits);
                    Ok(Expression::Number(n as f64 + decimal_part))
                }
                other => Err(CalcError::ExpectedFractionDigits(other)),
            }
        } else {
            Ok(Expression::Number(n as f64))
        }
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Expression, CalcError> {
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };

    let expr = parser.parse_expression()?;
    match parser.peek() {
        Token::EOF => Ok(expr),
        other => Err(CalcError::UnexpectedTokenAfterExpression(other.clone())),
    }
}

fn evaluate_expression(expr: &Expression) -> Result<f64, CalcError> {
    match expr {
        Expression::Number(n) => Ok(*n),
        Expression::Identifier(name) => match name.as_str() {
            "pi" | "PI" | "Pi" => Ok(std::f64::consts::PI),
            "e" | "E" => Ok(std::f64::consts::E),
            _ => Err(CalcError::UnknownIdentifier(name.clone())),
        },

        Expression::Addition(left, right) => Ok(evaluate_expression(left)? + evaluate_expression(right)?),

        Expression::Subtraction(left, right) => Ok(evaluate_expression(left)? - evaluate_expression(right)?),

        Expression::Multiplication(left, right) => Ok(evaluate_expression(left)? * evaluate_expression(right)?),

        Expression::Division(left, right) => {
            let denom = evaluate_expression(right)?;
            if denom == 0.0 {
                return Err(CalcError::DivideByZero);
            }
            Ok(evaluate_expression(left)? / denom)
        }

        Expression::Exponentiation(left, right) => Ok(evaluate_expression(left)?.powf(evaluate_expression(right)?)),

        Expression::FunctionCall { name, arg } => match name.as_str() {
            "sqrt" => Ok(evaluate_expression(arg)?.sqrt()),
            _ => Err(CalcError::UnknownFunction(name.clone())),
        },
        
        Expression::Parenthesis(inner) => evaluate_expression(inner),
    }
}

fn parse_expression(tokens: Vec<Token>) {
    let expr = match parse_tokens(tokens) {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    println!("Parsed Expression: {:?}", expr);
    match evaluate_expression(&expr) {
        Ok(value) => println!("Evaluated Expression: {}", value),
        Err(err) => eprintln!("Error: {err}"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str) -> Result<f64, CalcError> {
        let tokens = parse_input(input)?;
        let expr = parse_tokens(tokens)?;
        evaluate_expression(&expr)
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-10,
            "expected {expected}, got {actual}"
        );
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
        assert_eq!(parse_input(input).unwrap(), expected_tokens);
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
        assert_eq!(parse_tokens(tokens).unwrap(), expected_expression);
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
        assert_eq!(parse_tokens(tokens).unwrap(), expected_expression);
    }

    #[test]
    fn test_parse_tokens_unary_minus() {
        let tokens = vec![Token::Minus, Token::Number(1), Token::EOF];
        let expected_expression = Expression::Subtraction(
            Box::new(Expression::Number(0.0)),
            Box::new(Expression::Number(1.0)),
        );
        assert_eq!(parse_tokens(tokens).unwrap(), expected_expression);
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
        assert_eq!(parse_tokens(tokens).unwrap(), expected_expression);
    }

    #[test]
    fn test_pemdas_mul_before_add() {
        assert_eq!(eval("1+2*3").unwrap(), 7.0);
    }

    #[test]
    fn test_pemdas_parentheses_override() {
        assert_eq!(eval("(1+2)*3").unwrap(), 9.0);
    }

    #[test]
    fn test_pemdas_power_right_associative() {
        assert_eq!(eval("2^3^2").unwrap(), 512.0);
    }

    #[test]
    fn test_pemdas_unary_minus_with_power() {
        assert_eq!(eval("-2^2").unwrap(), -4.0);
        assert_eq!(eval("(-2)^2").unwrap(), 4.0);
    }

    #[test]
    fn test_error_unexpected_char() {
        assert!(parse_input("1@").is_err());
    }

    #[test]
    fn test_error_divide_by_zero() {
        assert_eq!(eval("1/0").unwrap_err(), CalcError::DivideByZero);
    }

    #[test]
    fn test_error_unknown_identifier() {
        assert_eq!(eval("a").unwrap_err(), CalcError::UnknownIdentifier("a".to_string()));
    }

    #[test]
    fn test_eval_sqrt() {
        assert_close(eval("sqrt(9)").unwrap(), 3.0);
        assert_close(eval("sqrt(1+3)").unwrap(), 2.0);
    }

    #[test]
    fn test_eval_constants() {
        assert_close(eval("pi").unwrap(), std::f64::consts::PI);
        assert_close(eval("e").unwrap(), std::f64::consts::E);
        assert_close(eval("2*pi").unwrap(), 2.0 * std::f64::consts::PI);
    }
}
