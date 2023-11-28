#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub start: T,
    pub end: T
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub row: usize,
    pub col: usize,
    pub len: usize,

    pub range: Range<usize>
}

#[derive(Debug, Clone)]
pub enum Token {
    Paren   { span: Span, close: bool },
    Bracket { span: Span, close: bool },
    Brace   { span: Span, close: bool },

    Pound(Span),
    Comma(Span),
    Colon(Span),
    SemiColon(Span),
    Dot(Span),

    Int(Span),
    Float(Span),
    Identifier(Span),
    String(Span),
    Comment(Span),

    Operator(Span),
}
