# 8.6 · The builtins

Five functions are compiler-special-cased, because they need structural knowledge the language does not otherwise expose:

```water
io::print(fmt: str, ...)      io::eprint(fmt: str, ...)
assert(cond: bool, msg: str)  assert_eq<T>(a: T, b: T)   assert_ne<T>(a: T, b: T)
```

They are **not extensible and not first-class**: you cannot take their address or write another. That is the cost of no traits, paid in one place, visibly. `size_of::<T>()`, `align_of::<T>()`, `offset_of::<T>(field)` are constant-expression builtins returning `usize`.

**Structural `print`.** Because every type's layout is known at compile time, `io::print("{}", x)` prints any concrete value — structs, enums with named payloads, slices, `Option`, `Result`. **Pointers print as addresses and are never followed** (`Node { id: 4, next: 0x7f… }`), which makes cycles impossible without a cycle detector. The format mini-language, complete:

```
{}          the value, structurally
{:x} {:X}   hexadecimal, lower / upper       (integers and pointers)
{:b}        binary                            (integers)
{:8}        min width 8, right-aligned        (numbers)
{:08}       min width 8, zero-padded          (numbers)
{:.3}       3 digits after the point          (floats)
{:8.3}      both                              (floats)
{:08x}      width and radix combine
{{  }}      literal braces
```

Specifiers and arity are type-checked at compile time; `{:x}` on a `str` is a compile error. `printf`'s failure mode is not expressible. Not extensible — a custom rendering is a method (`shape.to_str(buf)`), which methods now make ergonomic. That extensibility gap is the price of no traits, paid once.
