# 12 · FFI

```water
extern "c" fn write(fd: i32, buf: *const u8, count: usize) -> isize;
extern "c" fn malloc(size: usize) -> Option<*u8>;
extern "c" fn free(p: Option<*u8>) -> ();
```

`"c"` is the only ABI string in v1. Every parameter and the return must be C-ABI (§4.15); a `[]T`, `str`, payload enum, or generic in an `extern` signature is a compile error naming the type and the C-compatible struct to define instead. `extern` functions are trusted assertions — the compiler checks the call site, not the callee. `const` does not survive the boundary usefully; `mem::unconst` is the shim where a C API takes a mutable pointer to your `[]const T`. `extern` functions may not be called from a constant expression. `pub` functions are exported unmangled with the C ABI when their signatures are C-ABI; a `pub` function with a non-C-ABI signature is simply not exported, callable from water only. There is no runtime to initialize; link with `cc`.
