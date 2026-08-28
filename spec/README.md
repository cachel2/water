# water-spec

The water language spec, one file per part. Text is verbatim from v1.0. Section
numbers are kept so every cross-reference in the text still resolves.

## Parts

| § | File | What it settles |
|---|---|---|
| 1 | [01-what-water-is.md](01-what-water-is.md) | The pitch. The two UBs. Non-goals |
| 2 | [02-lexical-structure.md](02-lexical-structure.md) | Keywords, literals, operators, comments |
| 3 | [03-surface-and-grammar-shape.md](03-surface-and-grammar-shape.md) | Why the parser stays trivial. Anti-footgun rules |
| 4.1 | [04-01-primitives-and-unit.md](04-01-primitives-and-unit.md) | Primitives, `bool`, `()` |
| 4.2 | [04-02-integers.md](04-02-integers.md) | Trapping arithmetic and the `as` table |
| 4.3 | [04-03-const.md](04-03-const.md) | `*const` and `[]const`. Why const is not transitive |
| 4.4 | [04-04-pointers.md](04-04-pointers.md) | Pointers, `Option<*T>`, pointer arithmetic |
| 4.5, 4.6 | [04-05-arrays-and-slices.md](04-05-arrays-and-slices.md) | Arrays, slices, bounds checks |
| 4.7, 4.8 | [04-06-structs.md](04-06-structs.md) | C layout, field defaults, `packed` |
| 4.9 | [04-07-str.md](04-07-str.md) | `str` and its UTF-8 invariant |
| 4.10–4.12 | [04-08-enums.md](04-08-enums.md) | Plain enums, payload enums, `Option`, `Result` |
| 4.13 | [04-09-generics.md](04-09-generics.md) | Monomorphization without bounds. Turbofish |
| 4.14 | [04-10-function-pointers.md](04-10-function-pointers.md) | Function pointers and why there are no closures |
| 4.15 | [04-11-c-abi-subset.md](04-11-c-abi-subset.md) | Which types cross the FFI boundary |
| 5.1–5.5 | [05-01-items-fn-const-static.md](05-01-items-fn-const-static.md) | Items, tail returns, `const`, `static` |
| 5.6 | [05-02-impl-and-methods.md](05-02-impl-and-methods.md) | `impl`, `self`, method resolution, chaining |
| 6 | [06-declarations-and-inference.md](06-declarations-and-inference.md) | `let`, `undefined`, definite-init, inference |
| 7.1–7.4 | [07-01-blocks-if-while-for.md](07-01-blocks-if-while-for.md) | Blocks, the semicolon, `if`, `while`, `for` |
| 7.5 | [07-02-match.md](07-02-match.md) | Exhaustiveness, arms, patterns |
| 7.6, 7.7 | [07-03-defer-and-question.md](07-03-defer-and-question.md) | `defer` and `?` |
| 8.1 | [08-01-precedence.md](08-01-precedence.md) | The precedence table |
| 8.2 | [08-02-places.md](08-02-places.md) | Places. Reassignability is not writability |
| 8.3–8.5 | [08-03-evaluation-order.md](08-03-evaluation-order.md) | Left to right, everywhere. Equality |
| 8.6 | [08-04-builtins.md](08-04-builtins.md) | `print`, `assert`, the format language |
| 9 | [09-name-resolution.md](09-name-resolution.md) | Scopes and the twelve rules |
| 10, 10.1 | [10-01-memory.md](10-01-memory.md) | `alloc`, `free`, volatile |
| 10.2 | [10-02-debug-allocator.md](10-02-debug-allocator.md) | `--sanitize`, graded honestly |
| 11 | [11-traps.md](11-traps.md) | The trap list and `water_trap` |
| 12 | [12-ffi.md](12-ffi.md) | `extern "c"` |
| 13 | [13-modules-and-build.md](13-modules-and-build.md) | Filesystem modules, manifest, CLI, `main` |
| 14 | [14-tests.md](14-tests.md) | `test` items and the double run |
| 15 | [15-diagnostics.md](15-diagnostics.md) | What every error message must answer |
| 16 | [16-interpreter.md](16-interpreter.md) | Sanitizer, fast loop, const evaluator |
| 17 | [17-compiler.md](17-compiler.md) | The pipeline. IR-1 and IR-2 |
| 18 | [18-grammar.md](18-grammar.md) | The complete EBNF |
| 19 | [19-examples.md](19-examples.md) | Five normative programs |
| 20 | [20-01-the-freeze.md](20-01-the-freeze.md) | What may change and what remains |
| 20.2 | [20-02-errata.md](20-02-errata.md) | Errata and the authority order |

## Notes

Packaging only. No rule was added, removed or reworded.

§6.4 is cited by §1.1 and §17 but has no body in the source. A labelled note in
[06-declarations-and-inference.md](06-declarations-and-inference.md) restates
the rule those two sections already give so the citation resolves.
