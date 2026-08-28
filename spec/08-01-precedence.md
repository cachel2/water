# 8.1 · Precedence — the complete table

High to low. Every operator appears exactly once.

| # | Operators | Associativity |
|---|---|---|
| 1 | `.` (field / method) · `[]` · `()` (call) · `?` (postfix) | left |
| 2 | `-` `!` `~` `&` `*` (prefix unary) | right |
| 3 | `as` `as?` `as!` | left |
| 4 | `*` `/` `%` `*%` `*\|` | left |
| 5 | `+` `-` `+%` `-%` `+\|` `-\|` | left |
| 6 | `<<` `>>` | left |
| 7 | `&` | left |
| 8 | `^` | left |
| 9 | `\|` | left |
| 10 | `==` `!=` `<` `<=` `>` `>=` | **non-associative** |
| 11 | `&&` | left |
| 12 | `\|\|` | left |
| 13 | `..` `..=` | **non-associative** |

Assignment and `,` are not in this table (statement and separator). Prefix `&` on a `const` place yields `*const T`; on a non-const place, `*T`. Level 10 being non-associative is what makes `a < b < c` a parse error and what makes turbofish (§4.13) a spelling rather than an ambiguity. Levels 7–9 binding tighter than 10 fixes C's `a & b == c` bug, though §3.3 rule 4 still requires the parens.
