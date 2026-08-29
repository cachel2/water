/// The offsets are not global to the file, they are global to the project.
/// Each file occupies a contiguous range of offset space.
/// Span must not contain FileId to keep it 8-bytes,
/// the reason is simple, is the most copied struct of waterc.
///
/// Who will resolve it?
/// A SourceMap that does not exist yet, it must translate
/// the offsets to file, line and column with binary search in ranges.
/// Furthermore, move the approximation of the sum of all sources
/// to the source map and remove it from `Lexer::new()`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }
}
