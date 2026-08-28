# 4.10–4.12 · Enums, `Option`/`Result`, and bit operations

## 4.10 Enums

Plain enums are C enums with a real type:

```water
enum Color { Red, Green, Blue }
enum Errno : i32 { NotFound = 2, Denied = 13 }
```

Without `: T`, the discriminant type is the smallest unsigned type that fits and explicit discriminants are not allowed. With `: T`, the type is `T`, discriminants may be given, and unspecified ones continue from the previous (C's rule). Duplicate discriminants are an error. An enum with no variants is a compile error (§4.7).

Payload enums are tagged unions — the feature C should have had:

```water
enum Shape {
    Circle(radius: f64),
    Rect { w: f64, h: f64 },
    Empty,
}
```

Payload fields are **named at the declaration**, so every diagnostic, every `print`, and every dump speaks in your words: `BadDigit(pos: 4)`, not `BadDigit(4)`. The discriminant is not addressable and the payload is reachable only through `match`. Layout is `{ tag, union }`, tag sized to the variant count, whole aligned to the strictest member. The only guaranteed niche optimizations are those in §4.4. A payload enum has no C ABI (§12).

## 4.11 `Option<T>` and `Result<T, E>`

Library types, not language types — ordinary payload enums in `core`:

```water
enum Option<T> { Some(value: T), None }
enum Result<T, E> { Ok(value: T), Err(error: E) }
```

`Some`, `None`, `Ok`, `Err` are in the prelude. What the *language* provides is `?` (§7.7) and the layout guarantees of §4.4. This is why generics exist at all: without them these would be compiler builtins, and then every container would be too.

## 4.12 Bit operations replace bit fields

There are no bit fields (§1.3). Wire and register bit-packing is masks and shifts, which are portable and one line. `packed` handles byte layout; masks handle sub-byte.
