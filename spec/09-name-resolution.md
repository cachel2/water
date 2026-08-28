# 9 · Name resolution

Mechanical; never consults intent, evaluation, or (for items) declaration order.

**Scopes**, innermost out: block scope (each `{ }`, plus a `match` arm's bindings and a `for` variable); function scope (parameters and generic parameters); module scope (every item and every `use`); root scope (the contextual words of §2).

**Rules:**

1. **Items are order-independent** — the whole module is collected before any body is checked. No forward declarations.
2. **Locals are order-dependent** — a name refers to the `let` lexically above it in the same or an enclosing block.
3. **A declaration may not shadow a name in scope at that point.** Two sequential `for` loops may both use `i`; a `let x` inside a block where an outer `x` is live is a compile error naming both.
4. **A local may shadow a module item or `use`**; a later use of that name that meant the item is then a compile error naming the local.
5. **A local may not shadow a root-scope name** (`let i32 = …` is an error).
6. **`use` collisions are errors at the `use`.** No renaming (`use x as y` does not exist); qualify instead.
7. **`use` is per-module and does not re-export.**
8. **Unqualified order:** blocks innermost-out, then function scope, then module items and `use`s as one namespace, then root. First hit wins; no overloading, so never a set to choose from.
9. **Qualified paths** (`a::b::c`) resolve from the module root, then by `use` at the first segment.
10. **Variant names** resolve against the enum, qualified (`Shape::Circle`) or bare in a `match` pattern against a known subject type (§7.5) or bare in the prelude (`Some`, `None`, `Ok`, `Err`). A bare user-enum variant in expression position must be qualified.
11. **Method names** resolve against the receiver type's `impl` blocks (§5.6); associated functions as `Type::f`.
12. **No overloading, of anything.** One name, one thing.
