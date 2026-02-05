use crate::error::CalcError;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
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

pub(crate) fn parse_tokens(tokens: &[Token]) -> Result<Expression, CalcError> {
    let mut parser = Parser { tokens, pos: 0 };
    let expr = parser.parse_expression()?;
    match parser.peek() {
        Token::EOF => Ok(expr),
        other => Err(CalcError::UnexpectedTokenAfterExpression(other.clone())),
    }
}

