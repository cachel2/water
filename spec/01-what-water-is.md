# 1 · What water is

**A general-purpose systems language that reads like Rust and remembers like C.** You get Rust's surface — `struct`, `enum`, `match`, `impl`, method chaining, `let`, `?`, tail expressions, `()` — and C's model underneath: `malloc` and `free`, raw pointers, no runtime, no GC, no ownership checker, no borrow checker, no lifetimes, no hidden allocation, no hidden control flow, no destructors. Values are bitwise-copyable, as in C; you manage the heap yourself.

What Rust taught that costs nothing at runtime, water keeps. What Rust taught that costs a borrow checker — ownership, lifetimes, `Fn` traits, closures — water declines, on purpose and with its eyes open (§1.3). The result is a language you can hold in your head and build alone, that still feels like writing Rust.

## 1.1 The concession

C23 enumerates 221 kinds of undefined behavior (Annex J.2, WG14 N3155). water has **two**:

> **UB-1 — invalid dereference.** Dereferencing a `*T`, or accessing through a `[]T`, that does not point to a live, properly-aligned object of type `T`, is undefined.
>
> **UB-2 — invalid free.** Passing to `free`/`free_n` a pointer not returned by an `alloc*` call, or one already freed, is undefined.

**Scoped honestly:** the claim holds for code that does not use `mem::unconst`, `mem::ptr_cast`, `mem::ptr_from_addr`, or `extern`. Those four are assertions you make and the compiler accepts — a different thing from undefined behavior. They are functions rather than syntax (§4.3) precisely so that `grep` enumerates every one in a program.

Everything else in J.2 is defined here or made unrepresentable; the entry-by-entry audit was done for water's predecessor and its six conclusions are written into this document as rules, not errata (§1.4). UB-1 and UB-2 are the price of manual memory, and water does not pretend to have paid it — it hands you the interpreter instead (§16). *The compiler prevents what it can prove; the interpreter catches the rest while you test; nothing is prevented by making the language bigger.*

Two things can be broken but are **not** UB: `str`'s UTF-8 promise (§4.9), and reading memory that definite-initialization could not prove written (§6.4). The first yields wrong output; the second yields an unspecified value. Neither is memory unsafety.

## 1.2 The surface, at a glance

```water
use std::io;

struct Point { x: f64, y: f64 }

enum Shape {
    Circle(radius: f64),
    Rect { w: f64, h: f64 },
    Empty,
}

impl Shape {
    fn area(self) -> f64 {
        match *self {
            Circle(r)     => 3.14159 * r * r,
            Rect { w, h } => w * h,
            Empty         => 0.0,
        }
    }
}

fn main() {
    let shapes = [Shape::Circle(2.0), Shape::Rect { w: 3.0, h: 4.0 }, Shape::Empty];
    let mut total = 0.0;
    for (s in shapes) {
        total += s.area();
    }
    io::print("total area = {}\n", total);
}
```

`fn` with a tail expression instead of `return`. `impl` with a `self` method. `s.area()` method call. `match` without parentheses, `if` with them. `let` and `let mut`. `()` where C wrote `void`. That is the whole flavor.

## 1.3 Non-goals — frozen

No ownership or borrow checker. No lifetimes. No `Fn`/`FnMut`/`FnOnce`. **No closures** — a function value is a bare function pointer with no captured environment (§4.14). No traits, interfaces, or typeclasses — `impl` gives inherent methods only (§5.6). No operator overloading. No macros or preprocessor. No exceptions or unwinding. No async. No GC. No runtime reflection. No inheritance. No move semantics — every value is bitwise-copyable, as in C. No `unsafe` keyword — the whole language is what Rust calls unsafe, and says so. No build scripts. No package registry in v1.

- **No tuples.** Return a named struct or `()`.
- **No `void`.** The empty type is `()`, unit, a real value (§4.1). A function with no `->` returns `()`.
- **No threads, atomics, or memory model.** `std::thread` over pthreads may come later as a library with C's data-race story, which is to say none. This is the one place "Rust surface, C memory" has nothing better to offer, and pretending otherwise is how the first design died.
- **No bit fields.** Implementation-defined in C, unportable across ABIs, a documented source of miscompilation. Masks and shifts (§4.11).
- **No `const` except on pointers and slices** (§4.3). No const locals — `let` bindings are immutable by default already (§6.1). No const functions, no const generics.
- **No `==` on structs or enums** — primitives and pointers only (§8.5). Write an `eq` method.
- **No integer range coverage in `match`.** Only `_` makes a `match` on an integer exhaustive (§7.5).
- **No generic bounds** (§4.13). A type parameter can be copied, addressed, stored, and passed — nothing else.

## 1.4 What water inherited

water is the surface of Rust laid over the memory model, trap discipline, and tooling of its predecessor `t3`, which was frozen and audited before this document existed. Three inheritances are load-bearing and are stated once here so the rest of the document can rely on them:

- **The two-UB result** and the C23 J.2 audit that established it. The audit's six findings — UB-1's exact wording, the `freeze`-on-unprovable-load rule, the negative-shift trap, the pointer-difference trap, the empty-aggregate compile error, and the escape-hatch scoping — are written into this document as ordinary rules. They are not errata here; they were errata against t3 and are settled law in water.
- **The interpreter as sanitizer** (§16). The single most important design decision, and the reason manual memory is defensible: every `water test` run executes under an interpreter that models the heap and catches both UBs with full allocation history.
- **The debug allocator** (§10.2), graded honestly, closing the interpreter's coverage gap under fuzzing and load.
