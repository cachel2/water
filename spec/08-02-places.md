# 8.2 · Places

A **place** is: an identifier bound by `let`/`let mut`, a parameter, a `match`-arm pattern binding, or a `for` loop variable; `p.f`; `a[i]`; `*p`; or a parenthesized place. Everything else is a value.

**Two independent questions govern assignment, and conflating them is a recurring error this document made three times before stating them apart:**

*Question 1 — may this binding be reassigned?* A binding may be reassigned iff it is a mutable place: a `let mut`, a `mut` parameter (§18), a `static mut`, or the induction variable of a C-style `for (i = 0; …)` (a `let mut` by §7.3). An immutable `let`, a non-`mut` parameter, a `match`-arm binding, and a `for-in` variable may **not** be reassigned — `x = …` on them is a compile error. This question is about the *name*.

*Question 2 — may I write through this pointer or into this place?* A write to `*p`, or to a field/index reached through `*p`, is legal iff `p`'s type is `*T` (not `*const T`) — decided by the **pointer's constness**, per §4.3 rule 2 (`const` is not transitive; C's semantics). This has **nothing to do with whether the binding holding `p` is mutable.** A non-`mut` parameter `self: *Self` cannot be repointed (Question 1), yet `*self`, `self.field`, and `&self.field` (yielding `*T`) are all writable, because `self`'s *type* is a mutable pointer (Question 2). §19.5's `fn putc(self: *Self, …)` writing `&self.dr` is exactly this case, and it is the reason `const` was made non-transitive in the first place.

The left side of an assignment must therefore be either a reassignable binding (Question 1) or a write through a non-const pointer / into a place reached from one (Question 2). `&` requires a place, and yields `*const T` on a `const` place or `*T` on a non-const one — where "const place" means one reached through a `*const` pointer or a `[]const` slice, again independent of binding mutability. `&` of a temporary, a literal, a call result, or a packed field (§4.8) is a compile error naming the local to declare.

Match-arm bindings and the `for-in` variable are places because they occupy a storage slot with an address, exactly like a `let`; this is what lets `s.area()` auto-take `&s` in `match … { Some(s) => s.area() }` and in `for (s in xs) { s.area() }` (§5.6). Under `for (*x in xs)`, `x : *T` (or `*const T` if `xs` is `[]const T`); the `x` binding is immutable (Question 1: you cannot repoint it) while `*x` is writable iff the slice was not const (Question 2) — the two questions, side by side, in one construct.

*See errata E-001, C-002, C-003 (§20.2).*
