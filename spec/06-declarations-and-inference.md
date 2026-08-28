# 6 · Declarations, initialization, inference

## 6.1 `let` and `let mut`

```water
let x: i32 = 5;          // immutable
let mut count = 0;       // mutable, type inferred
let p = &node;
let buf: [4096]u8 = undefined;
```

Every local is initialized at its declaration; there is no `let x: i32;`. A `let` binding is **immutable** — reassigning it is a compile error naming `let mut`. `let mut` is mutable. This is the one place water is stricter than C and matches Rust: immutable by default. It costs the compiler nothing (a flag on the binding) and it is the cheapest bug-catch in the language.

## 6.2 `undefined`

`= undefined` opts out of initialization, for buffers you will fill. Legal **only** as the entire initializer of a `let`/`let mut`, never as a subexpression, argument, or field default.

Reading a location before it is written is caught by **definite-initialization analysis** over MIR; the read does not compile, whether or not `undefined` was written. Where the analysis cannot prove initialization, it is a compile error demanding you restructure — not a warning.

## 6.3 Inference

**Local inference only.** Signatures are always fully explicit — a function's type never depends on its callers. Inside a body, unification runs over the whole body, forward and backward: `let mut i = 0;` followed by `i < xs.len` infers `i: usize`. An unconstrained integer literal defaults to `i32`, a float literal to `f64`. Ambiguity a default cannot resolve is an error naming the candidates and the one-line fix. Inference never crosses a function boundary.

## 6.4 Unprovable loads

> **Editorial note, not part of the frozen text.** §1.1 and §17 (IR-2) both cite "§6.4", but the frozen document has no §6.4 body. The rule those two sections already state is reproduced here so the citation resolves. It adds nothing.
>
> Reading memory that definite-initialization could not prove written yields an **unspecified value, not undefined behavior** (§1.1). The implementation obligation behind that is IR-2 (§17): such a load must be `freeze`d.
