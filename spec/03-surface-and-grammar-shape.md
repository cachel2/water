# 3 · Surface and grammar shape

## 3.1 Why the parser stays trivial

`waterc`'s parser is **recursive descent plus a Pratt expression parser, one token of lookahead, no backtracking, no symbol-table feedback, no lexer hack.** This is a normative property of the grammar. Every rule in §18 preserves it. Three decisions pay for it, and two of them are why water can look like Rust without inheriting Rust's parser:

- **`let` and `fn` lead their forms.** The single hardest thing in a C-family grammar is telling a declaration from an expression at statement position (`Vec v = …;` vs `a * b;`). A leading keyword kills it. Because `let` and `fn` announce themselves, water can then put types *after* names, Rust-style (§3.2), with no ambiguity — the thing that forces C into its declarator mess never arises.
- **Generic arguments in expression position use turbofish.** `f::<T>(x)`. In expressions, `f<T>(x)` and `(f < T) > (x)` are indistinguishable with one token of lookahead, and this rule is not negotiable. In *type* position there is no ambiguity — comparison does not occur there — so `Vec<i32>`, `Option<i32>`, and `struct Vec<T>` are written plainly. Turbofish appears only where a generic call cannot be inferred, which is rare (§4.13). This is the one Rust wart water keeps, for the same reason Rust has it.
- **`if`/`while` keep C's parentheses; `match` does not.** `if (c) { … }` and `while (c) { … }` take parentheses (§7.2). `match x { … }` does not (§7.5). This is a deliberate, user-chosen asymmetry; it costs the grammar nothing because both forms are unambiguous either way, and it happens to make a bare struct literal in an `if` subject impossible to confuse with the block.

## 3.2 Types come after names

```water
let x: i32 = 5;
let mut count: u32 = 0;
fn max(a: i32, b: i32) -> i32 { … }
struct Point { x: f64, y: f64 }
```

Bindings, parameters, and fields are `name: Type`, exactly as in Rust, enabled by the leading `let`/`fn`/field context. Type *constructors* are prefix and read left to right:

```water
*u8            pointer to u8
*const u8      pointer to const u8
[]u8           slice of u8
[]const u8     slice of const u8
[N]u8          array of N u8               (N a constant expression)
fn(i32, i32) -> i32                        function pointer
Option<*u8>    nullable pointer
Vec<u8>        a generic type
Self           the receiver type, inside an impl
```

There is no spiral rule, no `int *a, b` footgun, and no ambiguity between a type and an expression. `*const`/`[]const` are the only qualifiers (§4.3).

## 3.3 The anti-footgun rules — normative

1. **Assignment is a statement, never an expression.** `if (x = y)` is a parse error naming `==`.
2. **Comparison operators are non-associative.** `a < b < c` is a parse error naming the fix.
3. **Braces are mandatory** on every `if`/`else`/`while`/`for` body and every `match` arm that is a block. No dangling-else class exists.
4. **Bitwise mixed with comparison requires parentheses.** `a & b == c` is a compile error, not a lint — even though §8.1 gives `&` the tighter precedence, because the reader is the one being protected.
5. **Shift mixed with arithmetic requires parentheses.** `a + b << c` is a compile error.
6. **`&&` mixed with `||` requires parentheses.** `a && b || c` is a compile error.
7. **No fallthrough.** `match` has no `break`.
8. **No implicit conversions of any kind**, including integer promotion.
9. **`if`/`while` conditions must be `bool`.** No integer truthiness, no pointer truthiness.
