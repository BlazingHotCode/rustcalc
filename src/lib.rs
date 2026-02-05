mod error;
mod eval;
mod lexer;
mod parser;

pub use error::CalcError;
pub use parser::Expression;

pub fn parse(input: &str) -> Result<Expression, CalcError> {
    let tokens = lexer::tokenize(input)?;
    parser::parse_tokens(&tokens)
}

pub fn eval(input: &str) -> Result<f64, CalcError> {
    let expr = parse(input)?;
    eval::evaluate_expression(&expr)
}

pub fn eval_expression(expr: &Expression) -> Result<f64, CalcError> {
    eval::evaluate_expression(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    fn eval_input(input: &str) -> Result<f64, CalcError> {
        eval(input)
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-10,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn test_parse_input_tokens() {
        let input = "12 + 34 - 5";
        let expected_tokens = vec![
            Token::Number(12),
            Token::Plus,
            Token::Number(34),
            Token::Minus,
            Token::Number(5),
            Token::EOF,
        ];
        assert_eq!(crate::lexer::tokenize(input).unwrap(), expected_tokens);
    }

    #[test]
    fn test_parse_tokens_structure() {
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
        assert_eq!(crate::parser::parse_tokens(&tokens).unwrap(), expected_expression);
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
        assert_eq!(crate::parser::parse_tokens(&tokens).unwrap(), expected_expression);
    }

    #[test]
    fn test_parse_tokens_unary_minus() {
        let tokens = vec![Token::Minus, Token::Number(1), Token::EOF];
        let expected_expression = Expression::Subtraction(
            Box::new(Expression::Number(0.0)),
            Box::new(Expression::Number(1.0)),
        );
        assert_eq!(crate::parser::parse_tokens(&tokens).unwrap(), expected_expression);
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
        assert_eq!(crate::parser::parse_tokens(&tokens).unwrap(), expected_expression);
    }

    #[test]
    fn test_pemdas_mul_before_add() {
        assert_eq!(eval_input("1+2*3").unwrap(), 7.0);
    }

    #[test]
    fn test_pemdas_parentheses_override() {
        assert_eq!(eval_input("(1+2)*3").unwrap(), 9.0);
    }

    #[test]
    fn test_pemdas_power_right_associative() {
        assert_eq!(eval_input("2^3^2").unwrap(), 512.0);
    }

    #[test]
    fn test_pemdas_unary_minus_with_power() {
        assert_eq!(eval_input("-2^2").unwrap(), -4.0);
        assert_eq!(eval_input("(-2)^2").unwrap(), 4.0);
    }

    #[test]
    fn test_error_unexpected_char() {
        assert!(crate::lexer::tokenize("1@").is_err());
    }

    #[test]
    fn test_error_divide_by_zero() {
        assert_eq!(eval_input("1/0").unwrap_err(), CalcError::DivideByZero);
    }

    #[test]
    fn test_eval_sqrt() {
        assert_close(eval_input("sqrt(9)").unwrap(), 3.0);
        assert_close(eval_input("sqrt(1+3)").unwrap(), 2.0);
    }

    #[test]
    fn test_eval_constants() {
        assert_close(eval_input("pi").unwrap(), std::f64::consts::PI);
        assert_close(eval_input("e").unwrap(), std::f64::consts::E);
        assert_close(eval_input("2*pi").unwrap(), 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_error_unknown_identifier() {
        assert_eq!(
            eval_input("a").unwrap_err(),
            CalcError::UnknownIdentifier("a".to_string())
        );
    }

    #[test]
    fn test_eval_multi_arg_functions() {
        assert_close(eval_input("max(1,2,3,2)").unwrap(), 3.0);
        assert_close(eval_input("min(1,2,3,2)").unwrap(), 1.0);
        assert_close(eval_input("max(1+2, 2*3, 4^2)").unwrap(), 16.0);
    }

    #[test]
    fn test_error_wrong_arity() {
        assert_eq!(
            eval_input("sqrt(1,2)").unwrap_err(),
            CalcError::WrongArity {
                name: "sqrt".to_string(),
                expected: 1,
                got: 2
            }
        );
        assert_eq!(
            eval_input("max()").unwrap_err(),
            CalcError::WrongArity {
                name: "max".to_string(),
                expected: 1,
                got: 0
            }
        );
    }
}
