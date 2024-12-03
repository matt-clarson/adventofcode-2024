use std::{
    collections::VecDeque,
    io::{Bytes, Read},
};

/// A [Parser] instance for working with types implementing [std::io::Read].
pub type BytesParser<R> = Parser<BytesReader<R>>;

/// A parser over a stream of bytes, where reading each byte can produce an error (e.g. a byte
/// stream being pulled from some IO source.
/// Panics when the underlying stream does produce an error.
/// Provides low-level methods for parsing the byte stream, and higher-level methods for parsing
/// common lexemes.
pub struct Parser<S: Iterator<Item = anyhow::Result<u8>>> {
    source: S,
    peeked: VecDeque<char>,
    peeked_container: String,
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
            peeked: VecDeque::new(),
            peeked_container: String::with_capacity(8),
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
    pub fn next_integer(&mut self) -> Option<i32> {
        self.skip_if_eq(' ');
        self.integer()
    }

    /// Eagerly consumes digit characters from the source stream and parses them into a single
    /// integer value. Integer can also be started with '-' for negatives.
    /// If the next character in the stream is neither a digit or '-', returns None.
    ///
    /// Note that the method [Parser::next_integer] exists as a wrapper for this method that also
    /// consumes leading whitespace before the next integer.
    pub fn integer(&mut self) -> Option<i32> {
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

    #[allow(unused)]
    /// Eagerly consume all characters matching function `f`, stop at the first character that does
    /// not match without consuming that character from the stream.
    pub fn skip_if<F: Fn(char) -> bool>(&mut self, f: F) {
        while self.next_if(&f).is_some() {}
    }

    #[allow(unused)]
    /// Skip the next `n` characters.
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    /// Consume and return the next character in the stream if the provided function `f` returns
    /// `true` when passed that character, otherwise returns `None` and does not consume any
    /// characters from the stream.
    pub fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<char> {
        self.peek()
            .filter(|peeked| f(*peeked))
            .and_then(|_| self.peeked.pop_back())
    }

    /// Consume and return the next character in the stream if that character equals `c`, otherwise
    /// returns `None` and does not consume any characters from the stream.
    pub fn next_if_eq(&mut self, c: char) -> Option<char> {
        self.peek()
            .filter(|peeked| *peeked == c)
            .and_then(|_| self.peeked.pop_back())
    }

    #[allow(unused)]
    /// Iterates over a list of `&'static str`s and returns the first that matches the next
    /// characters in the source stream.
    /// If a match is found, consumes the matching `str`s bytes from tehs tream. Otherwise the
    /// stream is not advanced (except where some portion of the stream is cached internally).
    ///
    /// If there is a need to map the matched strings to values, consider using
    /// [Parser::take_matching_and].
    pub fn take_matching<V: IntoIterator<Item = &'static str>>(
        &mut self,
        v: V,
    ) -> Option<&'static str> {
        v.into_iter().find_map(|s| {
            let n = s.as_bytes().iter().try_fold(0usize, |i, b| {
                if self.peek_n(i + 1).as_bytes().get(i) == Some(b) {
                    Some(i + 1)
                } else {
                    None
                }
            })?;
            self.skip(n);
            Some(s)
        })
    }

    #[allow(unused)]
    /// Iterates over a list of `&'static str` and T pairs, and returns the T for the first string
    /// that matches the next characters in the source stream.
    /// If a match is found, consumes the matching `str`s bytes from tehs tream. Otherwise the
    /// stream is not advanced (except where some portion of the stream is cached internally).
    pub fn take_matching_and<T, V: IntoIterator<Item = (&'static str, T)>>(
        &mut self,
        v: V,
    ) -> Option<T> {
        v.into_iter().find_map(|(s, t)| {
            let n = s.as_bytes().iter().try_fold(0usize, |i, b| {
                if self.peek_n(i + 1).as_bytes().get(i) == Some(b) {
                    Some(i + 1)
                } else {
                    None
                }
            })?;
            self.skip(n);
            Some(t)
        })
    }

    #[allow(unused)]
    /// Load the next `n` characters from the stream into a buffer and return them. The buffer is
    /// cached and consumed prior to reading anymore values from the stream.
    pub fn peek_n(&mut self, n: usize) -> &str {
        if n > self.peeked.len() {
            for _ in 0..(n - self.peeked.len()) {
                if let Some(c) = self.take_next() {
                    self.peeked.push_front(c);
                } else {
                    break;
                }
            }
        }

        self.peeked_container.clear();
        self.peeked_container
            .extend(self.peeked.iter().rev().take(n));

        &self.peeked_container
    }

    /// Returns the next character in the stream without consuming that value. Repeated calls will
    /// return the same value without advancing the stream.
    ///
    /// Note that this function _will_ advance the underlying stream by one when loading a
    /// previously un-peeked character.
    pub fn peek(&mut self) -> Option<char> {
        self.peeked.back().copied().or_else(|| {
            let next = self.take_next()?;
            self.peeked.push_front(next);
            Some(next)
        })
    }

    /// Advance the underlying stream by one and return the next character. Returns `None` when the
    /// stream ends.
    #[allow(unused)]
    pub fn next(&mut self) -> Option<char> {
        self.peeked.pop_back().or_else(|| self.take_next())
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
    fn parser_skip_if() {
        let mut parser = parser_for!("abc");

        assert_eq!(parser.next(), Some('a'));
        parser.skip_if(|c| c == 'a' || c == 'b');
        assert_eq!(parser.next(), Some('c'));
    }

    #[test]
    fn parser_parses_single_digit_integer() {
        let mut parser = parser_for!("1");

        assert_eq!(parser.next_integer(), Some(1));
    }

    #[test]
    fn parser_parses_multi_digit_integer() {
        let mut parser = parser_for!("123");

        assert_eq!(parser.next_integer(), Some(123));
    }

    #[test]
    fn parser_parses_integer_with_leading_zeros() {
        let mut parser = parser_for!("0001");

        assert_eq!(parser.next_integer(), Some(1));
    }

    #[test]
    fn parser_parses_integer_with_trailing_zeros() {
        let mut parser = parser_for!("1000");

        assert_eq!(parser.next_integer(), Some(1000));
    }

    #[test]
    fn parser_parses_integer_with_zeros() {
        let mut parser = parser_for!("10002");

        assert_eq!(parser.next_integer(), Some(10002));
    }

    #[test]
    fn parser_parses_negative_integer() {
        let mut parser = parser_for!("-1");

        assert_eq!(parser.next_integer(), Some(-1));
    }

    #[test]
    fn parser_skips_white_space_before_parsing_integer() {
        let mut parser = parser_for!("     1  39     -8");

        assert_eq!(parser.next_integer(), Some(1));
        assert_eq!(parser.next_integer(), Some(39));
        assert_eq!(parser.next_integer(), Some(-8));
        assert_eq!(parser.next_integer(), None);
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

    #[test]
    fn parser_peek_n() {
        let mut parser = parser_for!("hello world");

        assert_eq!(parser.peek_n(3), "hel");
        assert_eq!(parser.peek_n(4), "hell");
        assert_eq!(parser.peek_n(2), "he");

        parser.skip(8);

        assert_eq!(parser.peek_n(3), "rld");
        assert_eq!(parser.peek_n(5), "rld");
    }

    #[test]
    fn parser_take_matching() {
        let mut parser = parser_for!("onetowthreefonefive");

        macro_rules! numbers {
            () => {
                [
                    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
            };
        }

        assert_eq!(parser.take_matching(numbers!()), Some("one"));
        assert_eq!(parser.take_matching(numbers!()), None);
        assert_eq!(parser.next(), Some('t'));
        assert_eq!(parser.next(), Some('o'));
        assert_eq!(parser.next(), Some('w'));
        assert_eq!(parser.take_matching(numbers!()), Some("three"));
        assert_eq!(parser.take_matching(numbers!()), None);
        assert_eq!(parser.next(), Some('f'));
        assert_eq!(parser.take_matching(numbers!()), Some("one"));
        assert_eq!(parser.take_matching(numbers!()), Some("five"));
        assert_eq!(parser.take_matching(numbers!()), None);
    }

    #[test]
    fn parser_take_matching_and() {
        let mut parser = parser_for!("onetowthreefonefive");

        macro_rules! numbers {
            () => {
                [
                    ("zero", 0),
                    ("one", 1),
                    ("two", 2),
                    ("three", 3),
                    ("four", 4),
                    ("five", 5),
                    ("six", 6),
                    ("seven", 7),
                    ("eight", 8),
                    ("nine", 9),
                ]
            };
        }

        assert_eq!(parser.take_matching_and(numbers!()), Some(1));
        assert_eq!(parser.take_matching_and(numbers!()), None);
        assert_eq!(parser.next(), Some('t'));
        assert_eq!(parser.next(), Some('o'));
        assert_eq!(parser.next(), Some('w'));
        assert_eq!(parser.take_matching_and(numbers!()), Some(3));
        assert_eq!(parser.take_matching_and(numbers!()), None);
        assert_eq!(parser.next(), Some('f'));
        assert_eq!(parser.take_matching_and(numbers!()), Some(1));
        assert_eq!(parser.take_matching_and(numbers!()), Some(5));
        assert_eq!(parser.take_matching_and(numbers!()), None);
    }
}
