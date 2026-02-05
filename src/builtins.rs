use crate::error::CalcError;

pub(crate) type Operator = char;

struct BuiltinConst {
    name: &'static str, // stored lowercase
    value: f64,
}

struct BuiltinFunc {
    name: &'static str, // stored lowercase
    min_arity: usize,
    max_arity: Option<usize>,
    eval: fn(&[f64]) -> Result<f64, CalcError>,
}

const CONSTANTS: &[BuiltinConst] = &[
    BuiltinConst {
        name: "pi",
        value: std::f64::consts::PI,
    },
    BuiltinConst {
        name: "e",
        value: std::f64::consts::E,
    },
];

fn sqrt_impl(args: &[f64]) -> Result<f64, CalcError> {
    Ok(args[0].sqrt())
}

fn min_impl(args: &[f64]) -> Result<f64, CalcError> {
    let mut best = args[0];
    for &value in &args[1..] {
        best = best.min(value);
    }
    Ok(best)
}

fn max_impl(args: &[f64]) -> Result<f64, CalcError> {
    let mut best = args[0];
    for &value in &args[1..] {
        best = best.max(value);
    }
    Ok(best)
}

const FUNCTIONS: &[BuiltinFunc] = &[
    BuiltinFunc {
        name: "sqrt",
        min_arity: 1,
        max_arity: Some(1),
        eval: sqrt_impl,
    },
    BuiltinFunc {
        name: "min",
        min_arity: 1,
        max_arity: None,
        eval: min_impl,
    },
    BuiltinFunc {
        name: "max",
        min_arity: 1,
        max_arity: None,
        eval: max_impl,
    },
];

fn normalize_name(name: &str) -> String {
    name.to_ascii_lowercase()
}

pub(crate) fn eval_constant(name: &str) -> Option<f64> {
    let name = normalize_name(name);
    CONSTANTS
        .iter()
        .find(|c| c.name == name)
        .map(|c| c.value)
}

pub(crate) fn eval_function(name: &str, args: &[f64]) -> Result<f64, CalcError> {
    let normalized = normalize_name(name);
    let Some(func) = FUNCTIONS.iter().find(|f| f.name == normalized) else {
        return Err(CalcError::UnknownFunction(name.to_string()));
    };

    if args.len() < func.min_arity {
        return Err(CalcError::WrongArity {
            name: name.to_string(),
            expected: func.min_arity,
            got: args.len(),
        });
    }
    if let Some(max) = func.max_arity {
        if args.len() > max {
            return Err(CalcError::WrongArity {
                name: name.to_string(),
                expected: max,
                got: args.len(),
            });
        }
    }

    (func.eval)(args)
}

#[derive(Clone, Copy)]
enum Assoc {
    Left,
    Right,
}

struct BuiltinOp {
    symbol: Operator,
    // Higher number = binds tighter. Prefix precedence must be < '^' to keep `-2^2` == `-(2^2)`.
    prefix_precedence: Option<u8>,
    infix_precedence: Option<u8>,
    infix_assoc: Option<Assoc>,
    eval_prefix: Option<fn(f64) -> Result<f64, CalcError>>,
    eval_infix: Option<fn(f64, f64) -> Result<f64, CalcError>>,
}

fn add_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    Ok(a + b)
}
fn sub_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    Ok(a - b)
}
fn mul_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    Ok(a * b)
}
fn div_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    if b == 0.0 {
        return Err(CalcError::DivideByZero);
    }
    Ok(a / b)
}
fn pow_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    Ok(a.powf(b))
}

fn unary_plus_impl(a: f64) -> Result<f64, CalcError> {
    Ok(a)
}
fn unary_minus_impl(a: f64) -> Result<f64, CalcError> {
    Ok(-a)
}

const OPS: &[BuiltinOp] = &[
    BuiltinOp {
        symbol: '+',
        prefix_precedence: Some(25),
        infix_precedence: Some(10),
        infix_assoc: Some(Assoc::Left),
        eval_prefix: Some(unary_plus_impl),
        eval_infix: Some(add_impl),
    },
    BuiltinOp {
        symbol: '-',
        prefix_precedence: Some(25),
        infix_precedence: Some(10),
        infix_assoc: Some(Assoc::Left),
        eval_prefix: Some(unary_minus_impl),
        eval_infix: Some(sub_impl),
    },
    BuiltinOp {
        symbol: '*',
        prefix_precedence: None,
        infix_precedence: Some(20),
        infix_assoc: Some(Assoc::Left),
        eval_prefix: None,
        eval_infix: Some(mul_impl),
    },
    BuiltinOp {
        symbol: '/',
        prefix_precedence: None,
        infix_precedence: Some(20),
        infix_assoc: Some(Assoc::Left),
        eval_prefix: None,
        eval_infix: Some(div_impl),
    },
    BuiltinOp {
        symbol: '^',
        prefix_precedence: None,
        infix_precedence: Some(30),
        infix_assoc: Some(Assoc::Right),
        eval_prefix: None,
        eval_infix: Some(pow_impl),
    },
];

fn find_op(op: Operator) -> Option<&'static BuiltinOp> {
    OPS.iter().find(|o| o.symbol == op)
}

pub(crate) fn is_operator_char(ch: char) -> bool {
    find_op(ch).is_some()
}

pub(crate) fn infix_binding_power(op: Operator) -> Option<(u8, u8)> {
    let info = find_op(op)?;
    let prec = info.infix_precedence?;
    let assoc = info.infix_assoc?;
    let l_bp = prec;
    let r_bp = match assoc {
        Assoc::Left => prec + 1,
        Assoc::Right => prec,
    };
    Some((l_bp, r_bp))
}

pub(crate) fn prefix_binding_power(op: Operator) -> Option<u8> {
    find_op(op)?.prefix_precedence
}

pub(crate) fn eval_infix(op: Operator, left: f64, right: f64) -> Result<f64, CalcError> {
    let info = find_op(op).ok_or_else(|| CalcError::UnknownFunction(op.to_string()))?;
    let eval = info
        .eval_infix
        .ok_or_else(|| CalcError::UnknownFunction(op.to_string()))?;
    eval(left, right)
}

pub(crate) fn eval_prefix(op: Operator, value: f64) -> Result<f64, CalcError> {
    let info = find_op(op).ok_or_else(|| CalcError::UnknownFunction(op.to_string()))?;
    let eval = info
        .eval_prefix
        .ok_or_else(|| CalcError::UnknownFunction(op.to_string()))?;
    eval(value)
}
