# 7.5 · `match`

No parentheses around the subject (by choice):

```water
match shape {
    Circle(r)     => 3.14159 * r * r,
    Rect { w, h } => w * h,
    Empty         => 0.0,
}
```

**`match` is always an expression** (grammar: `match_expr`, §18). It reaches statement position the same way any expression does — through `expr ";"` — so a `match` used for effect carries a trailing `;` and its value, which must be `()`, is discarded. There is no separate statement form and no `match_stmt` production: **position decides whether the value is used, exactly as §7.1 already governs every expression**, and the shape of the arms decides nothing. `let n = match x { A => { 1 }, B => { 2 } };` has block arms and is an expression whose value is used; the same `match` written `match x { A => { f() }, B => { g() } };` at statement level has its `()` value discarded. Whether an arm is `=> expr` or `=> { … }` only determines where that arm's value comes from (§7.1), never whether the `match` is a statement.

Every arm's body is an expression or a block; a block arm's value is its tail (§7.1). All arms unify to one type. The complete rules:

1. **Exhaustive.** A missing variant is a compile error naming the missing constructors and their declaration sites.
2. **On integers, only `_` is exhaustive** — no range-coverage analysis. On `bool`, `true` and `false` are exhaustive.
3. **No fallthrough, no `break` to leave a match.** `break` inside an arm belongs to the enclosing loop.
4. **All arms unify**, with no coercion beyond `*T`→`*const T` and `[]T`→`[]const T`. A mismatch names both arms and both types.
5. **Divergent arms contribute no type — but only when the arm is a bare `return`/`break`/`continue`, not a block ending in one.** `return …`, `break`, `continue` may be arm bodies:

```water
let n = match parse_i32(arg) {
    Ok(v)  => v,
    Err(e) => return Err(e),      // no type; the match is still i32
};
```

A **block** arm that ends in `return`, by contrast, has value `()` (§7.1: a block's value is its tail, and `return` is a statement, so the block ends in a statement and yields `()`). It therefore does **not** count as divergent, and `()` does not unify with the other arms:

```water
// Does NOT compile: the Err block yields (), which does not unify with i32.
let n = match x {
    Ok(v)  => v,
    Err(e) => { cleanup(); return Err(e); },
};
```

This is a deliberate limitation, not a hole: §7.1 gives a clear answer (`()`), and an inconvenient answer is not silence (§20.2). The workaround is to bind the result in a `let mut` above a statement `match`, as §19.4 does. Making a block-tailed-in-`return` divergent would be a *change* — a new typing rule — and is parked in `ideas/` as "divergence analysis," unopened.

6. **An unreachable arm is a compile error** — an arm wholly covered by earlier arms. This catches the hazard in rule 8.
7. **Guards** (`Circle(r) if r > 0.0 =>`) never count toward exhaustiveness.
8. **Patterns:** `_`; a literal; a literal range (`1..=9`, `'a'..='z'`); a variant with binding (`Circle(r)`, `Rect { w, h }`, `Empty`); nested variant patterns (`Ok(Some(v))`); and a **bare identifier**, which is a *binding* unless the subject's type has a variant of that name, in which case it is that variant. A misspelled variant becomes a catch-all binding — and rule 6 turns that into a compile error at the next arm.
9. **Struct-variant patterns bind by field name only** (`Rect { w, h }`); no renaming, no `..`.
10. **The subject is evaluated once**; arms are tested in source order; guards run in that order.

Matching a value copies it (there is no move semantics); to match a large enum without copying, match on a pointer and bind pointers. `match *self` where `self: *const Shape` copies the `Shape` and matches it; this is the common form and is exactly what §5.6's methods use.

Integer and plain-enum matches lower to the same jump table C's `switch` does — minus the ability to forget a case.
