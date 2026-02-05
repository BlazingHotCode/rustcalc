use crate::lexer::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum CalcError {
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
            CalcError::ExpectedFractionDigits(got) => write!(f, "expected digits after '.', got {got:?}"),
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

