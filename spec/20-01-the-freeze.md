# 20 · The freeze

**Ratified 28 August 2026. This document does not change.**

Every operator has a precedence; every expression has an evaluation order and a type; every block has a value; every name has a resolution rule; every trap has a defined mechanism, hosted and bare-metal; every type either has a C ABI or is refused at the boundary; the grammar is complete and parses with one token of lookahead and no backtracking.

## 20.1 What the freeze covers

**The language:** §2–§9, §11, §12, §18. Those do not move. **Not `std` or tooling:** §10's APIs, §16.1's ladder, the CLI, the diagnostics catalogue, the build system. The test is mechanical: **if a change would alter a program's meaning, it is the language and it is frozen; if it only alters what `std` offers or what the tools say, it is not.**

Forbidden: adding or removing a keyword, operator, type constructor, production, or semantic rule; reinterpreting one because it is inconvenient to implement. If `waterc` is hard to write, `waterc` is wrong. If a program is awkward, the program is awkward. Ideas that arrive during implementation go to `ideas/`, dated, with the reason. They are candidates for a water v2 that does not exist and may never — and the list of the usual temptations is written down so they are recognized: `impl Trait for T` and traits, closures, `==` on aggregates, an effect system, async. Every one of them is the borrow-checker's gravity well, and water exists on the near side of it.

## 20.3 Where this came from

Two designs preceded water. The first was twenty documents and eight IRs — ownership, effects, contracts, an SMT ladder, a dataflow IR, a verification spine, an atlas, a debugger, a profiler — abandoned for being unbuildable alone and for not being the language its author wanted to use. The second, `t3`, cut all of that down to *C with sum types, exhaustive match, real modules, one tool, and errors that teach*, and was frozen and audited — but it was C's surface, and its author is a creature of Rust's surface and would not have reached for it daily.

water is the resolution: **t3's memory model and trap discipline and interpreter, under Rust's skin.** Twenty-three keywords. Three IRs. Two undefined behaviors, an interpreter that watches both, and — the thing that separates it from the C-flavored line it grew from — `struct`, `enum`, `match`, `impl`, method chaining, tail expressions, `?`, and `()`, which are why its author will open a `.wtr` file on purpose.

## 20.4 What remains, in order

1. **The corpus.** Every rule in this document, with a test. §19's examples are already normative and are the first entries. When complete, the corpus is normative and this document is a description.
2. **`waterc`.** Lexer → parser → HIR → typechecker → MIR → interpreter → LLVM. The interpreter before codegen.
3. **Three programs you actually use**, written in water.
4. **The debug allocator** (§10.2), post-1.0, never before the interpreter.
5. **Six months without adding syntax.** The only real test of "finished."

Nothing on that list is prose. That is the difference between this and the designs that came before it.

---

*Frozen 28 August 2026. The grammar does not move. Holes are errata under §20.2; everything else is a water v2 that does not exist. If this document and the interpreter disagree, this document is a bug report against the interpreter — until the corpus is complete, after which the interpreter is the language.*
