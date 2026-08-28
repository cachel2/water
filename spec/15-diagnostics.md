# 15 · Diagnostics

The one part allowed to be ambitious, because it is the reason to choose water over C, and a small language is what makes it affordable. Every diagnostic answers four questions: **what you wrote, what the compiler needed, why they don't match, and the one-line fix.**

```
error[E0107]: this `match` doesn't cover every variant of `Shape`
  --> render.wtr:22:5
   |
22 |     match shape {
   |     ^^^^^^^^^^^^ `Shape::Empty` is not handled here
   |
   = note: `Shape` has 3 variants; you handled `Circle` and `Rect`
   = note: `Shape::Empty` was declared at shape.wtr:6:5
   = fix: add an arm
   |
25 +         Empty => 0.0,
   |
   = or: add `_ => …` if you meant to ignore the rest
```

Normative rules: one real error per broken thing (recovery is per-item; cascades are bugs); the first error on any path is the true one or the compiler files a bug; every error has a stable id and `water explain E0107` prints the long form; a fix-it is mandatory where the fix is mechanical (`water fmt --fix` applies them); every diagnostic in this document has a golden test before its feature is done; ICEs print the IR stage, the dump, and the reproduction command.
