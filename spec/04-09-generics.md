# 4.13 · Generics — without bounds

Monomorphized, **and without bounds**. That is the whole design.

```water
struct Vec<T> { ptr: *T, len: usize, cap: usize }

impl Vec<T> {
    fn get(self, i: usize) -> Option<T> { … }
    fn push(self: *Self, x: T) -> *Self { …; self }
}
```

A type parameter `T` supports exactly four operations: **copy it, take its address, store it, pass it.** Not `a + b`, not `a == b`, not `a < b`, not `hash(a)`, not `a.method()` on an unknown method, not `print("{}", a)` on a `T` (though `print` on a concrete monomorphization is fine — §8.6).

*Why no bounds:* bounds require traits; traits require coherence, orphan rules, dispatch, associated types, `dyn`, and eventually specialization — Rust, and the borrow-checker-adjacent weight water exists to avoid. Bounds also destroy diagnostics: an unbounded `T` fails at the definition, where the error is local; a bounded `T` fails at the instantiation, which is where C++ template errors come from.

The consequence, stated rather than discovered:

```water
// Does not compile. `T` cannot be compared.
fn max<T>(a: T, b: T) -> T { if (a > b) { a } else { b } }
```

**Containers are generic; algorithms are not.** Anything that compares, hashes, or combines takes a function pointer — like `qsort`, but typed:

```water
fn sort<T>(xs: []T, cmp: fn(*const T, *const T) -> i32);
fn find<T>(xs: []const T, needle: *const T, eq: fn(*const T, *const T) -> bool) -> Option<usize>;
```

`Map<K, V>` takes a hash function and an equality function at construction. This is the honest cost of no traits, and it is the cost C already pays, minus the `void*` casts. `std` ships the common comparators (`cmp_i32`, `cmp_str`, `hash_str`, `eq_str`, …) so the cost is one argument, not one function you write.

**Inference and turbofish.** Generic arguments are inferred from argument types and from the expected type at the call site. Where they cannot be, they use `::<>`:

```water
let buf: []u8 = mem::alloc_n(4096);        // T = u8, from the annotation
let buf2   = mem::alloc_n::<u8>(4096);     // T = u8, explicit
```

Generic parameters may not be shadowed and may not share a name with any type in scope.
