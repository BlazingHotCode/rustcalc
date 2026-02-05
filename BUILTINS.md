# Builtins Guide

This project is set up so you can add **constants**, **functions**, and **operators** by editing a single file:

- [`src/builtins.rs`](src/builtins.rs)

The parser and evaluator are generic:

- The lexer recognizes operator characters via `builtins::is_operator_char(...)`.
- The parser uses operator binding powers from `builtins::prefix_binding_power(...)` / `builtins::infix_binding_power(...)`.
- The evaluator calls `builtins::eval_constant(...)`, `builtins::eval_function(...)`, `builtins::eval_prefix(...)`, and `builtins::eval_infix(...)`.

## Add a constant

In [`src/builtins.rs`](src/builtins.rs), add a `BuiltinConst` entry in `CONSTANTS`:

- `name` is stored lowercase (lookup normalizes input name to lowercase).

Example:

```rust
BuiltinConst { name: "tau", value: 2.0 * std::f64::consts::PI }
```

Then you can use it as `tau` in expressions.

## Add a function

In [`src/builtins.rs`](src/builtins.rs), add a `BuiltinFunc` entry in `FUNCTIONS` and a small `*_impl` function.

Guidelines:

- Function names are stored lowercase; input is normalized to lowercase on lookup.
- Use `min_arity` / `max_arity` to enforce the number of arguments.
  - `max_arity: Some(1)` means exactly 1 argument.
  - `max_arity: None` means variable number of arguments (but still must be `>= min_arity`).

Example (`avg(a,b,...)`):

```rust
fn avg_impl(args: &[f64]) -> Result<f64, CalcError> {
    let sum: f64 = args.iter().sum();
    Ok(sum / (args.len() as f64))
}

BuiltinFunc {
    name: "avg",
    min_arity: 1,
    max_arity: None,
    eval: avg_impl,
}
```

Then you can call it like: `avg(1,2,3,4)`.

## Add an operator (symbol)

Operators live in the single `OPS` table in [`src/builtins.rs`](src/builtins.rs).

Each `BuiltinOp` entry controls:

- `symbol`: the character the lexer recognizes (e.g. `'%'`).
- `prefix_precedence`: if `Some(n)`, the operator is allowed as a prefix unary operator.
- `infix_precedence` + `infix_assoc`: if present, the operator is allowed as an infix operator.
- `eval_prefix` / `eval_infix`: the evaluation function(s) to apply.

Precedence rule of thumb (current defaults):

- `+`/`-` infix: 10
- `*`/`/` infix: 20
- `^` infix: 30 (right-associative)
- unary `+`/`-` prefix: 25

Important:

- If you want `-2^2` to keep meaning `-(2^2)`, make sure unary `-` prefix precedence stays **lower than** `^` infix precedence.

Example: add modulo `%` with the same precedence as `*`/`/`:

```rust
fn mod_impl(a: f64, b: f64) -> Result<f64, CalcError> {
    if b == 0.0 {
        return Err(CalcError::DivideByZero);
    }
    Ok(a % b)
}

BuiltinOp {
    symbol: '%',
    prefix_precedence: None,
    infix_precedence: Some(20),
    infix_assoc: Some(Assoc::Left),
    eval_prefix: None,
    eval_infix: Some(mod_impl),
}
```

After that, `%` works automatically in the lexer/parser/evaluator.

## Testing

After adding builtins, run:

```bash
cargo test
```
