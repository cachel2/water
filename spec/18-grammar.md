# 18 · The complete grammar

Normative. Any production not here does not exist. `(* *)` are comments.

```ebnf
(* ---------- file ---------- *)
file   = { item } ;
item   = [ "pub" ] ( fn_item | struct_item | enum_item | impl_item
                   | const_item | static_item | use_item | test_item | extern_item ) ;

fn_item     = "fn" IDENT [ generics ] "(" [ params ] ")" [ "->" type ] block ;
params      = param { "," param } [ "," ] ;
param       = ( "self" | [ "mut" ] IDENT ":" type ) ;   (* bare self = self: *const Self *)
generics    = "<" IDENT { "," IDENT } ">" ;             (* no bounds *)

struct_item = [ "packed" ] "struct" IDENT [ generics ] "{" { field } "}" ;
field       = IDENT ":" type [ "=" expr ] "," ;

enum_item   = "enum" IDENT [ generics ] [ ":" type ] "{" [ variant { "," variant } [ "," ] ] "}" ;
variant     = IDENT ( "(" [ pfields ] ")" | "{" { field } "}" | [ "=" expr ] ) ;
pfields     = pfield { "," pfield } [ "," ] ;
pfield      = IDENT ":" type ;

impl_item   = "impl" IDENT [ generics ] "{" { fn_item } "}" ;

const_item  = "const"  IDENT ":" type "=" expr ";" ;
static_item = "static" [ "mut" ] IDENT ":" type "=" expr ";" ;
use_item    = "use" path [ "::" "{" IDENT { "," IDENT } [ "," ] "}" ] ";" ;
test_item   = "test" STRING block ;
extern_item = "extern" STRING "fn" IDENT "(" [ params ] ")" [ "->" type ] ";" ;

(* ---------- types ---------- *)
type = "*" [ "const" ] type
     | "[" "]" [ "const" ] type
     | "[" expr "]" type
     | "fn" "(" [ type { "," type } ] ")" [ "->" type ]
     | "(" ")"                              (* unit *)
     | "Self"
     | path [ "<" type { "," type } ">" ] ;

(* ---------- statements & blocks ---------- *)
block = "{" { stmt } [ expr ] "}" ;         (* trailing expr (no ;) = block value *)
stmt  = let_stmt | assign_stmt | expr ";" | return_stmt
      | if_stmt | while_stmt | for_stmt | defer_stmt
      | "break" ";" | "continue" ";" ;

let_stmt    = "let" [ "mut" ] IDENT [ ":" type ] "=" ( expr | "undefined" ) ";" ;
assign_stmt = place assign_op expr ";" ;
assign_op   = "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" ;
return_stmt = "return" [ expr ] ";" ;
defer_stmt  = "defer" call_expr ";" ;    (* call only, §7.6 rule 1 *)

if_stmt    = "if" "(" expr ")" block [ "else" ( if_stmt | block ) ] ;
while_stmt = "while" "(" expr ")" block ;
for_stmt   = "for" "(" ( c_for | in_for ) ")" block ;
c_for     = [ for_init ] ";" [ expr ] ";" [ place assign_op expr ] ;
for_init  = IDENT "=" expr | place assign_op expr ;
in_for    = [ "*" ] IDENT "in" expr ;

(* ---------- expressions ---------- *)
(* Pratt over the table in §8.1; primary/postfix below. if & match are expressions. *)
expr    = if_expr | match_expr | pratt ;
if_expr = "if" "(" expr ")" block "else" ( if_expr | block ) ;   (* value form needs else *)
match_expr = "match" expr "{" arm { "," arm } [ "," ] "}" ;
arm     = pattern [ "if" expr ] "=>" ( expr | block | "return" [ expr ] | "break" | "continue" ) ;

primary = literal
        | "(" [ expr ] ")"                 (* "()" is unit; "(e)" is grouping *)
        | array_lit
        | struct_lit
        | path [ "::" "<" type { "," type } ">" ] ;   (* turbofish *)

postfix = primary { "." IDENT [ "(" [ args ] ")" ]     (* field, or method call *)
                  | "[" index "]"
                  | "(" [ args ] ")"                    (* call *)
                  | "?" } ;
args    = expr { "," expr } [ "," ] ;
call_expr = postfix ;    (* must resolve to a call or method call; §7.6 rule 1 *)
index   = expr | range ;
range   = [ expr ] ( ".." | "..=" ) [ expr ] ;

struct_lit = path "{" [ finit { "," finit } [ "," ] ] "}" ;
finit      = IDENT ":" expr ;
array_lit  = "[" [ expr { "," expr } [ "," ] ] "]" ;

pattern = "_" | literal
        | literal ( ".." | "..=" ) literal
        | path "(" [ pattern { "," pattern } ] ")"
        | path "{" [ IDENT { "," IDENT } ] "}"
        | path | IDENT ;                    (* variant if the subject type has it, else binding *)

literal = INT | FLOAT | BYTE | STRING | "true" | "false" ;
path    = IDENT { "::" IDENT } ;
```

**Grammar properties, normative:** LL(1) by recursive descent for items, statements, types, and patterns; Pratt for expressions; one token of lookahead; no backtracking; no symbol-table feedback; no lexer hack. Any change that breaks this breaks the freeze.
