# 10.2 · The debug allocator — `std`, not the language

The interpreter (§16) catches both UBs with full history, but only along paths your tests execute. `std` closes part of the gap on native builds, selected by a flag, changing **no semantics**:

```
water build --sanitize          redzones, quarantine, headers, leak report at exit
water build --sanitize=pages    one guard page per allocation (hosted only)
```

Graded honestly (a library allocator sees `alloc`/`free`, never loads and stores, so it checks only at those two moments — that is the difference from ASAN, and it is load-bearing):

| | Detection |
|---|---|
| Double free, invalid free, invalid-pointer free, leaks | **reliable** |
| Adjacent overflow/underflow, use-after-free **write** | **probable, deferred** (checked at `free`/quarantine eviction) |
| Use-after-free **read**, out-of-bounds past the redzone, stack, globals | **not caught** |

`--sanitize=pages` turns the "not caught" rows into hardware faults at the instruction, at the cost of a page per allocation and a large slowdown — the mode for the one test that is lying to you. The fault is a SIGSEGV, not a water trap (no signals — §1.3); `std` may render it via `extern` (the escape hatch §12 permits and counts) or you reproduce under `--interp`. Allocation sites are captured as return addresses (frame pointers, forced by `--sanitize`) and resolved by `water dbg`/`addr2line`; **detection is native, the story is the interpreter's.** Roughly 400 lines of `std`, scheduled after the interpreter, never before it (§20).
