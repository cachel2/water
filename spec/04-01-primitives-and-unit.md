# 4.1 · Primitives and `()`

**§4 preamble.** Every type is known at compile time. No runtime type information, no dynamic dispatch, no `void*`-shaped hole. Every value is bitwise-copyable: assignment and argument passing copy, as in C. There is no move semantics and therefore no `Copy` marker — everything is `Copy`.

| Kind | Types |
|---|---|
| Signed | `i8 i16 i32 i64 isize` |
| Unsigned | `u8 u16 u32 u64 usize` |
| Float | `f32 f64` |
| Other | `bool`, `()` |

`bool` is one byte, values 0 and 1 only, and does not convert to an integer.

**`()` is the unit type** — one value, written `()`, carrying no information. It is not C's `void` ("no value"); it is "exactly one value, and it is uninteresting." That difference is why `()` composes where `void` cannot: `Result<(), IoError>` is the honest type of *"can fail, returns nothing useful,"* which C can only express by overloading an error code. A function with no `->` returns `()`. `let x = ();` is legal and useless. `()` can be the `T` in a generic, the element of an array, the `Ok` payload of a `Result`.

`isize`/`usize` are pointer-sized, and **normatively 32 or 64 bits** — water does not target 16-bit or 128-bit address spaces, and the `as` table (§4.2) depends on this.
