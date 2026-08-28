# 11 · Traps

A **trap** is defined behavior: the program stops. Not UB, not a panic, not an exception; it cannot be caught, resumed, or unwound from.

## 11.1 The complete list

| Trap | Where |
|---|---|
| integer overflow on `+ - *` | §4.2 |
| division or remainder by zero | §4.2 |
| `iN::MIN / -1`, `iN::MIN % -1` | §4.2 |
| shift amount `>=` bit width, or negative | §4.2 |
| `as!` value out of range | §4.2 |
| float→int conversion out of range, NaN, or infinite | §4.2 |
| array or slice index out of bounds | §4.5, §4.6 |
| slice range invalid | §4.6 |
| `str` slice not on a codepoint boundary | §4.9 |
| pointer difference not representable in `isize` | §4.4 |
| `alloc*` out of memory | §10 |
| `mem::copy` length mismatch or overlap | §10 |
| `assert`/`assert_eq`/`assert_ne` failure | §8.6 |
| explicit `std::trap(msg: str)` | here |

That is the complete list. Nothing else traps.

## 11.2 The mechanism

Every trap calls one function:

```water
fn water_trap(info: *const TrapInfo) -> ()      // must not return

struct TrapInfo { kind: u32, file: str, line: u32, col: u32, message: str }
```

- **Defers do not run**, in this frame or any (§7.6 rule 7).
- **`water_trap` must not return, and if it does, the program still dies.** water has no `never` type, so the signature cannot say so; instead the compiler emits an unconditional abort after every call to `water_trap` (`ud2`/`udf`/LLVM `unreachable`). A returning handler executes one more instruction and stops — two bytes, and the UB budget stays at two.
- **Hosted default:** `std` provides `water_trap` as a **weak symbol** that writes `message` to fd 2 and calls `abort()` (exit 134 on POSIX).
- **Freestanding:** the `core`-only build provides **no** `water_trap`. You define it, or the link fails naming it. No silent fallback, no trap that quietly becomes a no-op on a microcontroller.
- **Overriding:** a strong `water_trap` replaces the default anywhere. One symbol is the whole extension mechanism.
- **The message is pre-rendered** into a static buffer, no allocation — a trap must work when the heap is gone.
- **Under the interpreter**, traps carry the value and allocation history that make them readable (§16); the exit code is the same.
