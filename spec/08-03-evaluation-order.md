# 8.3–8.5 · Method calls, evaluation order, equality

## 8.3 Method calls

`r.m(args)` resolves by §5.6. `r.f` is field access (auto-deref one level). `r.m(args).n()` chains: the value or pointer produced by `.m(...)` is the receiver of `.n`. There is no UFCS ambiguity because there are no traits — a method name resolves against the receiver type's own `impl` blocks and nowhere else.

## 8.4 Evaluation order — complete and normative

**Everything is left to right, in source order, always.** No unspecified order exists anywhere.

| Construct | Order |
|---|---|
| `f(a, b, c)` | `f` (if an expression), then `a`, `b`, `c` |
| `r.m(a, b)` | `r`, then `a`, `b`, then the call |
| `a op b` | `a`, then `b` |
| `a && b`, `a \|\| b` | `a`; `b` only if needed |
| `a[i]`, `a[i..j]` | `a`, then `i`, then `j`, then the check, then the access |
| `p.f` | `p`, then auto-deref, then the field |
| `place = expr` | **the place first, then `expr`, then the store** |
| `place op= expr` | the place once, then `expr`, then read-modify-write |
| `T { a: x, b: y }` | `x`, then `y` — **source order** — then omitted defaults, in declaration order |
| `[a, b, c]` | `a`, `b`, `c` |
| `if (c) { a } else { b }` | `c`, then the taken branch |
| `match e { … }` | `e` once; arms in source order; a guard runs only when its pattern matches |
| block with tail | statements in order, then the tail, then this block's defers (LIFO), then yield |
| `x?` | `x`, then the test |
| `x as U` | `x`, then the conversion |
| `defer e;` | `e`'s operands **now** (§7.6 rule 4); the call at block exit |

`place = expr` evaluating the place first is a deliberate departure from Rust (right-first) and from C (unspecified). One rule — left to right, everywhere — is worth more than matching either. Traps happen exactly where the order says.

## 8.5 Equality

`==` and `!=` are defined on integers (same type), floats (IEEE; `NaN != NaN`), `bool`, pointers (address comparison), and function pointers. **Not on structs, enums, arrays, slices, or `str`** — comparing by bytes is wrong (padding), field-wise is a derive, and a derive is a trait. Write an `eq` method: `a.eq(&b)`. For `str`, `str::eq(a, b)` is in `core`. `assert_eq` (§8.6) is a compiler builtin and exempt.
