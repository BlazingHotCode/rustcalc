use crate::error::CalcError;
use crate::parser::Expression;
use crate::builtins;

pub(crate) fn evaluate_expression(expr: &Expression) -> Result<f64, CalcError> {
    match expr {
        Expression::Number(n) => Ok(*n),
        Expression::Identifier(name) => builtins::eval_constant(name)
            .ok_or_else(|| CalcError::UnknownIdentifier(name.clone())),
        Expression::UnaryOp { op, expr } => {
            let value = evaluate_expression(expr)?;
            builtins::eval_prefix(*op, value)
        }
        Expression::BinaryOp { op, left, right } => {
            let a = evaluate_expression(left)?;
            let b = evaluate_expression(right)?;
            builtins::eval_infix(*op, a, b)
        }
        Expression::FunctionCall { name, args } => {
            let mut values = Vec::with_capacity(args.len());
            for arg in args {
                values.push(evaluate_expression(arg)?);
            }
            builtins::eval_function(name, &values)
        }
        Expression::Parenthesis(inner) => evaluate_expression(inner),
    }
}
