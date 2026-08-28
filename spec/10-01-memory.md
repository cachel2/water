# 10 · Memory

`malloc` and `free`, typed:

```water
mem::try_alloc<T>() -> Option<*T>
mem::try_alloc_n<T>(n: usize) -> Option<[]T>
mem::alloc<T>() -> *T               // traps on OOM
mem::alloc_n<T>(n: usize) -> []T    // traps on OOM
mem::try_realloc_n<T>(old: []T, n: usize) -> Option<[]T>
mem::free<T>(p: *T)
mem::free_n<T>(s: []T)
mem::copy<T>(dst: []T, src: []const T)      // traps on length mismatch or overlap
mem::move<T>(dst: []T, src: []const T)      // memmove; overlap allowed
mem::set<T>(dst: []T, value: T)
```

Trapping forms are the default because the honest failure mode for most programs is death; `try_` forms survive OOM. Allocated memory is uninitialized (poison under the interpreter). `alloc_n(0)` returns an empty slice with a non-null aligned pointer; freeing it is legal and does nothing. `free`/`free_n` take `*T`/`[]T`, not the `const` forms — `const`'s single most useful catch (§4.3).

**`free_n` frees by pointer; the length is ignored.** Freeing identifies an allocation by its `ptr` field alone — the `len` is not part of an allocation's identity, exactly as C's `free` takes only a pointer and does not care how much of the block you were using. So `free_n` on a **subslice** of an allocation is legal precisely when the subslice's `ptr` equals the allocation's base pointer, whatever its `len`: `read_all` returning `Ok(buf[0..n])` and the caller doing `defer mem::free_n(data)` (§19.4) frees the whole `alloc_n` block, because `buf[0..n]` shares `buf`'s base pointer. A subslice whose `ptr` is *not* the base pointer — `buf[1..n]` — is UB-2 (a pointer not returned by an `alloc*` call), and the interpreter, which records base pointers, reports it with the allocation's history. This is the rule the interpreter needs and the reason §19.4 is valid; without it the interpreter would see a `len` mismatch and have no defined response. *(Erratum E-004, §20.2.)*

No ownership tracking, no lifetime, no generation counter, no arena in the language. **An arena is thirty lines of water in `std`:**

```water
struct Arena { buf: []u8, used: usize }
impl Arena {
    fn new(cap: usize) -> Arena { Arena { buf: mem::alloc_n::<u8>(cap), used: 0 } }
    fn alloc<T>(self: *Self) -> *T { … }
    fn reset(self: *Self) { self.used = 0; }
    fn free(self: *Self) { mem::free_n(self.buf); }
}
```

Use it for your AST — with methods and chaining, it reads well. It is a library, not a language feature, and that distinction is the whole lesson of the line water descends from.

## 10.1 Volatile

```water
mem::read_volatile<T>(p: *const T) -> T
mem::write_volatile<T>(p: *T, value: T)
```

Volatile is an **operation, not a qualifier.** The answer to "is this access volatile?" is at the call site, one word long, never traced through a type. These lower to LLVM volatile load/store: never elided, reordered, merged, or split. **Volatile is not atomic and implies nothing about threads** — a promise about the number and width of accesses, which is what MMIO needs and all it needs. `T` must be a primitive or a pointer; a volatile aggregate access is a compile error naming the fields to do individually.
