use anyhow::anyhow;
use std::io::{BufRead, Read};

use crate::{
    day::Day,
    parser::{BytesParser, Parser},
};

struct Integers<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for Integers<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> Iterator for Integers<R> {
    type Item = anyhow::Result<Vec<i64>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut integers = self.parser.next_integer().map(|n| vec![n])?;

        while self
            .parser
            .take_newline()
            .or_else(|| self.parser.eof())
            .is_none()
        {
            if let Some(n) = self.parser.next_integer() {
                integers.push(n);
            } else {
                return Some(Err(anyhow!(
                    "line can only contain integers and whitespace"
                )));
            }
        }
        Some(Ok(integers))
    }
}

#[derive(Eq, PartialEq)]
enum LineState {
    Incrementing,
    Decrementing,
    Unsafe,
}

fn calculate_local_line_state(i0: i64, i1: i64) -> LineState {
    let d = i0 - i1;
    if (1..4).contains(&d) {
        LineState::Decrementing
    } else if (1..4).contains(&d.abs()) {
        LineState::Incrementing
    } else {
        LineState::Unsafe
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    fn line_is_safe(ints: &[i64]) -> bool {
        let line_state = calculate_local_line_state(ints[0], ints[1]);
        if line_state == LineState::Unsafe {
            return false;
        }

        for i in 1..ints.len() - 1 {
            let local = calculate_local_line_state(ints[i], ints[i + 1]);
            if local != line_state {
                return false;
            }
        }

        true
    }

    let num_safe = Integers::from(input).try_fold(0, |acc, ints| {
        ints.map(|ints| if line_is_safe(&ints) { acc + 1 } else { acc })
    })?;
    Ok(format!("{num_safe}"))
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let line_is_safe = |ints: &[i64]| -> bool {
        let line_state = calculate_local_line_state(ints[0], ints[1]);
        if line_state == LineState::Unsafe {
            return false;
        }

        for i in 1..ints.len() - 1 {
            let local = calculate_local_line_state(ints[i], ints[i + 1]);
            if local != line_state {
                return false;
            }
        }

        true
    };

    let mut v = Vec::with_capacity(10);
    let mut n = 0;
    for ints in Integers::from(input) {
        let ints = ints?;

        if line_is_safe(&ints) {
            n += 1;
            continue;
        }

        for i in 0..ints.len() {
            v.clear();
            for (j, n) in ints.iter().enumerate() {
                if i == j {
                    continue;
                }
                v.push(*n);
            }

            if line_is_safe(&v) {
                n += 1;
                break;
            }
        }
    }

    Ok(n.to_string())
}

pub fn solution<I: BufRead>() -> Day<I> {
    Day::part_1(part_1).part_2(part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    test_solution! {
        part_1 part_one_default_case
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        "2"
    }

    test_solution! {
        part_2 part_two_default_case
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        "4"
    }
}
