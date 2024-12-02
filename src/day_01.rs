use std::{
    collections::HashMap,
    io::{BufRead, Read},
    iter::zip,
};

use anyhow::anyhow;

use crate::{
    parser::{BytesParser, Parser},
    problem::Problem,
};

struct Pairs<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for Pairs<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> Iterator for Pairs<R> {
    type Item = anyhow::Result<(i32, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.parser.integer()?;

        let right = if let Some(n) = self.parser.integer() {
            n
        } else {
            return Some(Err(anyhow!("expect two integers per-line")));
        };

        self.parser
            .take_newline()
            .or_else(|| self.parser.eof())
            .and(Some(Ok((left, right))))
            .or(Some(Err(anyhow!(
                "expected line to end after second integer"
            ))))
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let pairs = Pairs::from(input);

    let mut left = vec![];
    let mut right = vec![];

    for pair in pairs {
        let (i0, i1) = pair?;
        left.push(i0);
        right.push(i1);
    }

    left.sort_unstable();
    right.sort_unstable();

    let sum = zip(left, right).fold(0, |acc, (i0, i1)| acc + (i1 - i0).abs());

    Ok(format!("{sum}"))
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let pairs = Pairs::from(input);

    let mut left = vec![];
    let mut nums = HashMap::new();

    for pair in pairs {
        let (i0, i1) = pair?;

        left.push(i0);

        if let Some(v) = nums.get_mut(&i1) {
            let _ = std::mem::replace(v, *v + 1);
        } else {
            nums.insert(i1, 1);
        }
    }

    let sum = left
        .iter()
        .fold(0, |acc, n| acc + n * nums.get(n).copied().unwrap_or(0));

    Ok(format!("{sum}"))
}

pub fn solution<I: BufRead>() -> Problem<I> {
    Problem::part_1(part_1).part_2(part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::test_solution;

    test_solution! {
        part_1 part_one_default_test_case
        "3   4
4   3
2   5
1   3
3   9
3   3",
        "11"
    }

    test_solution! {
        part_1 part_one_handles_multi_digit_inputs
        "11   15
4   18
15   203",
        "206"
    }

    test_solution! {
        part_1 part_one_handles_larger_left_hand_side
        "15   4
12   10
20   7",
        "26"
    }

    test_solution! {
        part_2 part_two_default_test_case
        "3   4
4   3
2   5
1   3
3   9
3   3",
        "31"
    }
}
