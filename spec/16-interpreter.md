# 16 · The interpreter

One program, three jobs — and the piece that makes manual memory defensible.

**1 — The sanitizer.** The interpreter models the heap: every allocation has an id, size, allocation site, live flag, and byte-level init map. Use-after-free, double-free, leaks (every live block at exit, with its site), reads of uninitialized bytes, raw-pointer OOB (which the native build cannot check), and misalignment — the entire remaining UB budget, caught with full history, every `water test`:

```
trap: use after free
  --> parse.wtr:88:17
   |
88 |     let t = tok.text[0];
   |             ^^^^^^^^ `tok.text` points into allocation #412
   |
   = allocated: parse.wtr:41:20  (alloc_n::<u8>(len), 24 bytes)
   = freed:     parse.wtr:79:5   (free_n(tok.text))
   = read here: 9 instructions later, offset 0
   = the slice was copied at parse.wtr:74:9 — the copy outlived the free
```

The position is not *trust me, I test well*; it is *test well, and the interpreter is watching while you do*.

**2 — The fast loop.** `water run --interp` skips LLVM; the edit-run cycle is milliseconds. It is also how you develop `waterc` before codegen exists, which is why it is built first.

**3 — The constant evaluator.** `const` and `static` initializers are interpreted MIR under §5.4's limits. No second semantics to keep in sync.

The interpreter runs MIR, sees every trap and check as an instruction, and is kept small and boring — the most-trusted code in the project. Interpreter/native disagreement is a compiler bug; the reproducer joins the corpus permanently.

## 16.1 The detection ladder

| Tier | Runs on | Catches | Cost |
|---|---|---|---|
| **Interpreter** (`water test`, `--interp`) | Every path your tests take | Both UBs, leaks, uninitialized reads, misalignment, raw-pointer OOB — with full history | ~50–100× |
| **`--sanitize`** | Every native run: fuzzing, staging, load | 4 classes reliably, 2 probably-late, 3 missed (§10.2) | ~2× |
| **`--sanitize=pages`** | The one test that is lying to you | UAF and OOB immediately, at the instruction | slow; a page per allocation |
| **`--release`** | Production | nothing | 0 |

Down the table: **precision falls, coverage rises, price falls.** That is the whole trade of manual memory, priced, instead of assumed away by a checker the language does not have.
