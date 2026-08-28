# 4.5–4.6 · Arrays and slices

## 4.5 Arrays

`[N]T` is a value type of `N` elements; `N` is a constant expression, and `N == 0` is a compile error (§4.7's zero-sized rule). **Arrays do not decay to pointers.** They are copied on assignment and on argument passing. `a[..]` produces `[]T`. `a.len` is `N`, a compile-time `usize` constant. Indexing is bounds-checked; the check is elided when the compiler proves the index in range (constant index; loop induction variable with a proven bound). An elided check is not a semantic change — an out-of-range index traps either way.

## 4.6 Slices

`[]T` is a fat pointer, layout normative:

```water
struct { ptr: *T, len: usize }
```

The type C never had and the reason half of C's CVEs exist.

```water
let buf: [16]u8 = undefined;
let s: []u8 = buf[..];       // whole array
let t: []u8 = buf[2..10];    // len 8
let u: []u8 = buf[2..];      // to the end
let v: []u8 = buf[..10];     // from the start
s.len                        // usize
s[i]                         // bounds-checked
```

Slicing checks `start <= end && end <= len` and **traps** otherwise. `a[i..i]` is legal and empty. Slices carry no ownership; a slice into freed memory is a dangling slice — UB-1, and what the interpreter is for.
