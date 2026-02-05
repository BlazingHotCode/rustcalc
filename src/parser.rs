use crate::error::CalcError;
use crate::lexer::Token;
use crate::{builtins, builtins::Operator};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    UnaryOp {
        op: Operator,
        expr: Box<Expression>,
    },
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall { name: String, args: Vec<Expression> },
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
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expression, CalcError> {
        let mut left = self.parse_prefix()?;

        loop {
            let Token::Op(op) = self.peek().clone() else {
                break;
            };

            let Some((l_bp, r_bp)) = builtins::infix_binding_power(op) else {
                break;
            };
            if l_bp < min_bp {
                break;
            }

            self.bump(); // consume operator
            let right = self.parse_expr_bp(r_bp)?;
            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expression, CalcError> {
        match self.peek().clone() {
            Token::Op(op) => {
                let Some(r_bp) = builtins::prefix_binding_power(op) else {
                    return self.parse_primary();
                };
                self.bump();
                let rhs = self.parse_expr_bp(r_bp)?;
                Ok(Expression::UnaryOp {
                    op,
                    expr: Box::new(rhs),
                })
            }
            _ => self.parse_primary(),
        }
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
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::CloseParen) {
                        args.push(self.parse_expression()?);
                        while matches!(self.peek(), Token::Comma) {
                            self.bump();
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect(Token::CloseParen)?;
                    Ok(Expression::FunctionCall {
                        name,
                        args,
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
