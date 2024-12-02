use std::io::{Bytes, Read};

/// A concrete [Parser] instance for working with types implementing [std::io::Read].
pub type BytesParser<R> = Parser<BytesReader<R>>;

/// A parser over a stream of bytes, where reading each byte can produce an error (e.g. a byte
/// stream being pulled from some IO source.
/// Panics when the underlying stream does produce an error.
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

    /// Utility for asserting that the source stream has reached its end. Consumes all proceeeding
    /// whitespace.
    /// Returns `None` if there are non-whitespace characters remaining in the stream.
    pub fn eof(&mut self) -> Option<()> {
        self.skip_if_eq(' ');
        if self.peek().is_none() {
            Some(())
        } else {
            None
        }
    }

    /// Utility for consuming a newline character (`\n`). Consumes all proceeding whitespace.
    /// Returns `None` if the next character after whitespace is not a newline chaaracter.
    pub fn take_newline(&mut self) -> Option<()> {
        self.skip_if_eq(' ');
        self.next_if_eq('\n').and(Some(()))
    }

    /// Eagerly consumes digit characters from the source stream and parses them into a single
    /// integer value. Integer can also be started with '-' for negatives.
    ///
    /// Consumes all whitespace characters (besides newlines `\n`).
    /// If the next character in the stream is neither a digit or '-', returns None.
    pub fn integer(&mut self) -> Option<i32> {
        self.skip_if_eq(' ');
        let mut s = self
            .next_if(|c| c.is_ascii_digit() || c == '-')
            .map(|c| format!("{c}"))?;

        while let Some(c) = self.next_if(|c| c.is_ascii_digit()) {
            s.push(c)
        }

        // SAFTEY: s only contains valid integer characters.
        Some(unsafe { s.parse().unwrap_unchecked() })
    }

    /// Eagerly consume all characters matching arg `c`, stop at the first character that does not
    /// match without consuming that character from the stream.
    pub fn skip_if_eq(&mut self, c: char) {
        while self.next_if_eq(c).is_some() {}
    }

    /// Consume and return the next character in the stream if the provided function `f` returns
    /// `true` when passed that character, otherwise returns `None` and does not consume any
    /// characters from the stream.
    pub fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<char> {
        self.peek()
            .filter(|peeked| f(*peeked))
            .and_then(|_| self.peeked.take())
    }

    /// Consume and return the next character in the stream if that character equals `c`, otherwise
    /// returns `None` and does not consume any characters from the stream.
    pub fn next_if_eq(&mut self, c: char) -> Option<char> {
        self.peek()
            .filter(|peeked| *peeked == c)
            .and_then(|_| self.peeked.take())
    }

    /// Returns the next character in the stream without consuming that value. Repeated calls will
    /// return the same value without advancing the stream.
    ///
    /// Note that this function _will_ advance the underlying stream by one when loading a
    /// previously un-peeked character.
    pub fn peek(&mut self) -> Option<char> {
        self.peeked.or_else(|| {
            let next = self.take_next()?;
            self.peeked.replace(next);
            Some(next)
        })
    }

    /// Advance the underlying stream by one and return the next character. Returns `None` when the
    /// stream ends.
    #[allow(unused)]
    pub fn next(&mut self) -> Option<char> {
        self.peeked.take().or_else(|| self.take_next())
    }

    fn take_next(&mut self) -> Option<char> {
        self.source.next().map(|b| match b {
            Ok(b) => b.into(),
            Err(err) => panic!("source stream produced an error: {}", err),
        })
    }
}

#[cfg(test)]
mod test {
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

        assert_eq!(parser.next(), Some('a'));
        assert_eq!(parser.next(), Some('b'));
        assert_eq!(parser.next(), Some('c'));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn parser_peeks_next_char_until_next_called() {
        let mut parser = parser_for!("abc");

        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.next(), Some('a'));
        assert_eq!(parser.peek(), Some('b'));
    }

    #[test]
    fn parser_takes_next_if_eq() {
        let mut parser = parser_for!("abc");

        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.next_if_eq('b'), None);
        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.next_if_eq('a'), Some('a'));
        assert_eq!(parser.peek(), Some('b'));
    }

    #[test]
    fn parser_takes_next_if_fn_returns_true() {
        let mut parser = parser_for!("abc");

        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.next_if(|_| false), None);
        assert_eq!(parser.peek(), Some('a'));
        assert_eq!(parser.next_if(|_| true), Some('a'));
        assert_eq!(parser.peek(), Some('b'));
    }

    #[test]
    fn parser_skip_if_eq() {
        let mut parser = parser_for!("abc");

        assert_eq!(parser.next(), Some('a'));
        parser.skip_if_eq('a');
        assert_eq!(parser.next(), Some('b'));
    }

    #[test]
    fn parser_parses_single_digit_integer() {
        let mut parser = parser_for!("1");

        assert_eq!(parser.integer(), Some(1));
    }

    #[test]
    fn parser_parses_multi_digit_integer() {
        let mut parser = parser_for!("123");

        assert_eq!(parser.integer(), Some(123));
    }

    #[test]
    fn parser_parses_integer_with_leading_zeros() {
        let mut parser = parser_for!("0001");

        assert_eq!(parser.integer(), Some(1));
    }

    #[test]
    fn parser_parses_integer_with_trailing_zeros() {
        let mut parser = parser_for!("1000");

        assert_eq!(parser.integer(), Some(1000));
    }

    #[test]
    fn parser_parses_integer_with_zeros() {
        let mut parser = parser_for!("10002");

        assert_eq!(parser.integer(), Some(10002));
    }

    #[test]
    fn parser_parses_negative_integer() {
        let mut parser = parser_for!("-1");

        assert_eq!(parser.integer(), Some(-1));
    }

    #[test]
    fn parser_skips_white_space_before_parsing_integer() {
        let mut parser = parser_for!("     1  39     -8");

        assert_eq!(parser.integer(), Some(1));
        assert_eq!(parser.integer(), Some(39));
        assert_eq!(parser.integer(), Some(-8));
        assert_eq!(parser.integer(), None);
    }

    #[test]
    fn parser_take_newline() {
        let mut parser = parser_for!("1\n\n2");

        assert_eq!(parser.take_newline(), None);
        assert_eq!(parser.next(), Some('1'));
        assert_eq!(parser.take_newline(), Some(()));
        assert_eq!(parser.take_newline(), Some(()));
        assert_eq!(parser.take_newline(), None);
    }

    #[test]
    fn parser_take_newline_skips_whitespace() {
        let mut parser = parser_for!("    \n1");

        assert_eq!(parser.take_newline(), Some(()));
        assert_eq!(parser.next(), Some('1'));
    }

    #[test]
    fn parser_eof() {
        let mut parser = parser_for!("1");

        assert_eq!(parser.eof(), None);
        assert_eq!(parser.next(), Some('1'));
        assert_eq!(parser.next(), None);
        assert_eq!(parser.eof(), Some(()));
    }

    #[test]
    fn parser_eof_skips_whitespace() {
        let mut parser = parser_for!("     ");

        assert_eq!(parser.eof(), Some(()));
    }
}
