# 4.15 · The C-ABI subset

A type is **C-ABI** iff it is: any primitive (`iN`, `uN`, `isize`, `usize`, `f32`, `f64`, `bool`); `()` as a return type (mapped to C `void`); `*T`/`*const T` where `T` is C-ABI or an opaque struct; `Option<*T>`/`Option<*const T>`; `fn(...)->R` and `Option<fn(...)->R>` with C-ABI parameters and return; a `struct`/`packed struct` all of whose fields are C-ABI; or a plain `enum` with an explicit `: T` repr.

Nothing else. `[]T`, `str`, payload enums, any other `Option<T>`, and any generic instantiation are **not** C-ABI, and using one in an `extern` declaration is a compile error naming the type and the reason (§12).

**`Option<*T>` is in the list and is the only generic that is.** §4.4 makes it normatively identical to a C pointer, null included. Excluding it would make `malloc` undeclarable and force lying with `*T` (never null) that dereferences null on the first OOM. The rule is "types with a defined C ABI"; `Option<*T>` has one.
