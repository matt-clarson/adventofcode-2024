use std::io::{BufRead, Read};

use crate::{
    day::Day,
    parser::{BytesParser, Parser},
};

enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

struct Instructions<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for Instructions<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> Instructions<R> {
    fn instr(&mut self) -> Option<anyhow::Result<Instruction>> {
        match self.parser.take_matching(["mul(", "do()", "don't()"])? {
            "mul(" => {
                let left = self.parser.integer()?;
                let right = self
                    .parser
                    .next_if_eq(',')
                    .and_then(|_| self.parser.integer())?;
                self.parser
                    .next_if_eq(')')
                    .map(|_| Ok(Instruction::Mul(left, right)))
            }
            "do()" => Some(Ok(Instruction::Do)),
            "don't()" => Some(Ok(Instruction::Dont)),
            _ => unreachable!(),
        }
    }
}

impl<R: Read> Iterator for Instructions<R> {
    type Item = anyhow::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.parser.eof().is_none() {
            if let Some(instr) = self.instr() {
                return Some(instr);
            } else {
                self.parser.next();
            }
        }
        None
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    Instructions::from(input)
        .try_fold(0, |acc, instr| match instr? {
            Instruction::Mul(i0, i1) => Ok(acc + i0 * i1),
            _ => Ok(acc),
        })
        .map(|n| n.to_string())
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let mut skip_mul = false;
    Instructions::from(input)
        .try_fold(0, |acc, instr| match instr? {
            Instruction::Mul(i0, i1) if !skip_mul => Ok(acc + i0 * i1),
            Instruction::Do => {
                skip_mul = false;
                Ok(acc)
            }
            Instruction::Dont => {
                skip_mul = true;
                Ok(acc)
            }
            _ => Ok(acc),
        })
        .map(|n| n.to_string())
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
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    ",
        "161"

    }

    test_solution! {
        part_2 part_two_default_case
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "48"
    }
}
