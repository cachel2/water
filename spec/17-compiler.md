# 17 · The compiler

`waterc`, written in Rust, as a library with a thin binary — `water fmt`, `water check`, the interpreter, and a future LSP all consume the same crates.

```
source (.wtr)
  │  lex + parse                    (recursive descent + Pratt, 1 token, no backtracking)
  ▼
 AST      exact source shape; what fmt and doc read
  │  resolve names (§9), desugar (defer, ?, method calls, for-in, if/match values)
  ▼
 HIR      every identifier bound; sugar gone; methods lowered to functions with an explicit self
  │  type check, monomorphize, definite-init, exhaustiveness
  ▼
 MIR      typed CFG; every check, trap, defer, and block-value explicit
  │        ←── the interpreter runs HERE
  ▼
 LLVM IR → LLVM → native
```

**Three IRs.** AST, HIR, MIR. Each has a stable dump (`water emit`) and a verifier. Generics monomorphize at HIR→MIR; methods desugar to functions taking `self` as an ordinary first parameter, so MIR has no notion of "method" — chaining and auto-ref are entirely a front-end concern. **`waterc` feeds LLVM plain MIR-with-allocas and lets `mem2reg` build SSA**, which is why swapping the backend (QBE, Cranelift) is a self-contained ~2k-line job and not a rewrite. Two backend-independent obligations, inherited from the audit and restated so an implementer cannot miss them:

- **IR-1 — no TBAA metadata.** §4.3 says water has no strict-aliasing rule; that is only true if LLVM is told nothing, or every `ptr_cast` becomes a miscompilation at `-O2`. Some alias analysis is lost; that is the price of the claim.
- **IR-2 — `freeze` on unprovable loads.** A load from a stack location definite-init could not prove written must be `freeze`d, or the naive `alloca`→`load` yields LLVM `poison` and the language's "unspecified, not undefined" promise (§6.4) becomes false. Heap from `alloc*` is already safe; only `undefined` locals need it.

Crates: `water_lex`, `water_parse`, `water_hir`, `water_types`, `water_mir` (+ interpreter), `water_llvm` (via inkwell), `water_diag`, `water_driver`. **Build order:** lexer → parser → HIR → typechecker → MIR → **interpreter** → LLVM. The interpreter before codegen. Self-hosting is plausible for a language this size and is a long-term goal, not a v1 requirement.
