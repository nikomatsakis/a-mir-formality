# Linting judgment functions

The `formality-lint` library identifies
`judgment_fn!` invocations, balances their delimiters, and applies regular
expressions within their bodies. Comments and string/character literals are
masked while preserving source locations. Ordinary Rust helpers outside the
macro bodies are not linted.

Run the tests from the repository root:

```sh
# Test the scanner and lint rules.
cargo test -p formality-lint

# Lint this repository through the library.
cargo test --test judgment_lint
```

The integration test calls `formality_lint::lint()` directly and reports
findings with `file:line:column` locations. Any lint finding or scan error
fails the test. No command-line wrapper is required.

## Explicit clones

The initial `explicit-clone` rule reports `.clone(...)` and qualified
`Clone::clone(...)` calls inside judgments. Prefer the generated constructors
and implicit borrowing/casting described in
[Idiomatic judgment functions](formality_core/idiomatic_judgment_fn.md).
For example:

```rust,ignore
// Avoid explicit ownership bookkeeping in the rule.
(let expr = Expr::BinOp { lhs: lhs.clone(), op: BinOpKind::BitXor, rhs: rhs.clone() })

// The generated constructor accepts borrowed, upcastable operands.
(let expr = Expr::bin_op(lhs, BinOpKind::BitXor, rhs))
```

This is a style rule, not a claim that the constructor avoids cloning
internally. A helper that takes owned values may need an API change before a
clone can be removed. Do not move proof logic into a helper just to hide a lint.

The root integration test in `tests/judgment_lint.rs` uses `formality-lint` as
a dev-dependency and calls `formality_lint::lint()` on the Rust sources under
`crates/`. It runs with the normal workspace tests and fails on any diagnostic.
There is no automatic rewrite or suppression mechanism; review each finding
before changing code.

## Scope and limitations

This is not a Rust parser or a judgment DSL parser. It does not expand macros,
resolve aliases of `judgment_fn`, evaluate `cfg`, or decide whether a Rust
conditional is really a proof obligation. It checks literal macro invocations,
including those in test code. Malformed comments, strings, or invocation
delimiters produce a scan error instead of a silently incomplete audit.

Additional regex checks can be added in `formality_lint`, with scanner
and diagnostic tests. Patterns such as `.is_proven()` or `.upcast()` need contextual review
before being made unconditional errors; this first version only enforces the
explicit-clone rule.
