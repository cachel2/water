//! The vocabulary of water (§2).
//!
//! A token is a kind plus a span; the text lives in the source, never copied.
//! The two tables that turn source bytes into a kind — words and punctuation —
//! live here too, so the lexer only has to decide *which* table to consult.

use water_diag::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    /// The source text this token covers: identifiers, literals, doc comments.
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.span.start as usize..self.span.end as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords (§2, the complete set of 23)
    Let,
    Mut,
    Const,
    Fn,
    Struct,
    Enum,
    Impl,
    Use,
    Pub,
    Extern,
    Packed,
    If,
    Else,
    While,
    For,
    In,
    Match,
    Break,
    Continue,
    Return,
    Defer,
    Test,
    As,

    /// Contextual words (`self`, `Self`, `true`, `false`, `undefined`, the
    /// primitive type names, `Option`/`Result`/`Some`/`None`/`Ok`/`Err`) lex as
    /// `Ident`; they are root-scope names, not keywords.
    Ident,

    /// One of the reserved words. Lexed so the parser can report a compile
    /// error naming why the word does not exist (§2).
    Reserved,

    // Literals. The suffix, base and escapes are the parser's problem; the
    // lexer only marks where the literal starts and ends.
    Int,
    Float,
    Byte,
    Str,

    // Arithmetic (trap on overflow)
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Wrapping
    PlusPercent,
    MinusPercent,
    StarPercent,

    // Saturating
    PlusPipe,
    MinusPipe,
    StarPipe,

    // Comparison (non-associative)
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    // Logical
    AmpAmp,
    PipePipe,
    Bang,

    // Bitwise
    Amp,
    Pipe,
    Caret,
    Tilde,
    LtLt,
    GtGt,

    // Assignment
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpEq,
    PipeEq,
    CaretEq,
    LtLtEq,
    GtGtEq,

    // Other operators
    Dot,
    Question,
    DotDot,
    DotDotEq,

    // Punctuation
    ColonColon,
    Comma,
    Semi,
    Colon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    FatArrow,
    /// `_` alone. `_foo` is an `Ident`: read the whole word, and only the exact
    /// string `_` becomes `Underscore`.
    Underscore,

    /// Only in function signatures and function-pointer types.
    Arrow,

    /// `///`. `//` line comments are skipped and never reach the parser.
    DocComment,

    /// A character that begins no valid token. The parser turns this into a
    /// diagnostic; the lexer never panics on bad input.
    Error,

    Eof,
}

impl TokenKind {
    /// The kind of a completed word: `[A-Za-z_][A-Za-z0-9_]*`.
    ///
    /// One match over the whole word, so keywords, reserved words, `_` and
    /// identifiers are all decided in a single lookup. rustc compiles this to a
    /// switch on length then bytes: no hashing, no allocation, no table.
    pub fn from_word(word: &str) -> TokenKind {
        use TokenKind::*;
        match word {
            "let" => Let,
            "mut" => Mut,
            "const" => Const,
            "fn" => Fn,
            "struct" => Struct,
            "enum" => Enum,
            "impl" => Impl,
            "use" => Use,
            "pub" => Pub,
            "extern" => Extern,
            "packed" => Packed,
            "if" => If,
            "else" => Else,
            "while" => While,
            "for" => For,
            "in" => In,
            "match" => Match,
            "break" => Break,
            "continue" => Continue,
            "return" => Return,
            "defer" => Defer,
            "test" => Test,
            "as" => As,

            "var" | "void" | "switch" | "union" | "typedef" | "static" | "inline" | "volatile"
            | "goto" | "register" | "auto" | "long" | "short" | "signed" | "unsigned" | "float"
            | "double" | "int" | "char" | "sizeof" | "null" | "NULL" | "nil" | "trait" | "dyn"
            | "async" | "await" | "unsafe" | "move" | "ref" | "where" | "type" | "mod" => Reserved,

            "_" => Underscore,
            _ => Ident,
        }
    }

