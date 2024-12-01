use std::io::{BufRead, BufReader, Read};

#[macro_export]
macro_rules! test_solution {
    ($part_fn:ident $test_name:ident $input:expr , $expected:expr) => {
        #[test]
        fn $test_name() {
            let input = $crate::test_util::StringBufRead::from($input);

            let output = $part_fn(input).expect("no error to be raised");

            assert_eq!($expected, output)
        }
    };
}

#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $($pattern:tt)*) => {
        assert!(matches!($value, $($pattern)*))
    };
}

pub use test_solution;

pub struct StringBufRead<'a>(BufReader<StringReader<'a>>);

impl<'a> From<&'a str> for StringBufRead<'a> {
    fn from(value: &'a str) -> Self {
        StringBufRead(BufReader::new(StringReader::new(value)))
    }
}

impl<'a> Read for StringBufRead<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> BufRead for StringBufRead<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

struct StringReader<'a> {
    iter: std::slice::Iter<'a, u8>,
}

impl<'a> StringReader<'a> {
    /// Wrap a string in a `StringReader`, which implements `std::io::Read`.
    pub fn new(data: &'a str) -> Self {
        Self {
            iter: data.as_bytes().iter(),
        }
    }
}

impl<'a> Read for StringReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for (i, b) in buf.iter_mut().enumerate() {
            if let Some(x) = self.iter.next() {
                let _ = std::mem::replace(b, *x);
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }
}
