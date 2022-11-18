use std::fmt;
use std::ops::Range;

use chumsky::Stream;

#[derive(Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "start must be less than or equal to end");
        Self { start, end }
    }

    pub fn range(self) -> Range<usize> {
        self.start..self.end
    }

    pub fn join(self, other: Self) -> Self {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self::new(start, end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.range().fmt(f)
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.range()
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

impl<T> HasSpan for (T, Span) {
    fn span(&self) -> Span {
        self.1
    }
}

impl chumsky::Span for Span {
    type Context = ();
    type Offset = usize;

    fn new(_: Self::Context, range: Range<Self::Offset>) -> Self {
        Self::new(range.start, range.end)
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

pub fn stream_from_str<'a>(
    s: &'a str,
) -> Stream<'a, char, Span, Box<dyn Iterator<Item = (char, Span)> + 'a>> {
    let len = s.chars().count();
    Stream::from_iter(
        Span::new(len, len),
        Box::new(s.chars().enumerate().map(|(i, c)| (c, Span::new(i, i + 1)))),
    )
}
