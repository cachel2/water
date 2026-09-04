use crate::Span;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DiagId {
    /// Unexpected token.
    E0001,
}
impl DiagId {
    pub fn as_str(self) -> &'static str {
        match self {
            DiagId::E0001 => "E0001",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Diagnostic {
    pub id: DiagId,
    pub span: Span,
    pub message: String,
}

#[derive(Debug)]
pub struct Bag {
    diagnostics: Vec<Diagnostic>,
}

impl Default for Bag {
    fn default() -> Self {
        Self::new()
    }
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            diagnostics: Vec::new(),
        }
    }
    pub fn report(&mut self, id: DiagId, span: Span, message: String) {
        self.diagnostics.push(Diagnostic { id, span, message })
    }

    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
