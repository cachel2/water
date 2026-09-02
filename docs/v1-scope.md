# v1 scope

## What this is

The build order for `waterc` v1: what gets written first, and why.

The spec did not change. Not a line, not a production, not a rule. §20.1 forbids
it and nothing here is an exception. This document has no authority over `spec/`;
if it ever contradicts it, this document is wrong.

It also does not explain water. That is what `spec/` is for. Everything below is
about the compiler, not the language.

## The cut rule

§20.1 gives the test, and it is mechanical: if a change would alter a program's
meaning, it is the language and it is frozen. If it only alters what `std` offers
or what the tools say, it is not.

So every cut below is on the far side of that line: a backend, a CLI subcommand,
a tier of the interpreter ladder, a library function, a diagnostic's polish. None
of them changes what a `.wtr` file means. Anything that would has stayed in, no
matter what it costs, and there is one case where that cost is real (see
exhaustiveness).

## The layers

| Layer | v1 | After |
|---|---|---|
| §2 lexer | complete (done) | |
| §18 parser | full grammar, per-item recovery | |
| §15 diagnostics | id, span, message, no cascades | fix-its, `explain`, golden tests |
| §9 resolution | complete | |
| §6 types | complete | |
| §4.9 generics | complete, `Option` depends on it | |
| §7.5 exhaustiveness | real variant analysis | |
| §11 traps | complete | |
| §16 interpreter | tier 1 complete | tiers 2 and 3 |
| §17 LLVM backend | complete, hosted target | other backends, cross targets |
| §12 FFI, §4.11 C ABI | out | required for 1.0 |
| CLI | `run [--interp]`, `build`, `test`, `check`, `emit` | `fmt`, `iface`, `fetch` |
| deps (§13.2) | local paths | git, revisions, content hashes |
| std | what the four programs use | the rest |

## Out of v1

Seven cuts. All of them on the `std`-and-tooling side of §20.1.

**The debug allocator (§10.2).** §20.4 already puts it post-1.0, never before the
interpreter. Nothing to decide.

**Interpreter tiers 2 and 3 (§16.1).** `--sanitize` and `--sanitize=pages` are the
debug allocator wearing two settings, and the debug allocator is cut. Tier 1 and
tier 4, the interpreter and `--release`, are both in.

**`water fmt` and `water iface`.** `fmt` needs an AST that keeps trivia, which is a
parser design commitment made once the parser is finished, not while writing it.
`iface` is a reader for a thing nothing reads yet.

**`water fetch` and git dependencies (§13.2).** Local paths resolve the module
tree exactly as well. Hashes, revisions, and an offline cache are a package
manager, and there are no packages.

**std beyond the four programs.** Whatever `wc`, `cat`, `head` and `grep` do not
call does not get written. A standard library designed before its first caller is
guesswork.

**Diagnostic polish (§15).** Ids, spans, messages and no cascades stay, because
they are the recovery architecture. Fix-its, `water explain`, and the golden-test
harness are output formatting over a catalogue that is not finished.

**Non-hosted targets.** `kind = "freestanding"` (§13.2) and the bare-metal half of
§11.2 are a second lowering, a linker script, and a `core`-only prelude, for a
machine that is not the one being developed on. One target triple, hosted.

## Deferred, not cut

§12 (FFI) and §4.11 (the C ABI subset) are frozen language. They cannot be cut,
and this is not a claim that they can be. They are the last thing implemented.
The backend links with `cc`, so the mechanism is there; what is missing is
`extern` as a user-facing declaration, and nothing in v1 needs one.

What that costs: v1 programs have no way to declare a call into `libc`, which is
why the four builtins below exist. What it must not cost is a front end that assumed C was never
coming. Two things stay true while FFI is not implemented:

HIR and the typechecker keep the C-ABI classification of every type, even with no
`extern` to check it against. §4.11 decides which types cross the boundary, and
that answer is a property of the type, not of the call. Compute it, test it, do
not call it.

`extern` parses (see the parser principle) and is rejected in HIR by name. The
rejection is one diagnostic and one match arm, and deleting it is the whole
implementation of §12's front end.

## In, no matter what

These are not features. They are shapes the compiler takes on day one and cannot
take later.

**Per-item recovery with no cascades (§15).** One real error per broken thing.
Retrofitting this means rewriting the parser's error paths and the typechecker's
error type at the same time. It is a decision about what an error *is*, made
before the first one is emitted.

