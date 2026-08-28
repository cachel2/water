# 4.14 · Function pointers — no closures

`fn(A, B) -> R` is a function pointer. A function's name used as a value is a pointer to it. **There are no closures**: no captures, no environment. If you need state, pass a typed context pointer explicitly.

```water
fn cmp_i32(a: *const i32, b: *const i32) -> i32 {
    if (*a < *b) { -1 } else if (*a > *b) { 1 } else { 0 }
}
sort(nums, cmp_i32);
```

This is the deliberate line between water and Rust: closures are what force `Fn`/`FnMut`/`FnOnce`, which force capture analysis, which force the borrow checker. water declines the whole chain. Function pointers are never null; nullable is `Option<fn(...)->R>`.
