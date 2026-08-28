# 4.3 · `const`

`const` modifies **pointers and slices only**. There is no `const T` standalone type and no const locals — a `let` binding is already immutable (§6.1), and `let mut` opts into mutation.

```water
*const T     // pointer you may not write through
[]const T    // slice you may not write through
```

Complete rules:

1. **Coercion is one-way and implicit.** `*T` → `*const T`, `[]T` → `[]const T`. The reverse never happens implicitly and no cast performs it; the escape hatch is `mem::unconst::<T>(*const T) -> *T`, deliberately ugly, for FFI shims.
2. **Not transitive.** Given `*const Node`, the field `p.next` has type `*Node` and you may write through it. C's semantics, stated rather than discovered.
3. **A capability, not a guarantee, and it buys zero optimization.** `*const T` means *you* cannot write through this pointer. It does not mean nobody can. No aliasing rule stands behind it, no `restrict`, no `noalias`; LLVM is told nothing. It catches mistakes at compile time and makes no code faster. C promised more here and never delivered it either.

For one bit in the type: `free_n` takes `[]T`, so you cannot free a borrowed slice; string literals cannot be written through (§4.9); and a signature says whether it reads or writes.
