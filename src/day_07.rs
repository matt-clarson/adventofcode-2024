use std::io::{BufRead, Read};

use anyhow::anyhow;

use crate::{
    day::Day,
    parser::{BytesParser, Parser},
};

struct Case(i64, Vec<i64>);

struct Cases<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for Cases<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> Iterator for Cases<R> {
    type Item = anyhow::Result<Case>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parser.eof().is_some() {
            return None;
        }

        let n = if let Some(n) = self.parser.next_integer() {
            n
        } else {
            return Some(Err(anyhow!("line must start with integer")));
        };

        if self.parser.next_if_eq(':').is_none() {
            return Some(Err(anyhow!("first integer must be followed by ':'")));
        }

        let mut v = vec![];
        while self
            .parser
            .take_newline()
            .or_else(|| self.parser.eof())
            .is_none()
        {
            if let Some(n) = self.parser.next_integer() {
                v.push(n);
            } else {
                return Some(Err(anyhow!(
                    "':' can only be followed by integers and whitespace"
                )));
            }
        }

        if v.is_empty() {
            return Some(Err(anyhow!("need at least one integer after ':'")));
        }

        Some(Ok(Case(n, v)))
    }
}

fn is_computable(n: i64, xs: &[i64]) -> bool {
    let (last, xs) = match xs.split_last() {
        None => return false,
        Some((last, [])) => return n == *last,
        Some((last, xs)) => (*last, xs),
    };

    n % last == 0 && is_computable(n / last, xs) || is_computable(n - last, xs)
}

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    Cases::from(input)
        .try_fold(0, |acc, case| {
            let Case(n, xs) = case?;
            if is_computable(n, &xs) {
                Ok(acc + n)
            } else {
                Ok(acc)
            }
        })
        .map(|sum| sum.to_string())
}

fn un_concat(n: i64, x: i64) -> Option<i64> {
    let ns = n.to_string();
    let xs = x.to_string();

    let (n0, n1) = ns.split_at_checked(ns.len() - xs.len())?;

    if xs != n1 {
        return None;
    }
    Some(n0.parse().unwrap_or(0))
}

fn is_computable_v2(n: i64, xs: &[i64]) -> bool {
    let (last, xs) = match xs.split_last() {
        None => return false,
        Some((last, [])) => return n == *last,
        Some((last, xs)) => (*last, xs),
    };

    n % last == 0 && is_computable_v2(n / last, xs)
        || un_concat(n, last).is_some_and(|n| is_computable_v2(n, xs))
        || is_computable_v2(n - last, xs)
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    Cases::from(input)
        .try_fold(0, |acc, case| {
            let Case(n, xs) = case?;
            if is_computable_v2(n, &xs) {
                Ok(acc + n)
            } else {
                Ok(acc)
            }
        })
        .map(|sum| sum.to_string())
}

pub fn solution<I: BufRead>() -> Day<I> {
    Day::part_1(part_1).part_2(part_2)
}

#[cfg(test)]
mod test {
    use crate::test_solution;

    use super::*;

    test_solution! {
        part_1 part_one_default_case
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        "3749"
    }

    test_solution! {
        part_1 part_one_handle_1_in_input
        "28383880: 20 47 9 76 1 89 469",
        "28383880"
    }

    test_solution! {
        part_1 part_one_handle_64bit_result
        "2147483647: 2147483645 2
    2147483647: 2147483645 2",
        "4294967294"
    }

    test_solution! {
        part_2 part_two_default_case
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        "11387"
    }

    test_solution! {
        part_2 part_two_handles_all_concat
        "1234: 1 2 3 4",
        "1234"
    }

    test_solution! {
        part_2 part_two_handles_too_much_concat
        "1234: 4 12 3 4",
        "0"
    }
}