**A span on every node.** Cheap while writing the node, expensive on every node
that already exists. §15's format is unimplementable without it.

**Generics with monomorphization at HIR→MIR (§17).** `Option<T>` is in the
prelude and `?` desugars through it, so there is no subset of water that runs
without generics. Monomorphization also fixes what MIR is: a typed CFG with no
polymorphism in it.

**Real exhaustiveness analysis (§7.5 rule 1).** The alternative, demanding `_` on
every `match`, is not a schedule decision. §7.5 rule 1 says a missing variant is
an error naming the missing constructors and their declaration sites. Requiring
`_` deletes that error, which changes what a program means, which §20.1 forbids.
It is also the exact diagnostic §15 uses to illustrate itself. It costs a week or
two and it is not optional.

**Every trap in §11.** Traps are semantics, hosted and bare-metal alike. An
arithmetic overflow that does not trap is a different language.

**Interpreter tier 1, complete (§16).** Both UBs, leaks, uninitialized reads,
misalignment, raw-pointer OOB, with full history. It is the sanitizer, the fast
loop, and the constant evaluator. Cut any of it and the pitch in the README stops
being true.

**IR-1 and IR-2 (§17).** The backend ships in v1, so its two obligations do too.
No TBAA metadata, or §4.3's absence of a strict-aliasing rule is a lie at `-O2`.
`freeze` on loads from stack slots definite-init could not prove written, or
§6.4's "unspecified, not undefined" is a lie. Both are semantics wearing a
backend's clothes, and both are cheap on the way in and archaeology later.

## The parser principle

The parser implements §18 in full. All of it, including productions for features
deferred to after v1.

Deferred features are rejected in HIR, by a named diagnostic, not by a hole in the
grammar. Three reasons. The grammar stays honest, which matters because §20.2
makes §18 the authority that settles contradictions. The parser stays usable as a
reference implementation of the frozen grammar. And enabling the feature later is
deleting a rejection, not editing a parser.

## The four builtins

`open`, `read`, `write`, `close`. Compiler builtins, implemented natively by the
interpreter, lowered to calls by the backend when there is one.

They exist because §12 is deferred and a program that cannot call `libc` cannot
touch a file. `cat` with no `open` is not a test of anything.

They are not the FFI. §12 is a user-facing feature: you declare `extern "c" fn`
and the compiler checks your call site. These four are the compiler's own door to
the system, in the same position as `io::print` in §8.6, which is already
special-cased for exactly this reason: it needs structural knowledge the language
does not expose. Same category, same non-extensibility, same "you cannot write
another."

One thing to settle before writing them: §8.6 says *five* functions are
special-cased. Four more is either an erratum under §20.2 or these live in `std`
as ordinary functions the driver links to an interpreter intrinsic. The second
reading needs no erratum and is preferred.

## Done

Four programs, written in water: `wc`, `cat`, `head`, `grep`. Each takes a
filename on the command line and does the obvious thing.

v1 is done when all four run under `water run --interp` and compile through LLVM
and run native, producing the same bytes both ways, with the interpreter
reporting no leaks and no UB at exit for every one of them. That is point 3 of
§20.4, and it is why the backend is the one thing the cut does not touch: a
language that has never emitted an object file has not been tested, it has been
described.

The gate is not an opinion. The programs either produce the right bytes or they
do not, and the interpreter either reports a live allocation at exit or it does
not.

## The line budget

The lexer is 130 lines of code and 120 of tests. Scaling that shape across the
rest gives roughly 16k lines for the whole compiler, about 8k of code and 8k of
tests:

| Crate | LOC with tests |
|---|---|
| `water_lex` | 250 (done) |
| `water_diag` | 600 |
| `water_parse` | 2500 |
| `water_hir` | 2000 |
| `water_types` | 4000 |
| `water_mir` | 2000 |
| interpreter | 2000 |
| `water_llvm` | 2000 (§17: a backend is a self-contained ~2k job) |
| driver + minimal std | 500 |
| **total** | **15850** |

This is not a prediction. It is an alarm. A crate that goes well past its number
is reporting that something bent: a feature crept in from the far side of the
freeze, an abstraction got built before it had two callers, or the layer is doing
work that belongs to the layer below it. Check which, then decide. The number
being wrong is fine. Passing it without noticing is not.
