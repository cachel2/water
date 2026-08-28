# 4.4 · Pointers

`*T` is a pointer to `T`. **It is never null.** It may dangle — the concession — but the compiler will never hand you a null one, and there is no `NULL`, `nil`, or `0` pointer in the language.

Nullable is a type: **`Option<*T>` is exactly pointer-sized, `None` represented as the all-zero address.** Normative, the entire null-safety story, zero bytes and zero cycles of cost. The same holds for `Option<[]T>` (`None` is a null `ptr` field) and `Option<fn(...)->R>`.

```water
let p: *Node = &node;
let q = *p;          // deref
p.next = &other;     // `.` auto-derefs exactly one level for field access
```

`.` auto-derefs one level for field access and for method receivers (§5.6). `*p` is the value; `&x` is the address (a place; §8.2). Taking the address of a temporary is a compile error naming the local to declare.

**Pointer arithmetic exists** (`p + 1` steps by `size_of::<T>()`), because this is C — and **the arithmetic itself is always defined**: `p + 1000` on a ten-element allocation is a perfectly good address. It is the *access* that is UB-1 (§1.1); C makes even the arithmetic undefined and water need not. Comparing pointers into different allocations is **defined** (address comparison).

**Subtraction is defined as the truncating element difference over the mathematical address difference.** Let `D` be the true integer difference of the two addresses — `addr(p)` minus `addr(q)` computed in ℤ, which may be negative and is never itself a water `usize` subtraction (so it cannot overflow or trap on its own; `addr` is a concept here, not `mem::addr_of` followed by `-`). Then `p - q` is `D / size_of::<T>()`, truncated toward zero, as an `isize`. Within one array `D` is always a multiple of the element size, so the division is exact and this is C's meaning; **across allocations, or between misaligned pointers, the remainder is dropped.** The operation **traps iff the result — the element count, after dividing — does not fit in `isize`** (§11.1 lists the trap as "the difference," i.e. the result, so the check is applied to the quotient, not to `D`). On a 32-bit target this matters: a byte difference `D` may exceed `isize` while the element count `D / size_of::<T>()` fits, and the count is what is returned. The result is meaningful only within one array; elsewhere it is a defined but useless number, like the address comparison above.

*See erratum E-002 (§20.2).*
