# rustcalc

A tiny command-line calculator written in Rust.

Supports:

- PEMDAS / operator precedence: `()`, `^`, `*`/`/`, `+`/`-`
- Unary `+` / unary `-`: `-1`, `1+-1`, `-(1)`
- Constants: `pi`, `e`
- Functions (comma-separated args): `sqrt(x)`, `min(a,b,...)`, `max(a,b,...)`

## Requirements

- Rust (stable) + Cargo

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

Type expressions and press Enter. Type `exit` to quit.

Examples:

```text
1+2*3
(1+2)*3
2^3^2
-2^2
(-2)^2
sqrt(1+3)
max(1,2,3,2)
2*pi
```

## Adding builtins (one place)

All constants, functions, and operators are defined in `src/builtins.rs`.

- **Add a constant**: add an entry to `CONSTANTS`
- **Add a function**: add an entry to `FUNCTIONS`
- **Add an operator**: add an entry to `OPS` (symbol, prefix/infix precedence, associativity, eval)

More details: see [`BUILTINS.md`](BUILTINS.md).

## License

See [`LICENSE`](LICENSE).
