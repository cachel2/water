# 5.6 · `impl` — inherent methods, no traits

```water
impl Shape {
    fn area(self) -> f64 { … }
    fn scaled(self, k: f64) -> Shape { … }
    fn grow(self: *Self, k: f64) { … }
    fn new(r: f64) -> Shape { Shape::Circle(r) }     // no self: an associated function
}
```

An `impl Type { … }` block attaches **inherent** functions to a type. There is **no `impl Trait for Type`** — no traits, no coherence, no orphan rule, no `dyn`, no specialization. That is the entire departure from Rust's `impl`, and it is what keeps the language small.

**`self` and `Self`:**

- Inside `impl T`, `Self` is `T`.
- A function whose first parameter is `self` is a **method**; otherwise it is an **associated function**, called `T::f(...)`.
- The `self` parameter is written in full — `self: Self`, `self: *const Self`, or `self: *Self` — with **one shorthand: bare `self` means `self: *const Self`**, the read-through-pointer form, which is the common case and the one that makes method chaining on reads free.
- There is no `&self`/`&mut self` spelling; water has no references, only pointers.

**Method resolution and auto-ref.** For `r.m(args)` where `r : R` and `m`'s self type is `S`, resolution is mechanical, one level, no search beyond the receiver's own type and its `impl` blocks (no traits to consult):

| `r` is | `S` is | Action |
|---|---|---|
| `T` (a place) | `*const T` or `*T` | auto-take the address |
| `*T` or `*const T` | `T` | auto-deref (copy the value out) |
| `*T` | `*const T` | coerce (mut → const) |
| exact match | | direct |

Auto-taking an address requires `r` to be a place (§8.2); `f().m()` where `f()` returns a value and `m` wants `*Self` is a compile error naming the local to bind. Chaining works because a method returning `*Self` (or any pointer, or any value) is itself a receiver for the next `.`:

```water
vec.push(3).push(4).push(5);          // push(self: *Self, ...) -> *Self
let n = shape.scaled(2.0).area();     // value → value → value
```

`impl` blocks for a type may be split across the module; a method may not be defined twice; a method may not share a name with a field.
