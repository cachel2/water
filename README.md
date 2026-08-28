# water

A small systems language — the writing of Rust, the memory of C.
`struct`, `enum`, `match`, `impl`, method chaining, `?`, tail expressions,
over `malloc`/`free`. No borrow checker, no GC, no closures, no macros.

The compiler is `waterc`; files end in `.wtr`.

## Why

To learn LLVM and how a compiler actually fits together, by building one
alone from zero. The language is one I'd want to use, not a research vehicle —
Rust's surface is most of what I like, so water keeps it and drops the parts
that need a borrow checker.

## Status

Pre-compiler. The spec is frozen (`spec/water-spec-v1.0.md`, v1.0); `waterc`
does not exist yet. Nothing compiles. This is the honest state.

The build order is fixed: lexer → parser → HIR → typechecker → MIR →
interpreter → LLVM. The interpreter comes before codegen, because it doubles
as the sanitizer and the constant evaluator.

## The pitch, in one number

C23 has ~221 kinds of undefined behavior. water has **two**: use-after-free
and double-free. Both are caught by the interpreter, with full allocation
history, on every `water test`. Everything else is defined or doesn't parse.

That number isn't cleverness — two thirds of C's UB is the preprocessor,
varargs, locale, signals, setjmp, and stdio. water didn't defeat them.
It declined to have them.

## What it looks like

​```water
fn max(a: i32, b: i32) -> i32 {
    if (a > b) { a } else { b }
}

fn main() {
    let biggest = max(10, 42);
    io::print("{}", biggest);
}
​```

## Not implemented (on purpose, forever)

Ownership, lifetimes, traits, closures, generics with bounds, operator
overloading, macros, async, exceptions, GC, threads. Each one is the
borrow-checker's gravity well; water exists on the near side of it.

## Not implemented (yet)

All of it. See `spec/` for the frozen design and `corpus/` for the
conformance tests, which are being written before the code that passes them.
