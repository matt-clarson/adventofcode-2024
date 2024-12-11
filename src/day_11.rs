use std::io::{BufRead, Read};

use gxhash::{HashMap, HashMapExt};
use smol_str::{SmolStr, SmolStrBuilder, ToSmolStr};

use crate::{day::Day, parser::Parser};

struct Stones {
    cache: HashMap<SmolStr, u64>,
}

impl<R: Read> From<R> for Stones {
    fn from(value: R) -> Self {
        let mut parser = Parser::from(value);
        let mut cache = HashMap::new();
        while parser.eof().is_none() {
            let mut s = SmolStrBuilder::new();
            parser.skip_if_eq(' ');
            while let Some(c) = parser.next_if(|c| c.is_ascii_digit()) {
                s.push(c);
            }
            let s = s.finish();
            if let Some(n) = cache.get_mut(&s) {
                let _ = std::mem::replace(n, *n + 1);
            } else {
                cache.insert(s, 1);
            }
        }

        Self { cache }
    }
}

enum Op {
    Add(SmolStr, u64),
    Sub(SmolStr, u64),
}

impl Stones {
    fn iterations(&mut self, n: usize) -> u64 {
        for _ in 0..n {
            let mut c = self.cache.clone();
            c.retain(|_, v| *v > 0);
            self.step();
        }

        return self.cache.values().sum();
    }

    fn step(&mut self) {
        let ops: Vec<Op> = self
            .cache
            .iter()
            .flat_map(|(s, n)| {
                if *n == 0 {
                    vec![].into_iter()
                } else if s == "0" {
                    vec![Op::Add("1".into(), *n), Op::Sub("0".into(), *n)].into_iter()
                } else if s.len() % 2 == 0 {
                    let (left, right) = s.split_at(s.len() / 2);
                    vec![
                        Op::Add(
                            unsafe { left.parse::<u64>().unwrap_unchecked() }.to_smolstr(),
                            *n,
                        ),
                        Op::Add(
                            unsafe { right.parse::<u64>().unwrap_unchecked() }.to_smolstr(),
                            *n,
                        ),
                        Op::Sub(s.clone(), *n),
                    ]
                    .into_iter()
                } else {
                    let x = unsafe { s.parse::<u64>().unwrap_unchecked() };
                    vec![Op::Add((x * 2024).to_smolstr(), *n), Op::Sub(s.clone(), *n)].into_iter()
                }
            })
            .collect();
        for op in ops {
            match op {
                Op::Add(s, x) => {
                    if let Some(n) = self.cache.get_mut(&s) {
                        let _ = std::mem::replace(n, *n + x);
                    } else {
                        self.cache.insert(s, x);
                    }
                }
                Op::Sub(s, x) => {
                    if let Some(n) = self.cache.get_mut(&s) {
                        let _ = std::mem::replace(n, *n - x);
                    }
                }
            }
        }
    }
}

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let stones = Stones::from(input).iterations(25);
    Ok(stones.to_string())
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let stones = Stones::from(input).iterations(75);
    Ok(stones.to_string())
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
        "125 17",
        "55312"
    }
}
