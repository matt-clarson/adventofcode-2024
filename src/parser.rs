use std::io::{Bytes, Read};

/// A concrete [Parser] instance for working with types implementing [std::io::Read].
pub type BytesParser<R> = Parser<BytesReader<R>>;

/// A parser over a stream of bytes, where reading each byte can produce an error (e.g. a byte
/// stream being pulled from some IO source.
/// Provides low-level methods for parsing the byte stream, and higher-level methods for parsing
/// common lexemes.
pub struct Parser<S: Iterator<Item = anyhow::Result<u8>>> {
    source: S,
    peeked: Option<char>,
}

/// Utility that maps errors produced by [Bytes](std::io::Bytes) to [anyhow::Error].
pub struct BytesReader<R: Read>(Bytes<R>);

impl<R: Read> Iterator for BytesReader<R> {
    type Item = anyhow::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|r| r.map_err(|e| e.into()))
    }
}

impl<R> From<R> for Parser<BytesReader<R>>
where
    R: Read,
{
    fn from(value: R) -> Self {
        Self::new(BytesReader(value.bytes()))
    }
}

impl<S: Iterator<Item = anyhow::Result<u8>>> Parser<S> {
    /// Create a new parser from a source stream.
    pub fn new(source: S) -> Self {
        Self {
            source,
            peeked: None,
        }
    }

    /// Eagerly consumes digit characters from the source stream and parses them into a single
    /// integer value. Integer can also be started with '-' for negatives.
    ///
    /// If the next character in the stream is neither a digit or '-', returns None without
    /// consuming any characters.
    pub fn integer(&mut self) -> Option<anyhow::Result<i32>> {
        let mut s = match self.next_if(|c| c.is_ascii_digit() || c == '-') {
            Some(Ok(c)) => format!("{c}"),
            Some(Err(err)) => return Some(Err(err)),
            None => return None,
        };

        while let Some(c) = self.next_if(|c| c.is_ascii_digit()) {
            match c {
                Ok(c) => s.push(c),
                Err(err) => return Some(Err(err)),
            }
        }

        Some(s.parse().map_err(anyhow::Error::from))
    }

    /// Eagerly consume all characters matching arg `c`, stop at the first character that does not
    /// match without consuming that character from the stream.
    pub fn skip_if_eq(&mut self, c: char) -> anyhow::Result<()> {
        while let Some(next) = self.next_if_eq(c) {
            next?;
        }

        Ok(())
    }

    /// Consume and return the next character in the stream if the provided function `f` returns
    /// `true` when passed that character, otherwise returns `None` and does not consume any
    /// characters from the stream.
    pub fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<anyhow::Result<char>> {
        match self.peek() {
            Some(Ok(peeked)) if f(peeked) => self.peeked.take().map(Ok),
            Some(Err(err)) => Some(Err(err)),
            _ => None,
        }
    }

    /// Consume and return the next character in the stream if that character equals `c`, otherwise
    /// returns `None` and does not consume any characters from the stream.
    pub fn next_if_eq(&mut self, c: char) -> Option<anyhow::Result<char>> {
        match self.peek() {
            Some(Ok(peeked)) if peeked == c => self.peeked.take().map(Ok),
            Some(Err(err)) => Some(Err(err)),
            _ => None,
        }
    }

    /// Returns the next character in the stream without consuming that value. Repeated calls will
    /// return the same value without advancing the stream.
    ///
    /// Note that this function _will_ advance the underlying stream by one when loading a
    /// previously un-peeked character.
    pub fn peek(&mut self) -> Option<anyhow::Result<char>> {
        if let Some(c) = self.peeked {
            return Some(Ok(c));
        }

        match self.take_next() {
            Some(Ok(c)) => {
                self.peeked = Some(c);
                Some(Ok(c))
            }
            other => other,
        }
    }

    /// Advance the underlying stream by one and return the next character. Returns `None` when the
    /// stream ends.
    pub fn next(&mut self) -> Option<anyhow::Result<char>> {
        if let Some(c) = self.peeked.take() {
            return Some(Ok(c));
        }
        self.take_next()
    }

    fn take_next(&mut self) -> Option<anyhow::Result<char>> {
        self.source
            .next()
            .map(|b| b.map(char::from).map_err(anyhow::Error::from))
    }
}

#[cfg(test)]
mod test {
    use crate::assert_matches;

    use super::Parser;

    macro_rules! parser_for {
        ($e:expr) => {{
            let source = $e.as_bytes().into_iter().map(|b| anyhow::Result::Ok(*b));
            Parser::new(source)
        }};
    }

    #[test]
    fn parser_takes_next_char_until_eof() {
        let mut parser = parser_for!("abc");

        assert_matches!(parser.next(), Some(Ok('a')));
        assert_matches!(parser.next(), Some(Ok('b')));
        assert_matches!(parser.next(), Some(Ok('c')));
        assert!(parser.next().is_none());
    }

    #[test]
    fn parser_peeks_next_char_until_next_called() {
        let mut parser = parser_for!("abc");

        assert_matches!(parser.peek(), Some(Ok('a')));
        assert_matches!(parser.peek(), Some(Ok('a')));
        assert_matches!(parser.next(), Some(Ok('a')));
        assert_matches!(parser.peek(), Some(Ok('b')));
    }

    #[test]
    fn parser_takes_next_if_eq() {
        let mut parser = parser_for!("abc");

        assert_matches!(parser.peek(), Some(Ok('a')));
        assert!(parser.next_if_eq('b').is_none());
        assert_matches!(parser.peek(), Some(Ok('a')));
        assert_matches!(parser.next_if_eq('a'), Some(Ok('a')));
        assert_matches!(parser.peek(), Some(Ok('b')));
    }

    #[test]
    fn parser_takes_next_if_fn_returns_true() {
        let mut parser = parser_for!("abc");

        assert_matches!(parser.peek(), Some(Ok('a')));
        assert!(parser.next_if(|_| false).is_none());
        assert_matches!(parser.peek(), Some(Ok('a')));
        assert_matches!(parser.next_if(|_| true), Some(Ok('a')));
        assert_matches!(parser.peek(), Some(Ok('b')));
    }

    #[test]
    fn parser_skip_if_eq() {
        let mut parser = parser_for!("abc");

        assert_matches!(parser.next(), Some(Ok('a')));
        assert!(parser.skip_if_eq(' ').is_ok());
        assert_matches!(parser.next(), Some(Ok('b')));
    }

    #[test]
    fn parser_parses_single_digit_integer() {
        let mut parser = parser_for!("1");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            1
        );
    }

    #[test]
    fn parser_parses_multi_digit_integer() {
        let mut parser = parser_for!("123");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            123
        );
    }

    #[test]
    fn parser_parses_integer_with_leading_zeros() {
        let mut parser = parser_for!("0001");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            1
        );
    }

    #[test]
    fn parser_parses_integer_with_trailing_zeros() {
        let mut parser = parser_for!("1000");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            1000
        );
    }

    #[test]
    fn parser_parses_integer_with_zeros() {
        let mut parser = parser_for!("10002");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            10002
        );
    }

    #[test]
    fn parser_parses_negative_integer() {
        let mut parser = parser_for!("-1");

        assert_eq!(
            parser
                .integer()
                .expect("returns value")
                .expect("returns non-error"),
            -1
        );
    }
}
