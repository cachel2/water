use water_diag::span::Span;
use water_lex::token::TokenKind;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Mul,
}
#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Int,
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl BinOp {
    pub fn from_token(kind: TokenKind) -> Option<(BinOp, u8)> {
        match kind {
            TokenKind::Star => Some((BinOp::Mul, 10)),
            TokenKind::Plus => Some((BinOp::Add, 9)),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            BinOp::Mul => "*",
            BinOp::Add => "+",
        }
    }
}
impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Expr {
    pub fn dump(&self, src: &str) -> String {
        match &self.kind {
            ExprKind::Int => src[self.span.start as usize..self.span.end as usize].to_string(),
            ExprKind::Binary { op, lhs, rhs } => {
                format!("({} {} {})", op, lhs.dump(src), rhs.dump(src))
            }
        }
    }
}
