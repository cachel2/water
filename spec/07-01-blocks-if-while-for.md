# 7.1–7.4 · Blocks, `if`, `while`, `for`, ranges

**§7 preamble.** Control leaves a function by `return`, by the tail expression of its body, or by a trap. No exceptions, no unwinding, no `longjmp`, no `goto`.

## 7.1 Blocks and the semicolon

A block is `{ stmt* expr? }`. **Its value is the trailing `expr` if present and not followed by `;`, otherwise `()`** (§5.3). The semicolon is meaningful:

- `expr` (no `;`) as the last element — the block's value.
- `expr;` — a statement; the value is discarded and the block continues (or ends with value `()`).

Discarding a non-`()` value with `;` is allowed — it is how you call for effect (`vec.push(3);`). There are no tail expressions anywhere except the last position of a block, and blocks nest, so `if`/`match` used as the tail of a block carry their value outward naturally.

## 7.2 `if` and `while`

```water
if (x > 0) { … } else if (x < 0) { … } else { … }
while (!done) { … }
```

Parentheses mandatory (kept from C, by choice). Braces mandatory. Condition must be `bool`.

**`if` is an expression.** `let m = if (a > b) { a } else { b };`. Both branches must have the same type; an `if` with no `else` has type `()` and its `then` block must be `()`.

There is no `do`/`while`.

## 7.3 `for`

```water
for (i = 0; i < n; i += 1) { … }        // C-style, `i` is a let mut introduced by the header
for (x in xs) { … }                      // slice, array, or str: by value
for (*x in xs) { … }                     // by pointer, for large T
for (i in 0..n) { … }                    // half-open range
for (i in 0..=n) { … }                   // inclusive range
```

`for (x in xs)` emits no bounds check — there is nothing to check. Iterating a `str` yields `u8`. The iterated expression is evaluated once. `break`/`continue` apply to the innermost loop; there are no labels and no `for`/`else`. A `for` is a statement, value `()`.

## 7.4 Ranges

`a..b` is half-open, `a..=b` inclusive. Ranges appear only inside `[ ]` (slicing), after `in` (loops), and in patterns (§7.5). A `..` elsewhere is a parse error.
