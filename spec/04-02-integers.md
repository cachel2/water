# 4.2 · Integers

**Arithmetic traps on overflow in every build profile.** Debug and release have identical semantics, always. Intent is spelled: `+% -% *%` wrap, `+| -| *|` saturate.

- Division and remainder by zero **trap**.
- `iN::MIN / -1` and `iN::MIN % -1` **trap**.
- `%` takes the sign of the dividend (C99).
- Shift amount `>=` bit width **traps**; shift amount **negative** traps. The shift amount need not match the shifted type.
- `<<` **does not overflow**: bits shifted out are discarded; `1i32 << 31` is `i32::MIN`.
- `>>` on a signed type is arithmetic (sign-extending); on unsigned, logical.
- **Signed and unsigned never mix.** `i32 + u32` does not compile. Unary `-` on an unsigned type does not compile.
- There is no integer promotion. `u8 + u8` is `u8` and can trap.

**Conversions.** `as` performs an integer→integer conversion **iff the target represents every value of the source, on every supported target**. Complete table:

| From | To | `as`? |
|---|---|---|
| `uN` → `uM`, `M ≥ N` | | yes |
| `iN` → `iM`, `M ≥ N` | | yes |
| `uN` → `iM`, `M > N` | `u8 as i32`, `u32 as i64` | **yes** |
| `uN` → `iN` | | no — `as?`/`as!` |
| `iN` → any `uM` | | no — `as?`/`as!` |
| `u8`,`u16`,`u32` → `usize`; `i8`,`i16`,`i32` → `isize` | | yes |
| `usize` → `u64`; `isize` → `i64` | | yes |
| `u64` → `usize`; `usize` → `u32`; `usize` ↔ `isize` | | no — `as?`/`as!` |
| `f32` → `f64` | | yes |

```water
x as? u8       // Option<u8>  — None if it does not fit
x as! u8       // u8          — traps if it does not fit
```

Int↔float uses named functions, never `as`: `f64::from_i32`, `f64::from_u32`, `f64::trunc_to_i32` (traps on NaN, infinity, out of range), `f64::round_to_i32`, `f32::from_f64_rounded`. Pointer↔integer is `mem::ptr_from_addr::<T>(usize) -> *T` and `mem::addr_of::<T>(*const T) -> usize`. Pointer↔pointer is `mem::ptr_cast::<T, U>(*T) -> *U`. These three plus `mem::unconst` (§4.3) are the entire escape-hatch surface, and they are functions so that `grep` finds every one.
