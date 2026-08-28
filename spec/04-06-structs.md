# 4.7–4.8 · Structs, the empty-aggregate rule, and `packed`

## 4.7 Structs and empty-aggregate rule

```water
struct Point { x: f64, y: f64 }
struct Header { magic: u32, version: u16, payload: []u8 }
```

**Layout is always C layout.** Field order preserved; alignment and padding per the platform C ABI. There is no `repr` attribute because there is nothing to choose: every struct is FFI-compatible by construction. Structs are copied on assignment; no copy constructors, no move semantics, no destructors.

```water
let p = Point { x: 1.0, y: 2.0 };
let q = p;              // a copy. both valid. this is C.
```

**Field defaults:**

```water
struct Config {
    port: i32 = 8080,
    timeout_ms: u32 = 5000,
    host: str,                 // no default: must be given
    verbose: bool = false,
}

let c = Config { host: "localhost" };
let d = Config { host: "0.0.0.0", port: 9000 };
```

A default is a constant expression (§5.4). A field **without** one must be supplied at every literal — no zero-initialization, and `Config { }` is an error naming `host`. A generic-typed field may not have a default. `undefined` is not a default. Field initializers evaluate in **source order**, then defaults in declaration order (§8.4).

**Empty aggregates are a compile error.** A `struct` with no fields, an `enum` with no variants (§4.10), and `[0]T` are rejected. Zero-sized types propagate into `size_of`, pointer steps of zero, `alloc_n::<Empty>(5)`, and slice iteration; every language that allowed them paid for years. The most restrictive answer is taken, and zero-sized types are a v2 question, unopened.

## 4.8 `packed` structs

```water
packed struct Dhcp {
    op: u8,
    htype: u8,
    xid: u32,        // offset 2, no padding
}
```

`packed` means no padding, alignment 1, field order exactly as written; unaligned accesses are emitted with the true alignment told to LLVM. **You cannot take the address of a packed field.** `&d.xid` is a compile error — the result would be a misaligned `*u32`, silently wrong on x86 and a fault on ARM. Copy it out, use it, write it back; the diagnostic says exactly that. A packed struct may not contain a non-packed struct of alignment > 1.
