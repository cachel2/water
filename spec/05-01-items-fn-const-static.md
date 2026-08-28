# 5.1–5.5 · Items: functions, `const`, `static`

## 5.1 The item set

```
pub? ( fn | struct | enum | impl | const | static | use | test | extern )
```

There is no `mod` item: the filesystem is the module tree (§13). **There are no forward declarations** — the whole module is read before it is checked, so declaration order never matters and nothing is written twice. `pub` marks what leaves the module; it is the only visibility level.

## 5.2 Functions

```water
fn name<generics>(params) -> Ret {
    body
}
```

`-> Ret` is optional; absent, the return type is `()`. The body is a block (§7.1); its **tail expression is the return value** (§5.3). Parameters are `name: Type`. Generic parameters are `<T, U>` — no bounds (§4.13).

## 5.3 Return, tail expressions, and `()`

A function returns in one of three ways, and only these:

1. **Tail expression.** A block whose final expression has no trailing `;` yields that expression as its value; a function body is a block, so its tail is the return value.

```water
fn max(a: i32, b: i32) -> i32 {
    if (a > b) { a } else { b }       // no `return`, no `;` on the tail
}
```

2. **`return expr;`** for early exit. `return;` returns `()`.

```water
fn first_even(xs: []i32) -> Option<i32> {
    for (x in xs) {
        if (x % 2 == 0) { return Some(x); }    // early
    }
    None                                        // tail
}
```

3. **Falling off a `()` function.** A body ending in a statement (a `;`-terminated line, or a `for`/`while`) has value `()`, valid only when the return type is `()`.

The rule is total: **a block's value is its tail expression if it has one, else `()`.** There are no other block values, no `break value`, no implicit returns from anywhere but the tail. `return` and the tail must agree in type with `-> Ret`; a mismatch names both.

## 5.4 `const` and constant expressions

```water
const LIMIT: usize = 4096;
const BUF: usize = LIMIT * 2;
```

`const` is evaluated at compile time by the interpreter (§16) — its third job. A **constant expression** is: a literal; a `const`; `size_of::<T>()`, `align_of::<T>()`, `offset_of::<T>(field)`; arithmetic, comparison, and bitwise operators on constant expressions; a struct/array/enum literal of constant expressions; an `if` or `match` on constant expressions; and a call to a `fn` whose body transitively contains only the above. It may not allocate, call `extern`, read a `static`, take an address, or do a volatile operation.

**Const evaluation is bounded at 1,000,000 interpreted MIR instructions per `const`.** Exceeding it is a compile error naming the `const`, the limit, and the last span executed. No termination analysis, no knob. A trap during const evaluation is a compile error showing the trap and the const-eval stack.

## 5.5 `static`

```water
static COUNTER: i32 = 0;
static mut TABLE: [256]u32 = build_table();   // init is a constant expression
```

A global. `static` is immutable, `static mut` is mutable; the initializer is a constant expression. Globals live in `.data`/`.bss`, have no thread story (§1.3), and `pub static` is exported. (This replaces t3's `var`-global; `let` is locals-only in water.)

## 5.7 The rest of the items

`use path;` and `use path::{a, b};` bring names in (§13). `test "name" { … }` is a test item (§14). `extern "c" fn …;` declares a C function (§12). Grammar in §18.
