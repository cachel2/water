//! The vocabulary of water (§2).
use crate::token::{Token, TokenKind};
use water_diag::span::Span;

pub struct Lexer<'src> {
    src: &'src str,
    bytes: &'src [u8],
    pos: u32,
}

impl<'src> Lexer<'src> {
    /// This might move to the SourceMap asap it exists,
    /// this is just an aprox, the `u32::MAX` must apply to
    /// the sum of all source.
    pub fn new(src: &'src str) -> Lexer<'src> {
        assert!(src.len() <= u32::MAX as usize, "source too large");
        Lexer {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut out = Vec::with_capacity(self.src.len() / 4);
        loop {
            let token = self.next_token();
            out.push(token);
            if token.kind == TokenKind::Eof {
                break;
            }
        }
        out
    }
    fn rest(&self) -> &[u8] {
        &self.bytes[self.pos as usize..]
    }
    fn string_like(&mut self, quote: u8, kind: TokenKind) -> Token {
        let start = self.pos;
        self.pos += 1;
        loop {
            match self.rest() {
                [] => {
                    return Token::new(TokenKind::Error, Span::new(start, self.pos));
                }
                [b'\\', _, ..] => {
                    self.pos += 2;
                }
                [_, ..] => {
                    let b = self.rest()[0];
                    self.pos += 1;
                    if b == quote {
                        return Token::new(kind, Span::new(start, self.pos));
                    }
                }
            }
        }
    }
    fn take_while(&mut self, f: impl Fn(u8) -> bool) -> Span {
        let start = self.pos;
        while self.rest().first().is_some_and(|&byte| f(byte)) {
            self.pos += 1;
        }
        Span::new(start, self.pos)
    }
    pub fn next_token(&mut self) -> Token {
        loop {
            match self.rest() {
                [] => return Token::new(TokenKind::Eof, Span::new(self.pos, self.pos)),
                [b' ' | b'\t' | b'\n' | b'\r', ..] => {
                    self.take_while(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r'));
                    continue;
                }
                [b'a'..=b'z' | b'A'..=b'Z' | b'_', ..] => {
                    let span = self.take_while(is_word);
                    let text = &self.src[span.start as usize..span.end as usize];
                    return Token::new(TokenKind::from_word(text), span);
                }
                [b'0'..=b'9', ..] => {
                    let start = self.pos;
                    let is_hex = matches!(self.rest(), [b'0', b'x' | b'X', ..]);
                    self.take_while(is_word);
                    let mut is_float = false;
                    if matches!(self.rest(), [b'.', b'0'..=b'9', ..]) {
                        self.pos += 1;
                        self.take_while(is_word);
                        is_float = true;
                    }
                    if !is_hex
                        && matches!(self.bytes[self.pos as usize - 1], b'e' | b'E')
                        && matches!(self.rest(), [b'+' | b'-', b'0'..=b'9', ..])
                    {
                        self.pos += 1;
                        self.take_while(is_word);
                        is_float = true;
                    }
                    return Token::new(
                        if is_float {
                            TokenKind::Float
                        } else {
                            TokenKind::Int
                        },
                        Span::new(start, self.pos),
                    );
                }
                [b'"', ..] => return self.string_like(b'"', TokenKind::Str),
                [b'\'', ..] => return self.string_like(b'\'', TokenKind::Byte),
                [b'/', b'/', ..] => {
                    let is_doc =
                        self.rest().starts_with(b"///") && !self.rest().starts_with(b"////");
                    let span = self.take_while(|b| b != b'\n' && b != b'\r');
                    if is_doc {
                        return Token::new(TokenKind::DocComment, span);
                    }
                    continue;
                }
                [_, ..] => {
                    let start = self.pos;
                    if let Some((kind, len)) = TokenKind::from_punct(self.rest()) {
                        self.pos += len;
                        return Token::new(kind, Span::new(start, self.pos));
                    }
                    self.pos += 1;
                    return Token::new(TokenKind::Error, Span::new(start, self.pos));
                }
            }
        }
    }
}
fn is_word(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        Lexer::new(src).tokenize().iter().map(|t| t.kind).collect()
    }

    #[test]
    fn empty_input() {
        assert_eq!(kinds(""), vec![Eof]);
    }

    #[test]
    fn only_whitespace() {
        assert_eq!(kinds("   "), vec![Eof]);
    }

    #[test]
    fn simple_let() {
        assert_eq!(kinds("let x"), vec![Let, Ident, Eof]);
    }

    #[test]
    fn symbols() {
        assert_eq!(kinds("+ - ( )"), vec![Plus, Minus, LParen, RParen, Eof]);
    }

    #[test]
    fn error() {
        assert_eq!(kinds("@"), vec![Error, Eof]);
    }

    #[test]
    fn spans() {
        let mut lexer = Lexer::new("let x");
        let mut out = Vec::new();

        loop {
            let token = lexer.next_token();
            out.push(token.span);

            if token.kind == Eof {
                break;
            }
        }

        assert_eq!(
            out,
            vec![Span::new(0, 3), Span::new(4, 5), Span::new(5, 5),]
        );
    }
    #[test]
    fn comment() {
        assert_eq!(kinds("// hola"), vec![Eof]);
    }
    #[test]
    fn doc_comment() {
        assert_eq!(kinds("/// hola"), vec![DocComment, Eof]);
    }

    #[test]
    fn middle_comment() {
        assert_eq!(kinds("a // b\nc"), vec![Ident, Ident, Eof]);
    }

    #[test]
    fn integer() {
        assert_eq!(kinds("42"), vec![Int, Eof]);
    }

    #[test]
    fn float() {
        assert_eq!(kinds("3.14"), vec![Float, Eof]);
    }

    #[test]
    fn hex() {
        assert_eq!(kinds("0xFF"), vec![Int, Eof]);
    }

    #[test]
    fn underscores() {
        assert_eq!(kinds("1_000"), vec![Int, Eof]);
    }

    #[test]
    fn range_not_float() {
        assert_eq!(kinds("1..10"), vec![Int, DotDot, Int, Eof]);
    }

    #[test]
    fn trailing_dot() {
        assert_eq!(kinds("1."), vec![Int, Dot, Eof]);
    }

    #[test]
    fn double_quote() {
        assert_eq!(kinds(r#""hola""#), vec![Str, Eof]);
    }

    #[test]
    fn byte_literal() {
        assert_eq!(kinds(r#"'a'"#), vec![Byte, Eof]);
    }
    #[test]
    fn escaped_quote() {
        assert_eq!(kinds(r#""a\"b""#), vec![Str, Eof]);
    }
    #[test]
    fn no_closing_quote() {
        assert_eq!(kinds(r#""I forgot"#), vec![Error, Eof]);
    }
    #[test]
    fn exponent_plus() {
        assert_eq!(kinds("1e+9"), vec![Float, Eof]);
    }
    #[test]
    fn exponent_minus() {
        assert_eq!(kinds("1e-9"), vec![Float, Eof]);
    }
    #[test]
    fn exponent_after_point() {
        assert_eq!(kinds("1.5e10"), vec![Float, Eof]);
    }
    #[test]
    fn hex_e_is_not_exponent() {
        assert_eq!(kinds("0xe-1"), vec![Int, Minus, Int, Eof]);
    }
    #[test]
    fn exponent_needs_digits() {
        assert_eq!(kinds("1e-"), vec![Int, Minus, Eof]);
    }
    #[test]
    fn empty_comment() {
        assert_eq!(kinds("//"), vec![Eof]);
    }
    #[test]
    fn empty_doc_comment() {
        assert_eq!(kinds("///"), vec![DocComment, Eof]);
    }
    #[test]
    fn four_slashes_is_not_doc() {
        assert_eq!(kinds("////"), vec![Eof]);
    }
    #[test]
    fn crlf_line_ending() {
        assert_eq!(kinds("/// hola\r\nx"), vec![DocComment, Ident, Eof]);
    }
}
