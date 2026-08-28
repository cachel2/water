# 13 · Modules and build

## 13.1 The filesystem is the module tree

```
src/
├── main.wtr          → module app
├── lex.wtr           → app::lex
└── parse/
    ├── expr.wtr      → app::parse::expr
    └── stmt.wtr      → app::parse::stmt
```

No `mod`, no `mod.rs`, no headers, no include guards, no forward declarations, no ODR, no preprocessor. A file is a module named by its path; `pub` marks what leaves it; `use` brings names in:

```water
use std::io;
use app::lex::{Token, TokenKind};
```

Moving a file is the refactor. **Circular imports between modules are legal** (no headers to order; items collected before checking). Circular *types* need a pointer, as in C. A directory with no same-named file beside it is a module with no items of its own.

## 13.2 One manifest

```toml
[project]
name = "doclint"
version = "0.1.0"

[deps]
shelf = { git = "…", rev = "9f2c…", hash = "blake3:…" }

[targets]
doclint  = { kind = "bin", root = "src/main.wtr" }
firmware = { kind = "freestanding", root = "src/boot.wtr", link = "layout.ld" }

[profiles.release]
opt = 3        # speed only; semantics never change with profile
```

No per-module config, no build scripts, ever. Dependencies are paths or git revisions pinned by content hash; no registry in v1; builds are offline (`water fetch` is the only networked command, never at compile time). `kind = "freestanding"` selects `core`-only, no default `water_trap` (§11.2), no default `main` wrapper, the linker script from the manifest — a target kind, not a fork.

## 13.3 The CLI

```
water new <name>     water build [--release] [--sanitize[=pages]]
water run [--interp] water test [filter]     water fmt      water check
water explain <id>   water emit <stage>      water iface <module>
water fetch
```

`water check` is type-check only, no codegen — the fast loop. `water emit ast|hir|mir|llvm` dumps any stage. `--sanitize` selects the debug allocator (§10.2) and changes no semantics.

## 13.4 Interface views

`water iface app::lex` renders the module's public surface — signatures and doc comments, nothing else. Generated, therefore never stale. It is also the incremental key: edit a body and dependents do not rebuild; change a signature and exactly the true dependents do. C's one good idea — read the interface without the implementation — as a view, not a hand-maintained file.

## 13.5 `main`

Exactly one of these in the root module of a `bin` target:

```water
fn main()
fn main(args: []str)
fn main() -> i32
fn main(args: []str) -> i32
```

`args[0]` is the program name. A `()` main exits 0. Anything else is a compile error naming the four.
