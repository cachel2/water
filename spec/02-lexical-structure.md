# 2 · Lexical structure

**Keywords (23) — the complete set:**

```
let    mut    const  fn     struct enum   impl   use
pub    extern packed if     else   while  for    in
match  break  continue      return defer  test   as
```

**Contextual words (predefined in the root scope, not shadowable):** `self Self true false undefined i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64 bool str Option Result Some None Ok Err`

**Reserved — a compile error naming why:** `var void switch union typedef static inline volatile goto register auto long short signed unsigned float double int char sizeof null NULL nil trait dyn async await unsafe move ref where type mod`

**Identifiers:** `[A-Za-z_][A-Za-z0-9_]*`. Case-sensitive. No length limit. ASCII only; no Unicode identifiers.

**Integer literals:** `42`, `0xFF`, `0o17`, `0b1010`, `1_000_000`. `_` may separate digits but not lead or trail. Untyped until bound (§6.3). A literal that does not fit its inferred type is a compile error.

**Float literals:** `3.14`, `1e-9`, `1.5e10`. A digit is required on both sides of the point: `1.` and `.5` are errors.

**Suffixed literals:** `42i64`, `3.0f32`. The suffix is the type; no inference is performed.

**Byte literals:** `'a'`, `'\n'` — type `u8`. ASCII only (0x00–0x7F); a multi-byte character in a byte literal is a compile error naming `str`.

**String literals:** `"hello"` — type `str`. Adjacent literals do not concatenate.

**Escapes (the complete set):** `\n \t \r \\ \" \' \0 \xNN`. Any other backslash sequence is an error.

**Operators — the complete set:**

```
+  -  *  /  %          arithmetic (trap on overflow)
+% -% *%               wrapping
+| -| *|               saturating
== != <  <= >  >=      comparison (non-associative)
&& || !                logical
&  |  ^  ~  << >>      bitwise
=  += -= *= /= %= &= |= ^= <<= >>=
&  *  .  ?  ..  ..=  as
::  ,  ;  :  (  )  {  }  [  ]  <  >  =>  _
```

`->` appears only in function signatures and function-pointer types; it is not a dereference operator (that is `*`) and not field access (that is `.`). `++` and `--` do not exist. `,` is a separator, not an operator. The ternary `?:` does not exist (`if` and `match` are expressions).

**Comments:** `//` line, `///` doc (attaches to the following item; a `///` not followed by an item is an error). No block comments.

**Whitespace** is insignificant except as a token separator. There is no automatic semicolon insertion — but the semicolon carries meaning (§7.1), so its absence is never guessed at, only obeyed.
