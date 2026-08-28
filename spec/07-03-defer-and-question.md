# 7.6–7.7 · `defer` and `?`

## 7.6 `defer`

```water
fn load(path: str) -> Result<(), IoError> {
    let f = fs::open(path)?;
    defer fs::close(f);

    let buf = mem::alloc_n::<u8>(4096);
    defer mem::free_n(buf);

    …
    Ok(())
}
```

Complete rules:

1. **`defer` takes a call expression only** — `defer f(args);` or `defer x.m(args);`. Not an arbitrary expression: because `if` and `match` are expressions, `defer if (c) { f() } else { g() };` would otherwise parse, and rule 4 leaves it unclear whether `c` and the branch selection happen at the `defer` or at block exit. The most restrictive answer, and the one every example already obeys, is that a deferred expression must be a function or method call; its argument operands are captured per rule 4, and there is no internal control flow to schedule. A `defer` of anything but a call is a compile error naming this rule.
2. **The call runs when the enclosing block exits**, by any path: tail, `return`, `break`, `continue`, `?`.
3. **LIFO**, within the registering block; inner blocks run their defers before the outer block runs its.
4. **A `defer` in a loop body runs at the end of every iteration.**
5. **Operands are captured at `defer` time**, not at exit — `defer mem::free_n(buf);` frees the value `buf` had at the `defer`, not whatever it is reassigned to later. This is the opposite of Go and the reason `defer` is readable.
6. **A `defer` may not transfer control** — `return`, `break`, `continue`, `?` inside the call's arguments are compile errors. (`?` is a `return`, so `defer f(x?);` does not compile.)
7. **A `defer` on a path not taken never runs** — it is an instruction, not a declaration.
8. **A trap does not run defers**, in this frame or any frame (§11). Unwinding is not a feature water has.
9. **A trap inside a deferred call aborts immediately.**

**Interaction with tail expressions (normative):** a block's tail expression is evaluated *first*, its value saved, *then* the block's defers run (LIFO), *then* the value is yielded. So `defer` cannot see or alter the value being returned, and a value returned through `?` still runs the defers registered before the `?`.

`defer` is the entire resource story: no destructors, no RAII, no drop glue, no ordering subtleties. It is an HIR transformation, visible in the MIR dump, and it is why `free` on every path is tractable without ownership.

## 7.7 `?`

```water
let n = parse_i32(arg)?;
```

Desugars exactly to:

```water
let n = match parse_i32(arg) {
    Ok(v)  => v,
    Err(e) => return Err(e),
};
```

- On `Result<T, E>` in a function returning `Result<_, E>` — **the same `E`.** No conversion, because there is no `From` (no traits). Converting an error is a `match`, and it is visible.
- On `Option<T>` in a function returning `Option<_>`.
- Nowhere else; `?` in a function returning `i32` is a compile error naming the return type.
- `?` inside a `defer` is a compile error (§7.6 rule 5).

That is the whole error-handling story: no error-union type, no `orelse`, no panics-as-errors, no unwinding, no `errno`. An error is a value of a type you declared, returned from a function, propagated by one operator, and — with `Result<(), E>` — expressible even when the success case carries nothing (§4.1).