    /// Maximal munch over punctuation: the longest operator that starts at
    /// `src[0]`, plus its length in bytes. `None` if no operator starts here.
    ///
    /// `src` must be non-empty and must not start with `//` — the lexer strips
    /// comments before asking. `>>` is lexed as one token; the parser splits it
    /// when it closes nested generics.
    pub fn from_punct(src: &[u8]) -> Option<(TokenKind, u32)> {
        use TokenKind::*;
        let next = src.get(1).copied();
        let third = src.get(2).copied();

        let hit = match *src.first()? {
            b'+' => match next {
                Some(b'=') => (PlusEq, 2),
                Some(b'%') => (PlusPercent, 2),
                Some(b'|') => (PlusPipe, 2),
                _ => (Plus, 1),
            },
            b'-' => match next {
                Some(b'=') => (MinusEq, 2),
                Some(b'%') => (MinusPercent, 2),
                Some(b'|') => (MinusPipe, 2),
                Some(b'>') => (Arrow, 2),
                _ => (Minus, 1),
            },
            b'*' => match next {
                Some(b'=') => (StarEq, 2),
                Some(b'%') => (StarPercent, 2),
                Some(b'|') => (StarPipe, 2),
                _ => (Star, 1),
            },
            b'/' => match next {
                Some(b'=') => (SlashEq, 2),
                _ => (Slash, 1),
            },
            b'%' => match next {
                Some(b'=') => (PercentEq, 2),
                _ => (Percent, 1),
            },
            b'=' => match next {
                Some(b'=') => (EqEq, 2),
                Some(b'>') => (FatArrow, 2),
                _ => (Eq, 1),
            },
            b'!' => match next {
                Some(b'=') => (BangEq, 2),
                _ => (Bang, 1),
            },
            b'<' => match (next, third) {
                (Some(b'<'), Some(b'=')) => (LtLtEq, 3),
                (Some(b'<'), _) => (LtLt, 2),
                (Some(b'='), _) => (LtEq, 2),
                _ => (Lt, 1),
            },
            b'>' => match (next, third) {
                (Some(b'>'), Some(b'=')) => (GtGtEq, 3),
                (Some(b'>'), _) => (GtGt, 2),
                (Some(b'='), _) => (GtEq, 2),
                _ => (Gt, 1),
            },
            b'&' => match next {
                Some(b'&') => (AmpAmp, 2),
                Some(b'=') => (AmpEq, 2),
                _ => (Amp, 1),
            },
            b'|' => match next {
                Some(b'|') => (PipePipe, 2),
                Some(b'=') => (PipeEq, 2),
                _ => (Pipe, 1),
            },
            b'^' => match next {
                Some(b'=') => (CaretEq, 2),
                _ => (Caret, 1),
            },
            b'.' => match (next, third) {
                (Some(b'.'), Some(b'=')) => (DotDotEq, 3),
                (Some(b'.'), _) => (DotDot, 2),
                _ => (Dot, 1),
            },
            b':' => match next {
                Some(b':') => (ColonColon, 2),
                _ => (Colon, 1),
            },
            b'~' => (Tilde, 1),
            b'?' => (Question, 1),
            b',' => (Comma, 1),
            b';' => (Semi, 1),
            b'(' => (LParen, 1),
            b')' => (RParen, 1),
            b'{' => (LBrace, 1),
            b'}' => (RBrace, 1),
            b'[' => (LBracket, 1),
            b']' => (RBracket, 1),
            _ => return None,
        };
        Some(hit)
    }

    /// How this kind is named in a diagnostic. Fixed tokens print themselves;
    /// the rest print their category, since their text is in the source.
    pub fn as_str(self) -> &'static str {
        use TokenKind::*;
        match self {
            Let => "let",
            Mut => "mut",
            Const => "const",
            Fn => "fn",
            Struct => "struct",
            Enum => "enum",
            Impl => "impl",
            Use => "use",
            Pub => "pub",
            Extern => "extern",
            Packed => "packed",
            If => "if",
            Else => "else",
            While => "while",
            For => "for",
            In => "in",
            Match => "match",
            Break => "break",
            Continue => "continue",
            Return => "return",
            Defer => "defer",
            Test => "test",
            As => "as",

            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            PlusPercent => "+%",
            MinusPercent => "-%",
            StarPercent => "*%",
            PlusPipe => "+|",
            MinusPipe => "-|",
            StarPipe => "*|",
            EqEq => "==",
            BangEq => "!=",
            Lt => "<",
            LtEq => "<=",
            Gt => ">",
            GtEq => ">=",
            AmpAmp => "&&",
            PipePipe => "||",
            Bang => "!",
            Amp => "&",
            Pipe => "|",
            Caret => "^",
            Tilde => "~",
            LtLt => "<<",
            GtGt => ">>",
            Eq => "=",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            AmpEq => "&=",
            PipeEq => "|=",
            CaretEq => "^=",
            LtLtEq => "<<=",
            GtGtEq => ">>=",
            Dot => ".",
            Question => "?",
            DotDot => "..",
            DotDotEq => "..=",
            ColonColon => "::",
            Comma => ",",
            Semi => ";",
            Colon => ":",
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            LBracket => "[",
            RBracket => "]",
            FatArrow => "=>",
            Underscore => "_",
            Arrow => "->",

            Ident => "identifier",
            Reserved => "reserved word",
            Int => "integer literal",
            Float => "float literal",
            Byte => "byte literal",
            Str => "string literal",
            DocComment => "doc comment",
            Error => "unknown character",
            Eof => "end of file",
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::TokenKind::{self, *};

    #[test]
    fn words() {
        assert_eq!(TokenKind::from_word("let"), Let);
        assert_eq!(TokenKind::from_word("trait"), Reserved);
        assert_eq!(TokenKind::from_word("_"), Underscore);
        assert_eq!(TokenKind::from_word("_foo"), Ident);
        assert_eq!(TokenKind::from_word("i32"), Ident);
        assert_eq!(TokenKind::from_word("letter"), Ident);
    }

    #[test]
    fn maximal_munch() {
        let punct = |s: &str| TokenKind::from_punct(s.as_bytes()).unwrap();
        assert_eq!(punct("<<=x"), (LtLtEq, 3));
        assert_eq!(punct("<<x"), (LtLt, 2));
        assert_eq!(punct("<x"), (Lt, 1));
        assert_eq!(punct("..=x"), (DotDotEq, 3));
        assert_eq!(punct("+%x"), (PlusPercent, 2));
        assert_eq!(punct("+|x"), (PlusPipe, 2));
        assert_eq!(punct("->x"), (Arrow, 2));
        assert_eq!(punct("=>x"), (FatArrow, 2));
        assert_eq!(punct("::x"), (ColonColon, 2));
        assert_eq!(TokenKind::from_punct(b"@"), None);
        assert_eq!(TokenKind::from_punct(b""), None);
    }

    #[test]
    fn munch_at_end_of_input() {
        assert_eq!(TokenKind::from_punct(b"<"), Some((Lt, 1)));
        assert_eq!(TokenKind::from_punct(b"<<"), Some((LtLt, 2)));
        assert_eq!(TokenKind::from_punct(b".."), Some((DotDot, 2)));
    }
}
