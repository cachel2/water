use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Lexer<'src> {
    src: &'src str,
    bytes: &'src [u8],
    pos: u32,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Lexer<'src> {
        assert!(src.len() <= u32::MAX as usize, "source too large");
        Lexer {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
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
                    self.take_while(is_word);
                    let mut is_float = false;
                    if matches!(self.rest(), [b'.', b'0'..=b'9', ..]) {
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
                    let is_doc = self.rest().starts_with(b"///");
                    let span = self.take_while(|b| b != b'\n');
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
        let mut lexer = Lexer::new(src);
        let mut out = Vec::new();
        loop {
            let kind = lexer.next_token().kind;
            out.push(kind);
            if kind == Eof {
                break;
            }
        }
        out
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
    fn double_quote_n_simple() {
        assert_eq!(kinds(r#"'a'"#), vec![Byte, Eof]);
    }
    #[test]
    fn double_quote_w_scape() {
        assert_eq!(kinds(r#""a\"b""#), vec![Str, Eof]);
    }
    #[test]
    fn no_closing_quote() {
        assert_eq!(kinds(r#""I forgot"#), vec![Error, Eof]);
    }
}
