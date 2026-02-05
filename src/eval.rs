use crate::error::CalcError;
use crate::parser::Expression;

pub(crate) fn evaluate_expression(expr: &Expression) -> Result<f64, CalcError> {
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
        Expression::FunctionCall { name, args } => match name.as_str() {
            "sqrt" => {
                if args.len() != 1 {
                    return Err(CalcError::WrongArity {
                        name: name.clone(),
                        expected: 1,
                        got: args.len(),
                    });
                }
                Ok(evaluate_expression(&args[0])?.sqrt())
            }
            "min" => {
                if args.is_empty() {
                    return Err(CalcError::WrongArity {
                        name: name.clone(),
                        expected: 1,
                        got: 0,
                    });
                }
                let mut best = evaluate_expression(&args[0])?;
                for arg in &args[1..] {
                    best = best.min(evaluate_expression(arg)?);
                }
                Ok(best)
            }
            "max" => {
                if args.is_empty() {
                    return Err(CalcError::WrongArity {
                        name: name.clone(),
                        expected: 1,
                        got: 0,
                    });
                }
                let mut best = evaluate_expression(&args[0])?;
                for arg in &args[1..] {
                    best = best.max(evaluate_expression(arg)?);
                }
                Ok(best)
            }
            _ => Err(CalcError::UnknownFunction(name.clone())),
        },
        Expression::Parenthesis(inner) => evaluate_expression(inner),
    }
}
