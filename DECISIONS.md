# Decisions — water

Why it was done this way, and what was ruled out. The code says what;
this says why. Written with `why "..."`.

## 2026-09-01

- `pos: u32` in the lexer, `usize` in the parser: the type is dictated by what it has to match. In the lexer it is a source offset and travels with `Span`; in the parser it is an index into `Vec<Token>`
  <sub>`c8c80b9` · main · /</sub>
- `Box<Expr>` in the AST instead of an arena with indices: the AST is short-lived, walked once or twice and lowered to HIR (§17). Arenas are saved for HIR and MIR, where repeated queries and stable ids pay for them
  <sub>`c8c80b9` · main · /</sub>
- `Span` in `water_diag` instead of `water_lex`: the whole compiler emits diagnostics and much of it never saw a token. Keeps the graph diag <- lex <- parse, a DAG with no cycles
  <sub>`c8c80b9` · main · /</sub>
- diagnostic id as an enum instead of `&static str`: the compiler then guarantees uniqueness and exhaustiveness. §15 requires stable ids, and with strings that stability is a hand-maintained convention with typos nothing checks
  <sub>`c8c80b9` · main · /</sub>
- `ExprKind::Error` instead of `expr` returning `Option<Expr>`: a tree that cannot hold a hole spreads the hole across every caller. One real error per broken thing (§15) requires parsing the siblings of a broken node
  <sub>`c8c80b9` · main · /</sub>
- `expect` does not consume the offending token: eating a `}` costs a whole item of recovery. Which tokens are anchors is knowledge that lives in the recovery function, not in the primitive. Standing obligation: if `expect` does not advance, something else must
  <sub>`c8c80b9` · main · /</sub>
