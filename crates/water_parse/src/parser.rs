use water_diag::span::Span;
use water_lex::{
    lex::Lexer,
    token::{Token, TokenKind},
};

use crate::ast::{BinOp, Expr, ExprKind};

/// Important distinction, `Vec<Token>` always contains the
/// Eof inside and pos is never greater than the index of Eof,
/// That's why we can index.
pub struct Parser<'src> {
    tokens: Vec<Token>,
    src: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        let tokens = Lexer::new(src).tokenize();
        Parser {
            tokens,
            src,
            pos: 0,
        }
    }
    fn bump(&mut self) -> Token {
        let current_token = self.tokens[self.pos];
        if current_token.kind != TokenKind::Eof {
            self.pos += 1;
        }
        current_token
    }
    fn peek(&self) -> TokenKind {
        self.tokens[self.pos].kind
    }

    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek() == kind {
            Some(self.bump())
        } else {
            None
        }
    }

    fn expr(&mut self, min_bp: u8) -> Expr {
        let token = self.bump();
        let mut left = Expr {
            kind: ExprKind::Int,
            span: token.span,
        };
        loop {
            let Some((op, bp)) = BinOp::from_token(self.peek()) else {
                break;
            };
            if bp <= min_bp {
                break;
            }
            self.bump();
            let right = self.expr(bp);
            let span = Span::new(left.span.start, right.span.end);
            left = Expr {
                kind: ExprKind::Binary {
                    op,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                },
                span,
            }
        }
        left
    }

    fn expect() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use water_diag::span::Span;

    use super::*;
    use TokenKind::*;

    fn dump(src: &str) -> String {
        Parser::new(src).expr(0).dump(src)
    }

    #[test]
    fn peek_at_eof() {
        let parser = Parser::new("");
        assert_eq!((parser.peek()), Eof);
    }

    #[test]
    fn bump_clamps_at_eof() {
        let mut parser = Parser::new("");
        for _ in 0..10 {
            assert_eq!((parser.bump()), Token::new(Eof, Span::new(0, 0)));
        }
    }

    #[test]
    fn bump_advances() {
        let mut parser = Parser::new("let x");

        assert_eq!((parser.bump()), Token::new(Let, Span::new(0, 3)));
        assert_eq!((parser.bump()), Token::new(Ident, Span::new(4, 5)));
        assert_eq!((parser.bump()), Token::new(Eof, Span::new(5, 5)));
    }

    #[test]
    fn eat_does_not_advance_on_mismatch() {
        let mut parser = Parser::new("let x");
        assert_eq!(parser.eat(Ident), None);
        assert_eq!(parser.pos, 0);
    }

    #[test]
    fn int_atom() {
        assert_eq!(dump("42"), "42");
    }

    #[test]
    fn single_binary() {
        assert_eq!(dump("1 + 2"), "(+ 1 2)");
    }
    #[test]
    fn mul_binds_tighter_than_add() {
        assert_eq!(dump("1 + 2 * 3"), "(+ 1 (* 2 3))");
    }
    #[test]
    fn mul_binds_tighter_on_the_left() {
        assert_eq!(dump("1 * 2 + 3"), "(+ (* 1 2) 3)");
    }
    #[test]
    fn add_is_left_associative() {
        assert_eq!(dump("1 + 2 + 3"), "(+ (+ 1 2) 3)");
    }
}
