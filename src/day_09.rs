use std::{
    collections::BinaryHeap,
    io::{BufRead, Read},
};

use gxhash::{HashSet, HashSetExt};

use crate::{
    day::Day,
    parser::{BytesParser, Parser},
};

struct Digits<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for Digits<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> Iterator for Digits<R> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser
            .next_if(|c| c.is_ascii_digit())
            // SAFTEY: checking if char is digit then parsing to unsigned int
            .map(|c| unsafe { c.to_string().parse().unwrap_unchecked() })
    }
}

struct Files {
    digits: Vec<u64>,
    idx: usize,
}

impl<R: Read> From<Digits<R>> for Files {
    fn from(digits: Digits<R>) -> Self {
        Self {
            digits: digits.collect(),
            idx: 0,
        }
    }
}

impl Files {
    fn checksum(mut self) -> u64 {
        let mut sum = 0;

        let mut i = 0;
        // move back by two at a time, so set j initially such that j-2 == self.digits.len()-1;
        let mut j = self.digits.len() + 1;

        let mut space = 0;
        let mut size_to_move = 0;

        while i < j {
            let target_size = space.min(size_to_move);
            sum += self.file_sum(self.id(j), target_size);

            space -= target_size;
            size_to_move -= target_size;

            if size_to_move == 0 {
                j -= 2;
                size_to_move = self.digits[j];
            }

            if space == 0 {
                let next_file_size = self.digits[i];
                sum += self.file_sum(self.id(i), next_file_size);
                space += self.digits[i + 1];
                i += 2;
            }
        }

        sum + self.file_sum(self.id(i), size_to_move)
    }

    fn checksum_v2(mut self) -> u64 {
        let mut files_by_size = vec![BinaryHeap::new(); 10];

        self.digits
            .iter()
            .enumerate()
            .step_by(2)
            .for_each(|(i, n)| {
                files_by_size[*n as usize].push(self.id(i));
            });

        let mut i = 0;

        let mut space = 0;

        let mut sum = 0;

        let mut seen: HashSet<u64> = HashSet::new();

        while i < self.digits.len() {
            if space > 0 {
                let (size, _) = files_by_size
                    .iter()
                    .enumerate()
                    .filter_map(|(i, h)| h.peek().map(|n| (i, *n)))
                    .fold((0, 0), |acc, cand| {
                        if cand.0 <= space && cand.1 > acc.1 && !seen.contains(&cand.1) {
                            cand
                        } else {
                            acc
                        }
                    });
                if size > 0 {
                    let id = files_by_size[size].pop().unwrap();
                    sum += self.file_sum(id, size as u64);
                    space -= size;
                    seen.insert(id);
                } else {
                    self.idx += space;
                    space = 0;
                }
            }

            if space == 0 {
                let id = self.id(i);
                let next_file_size = self.digits[i];
                if seen.insert(id) {
                    sum += self.file_sum(self.id(i), next_file_size);
                } else {
                    self.idx += next_file_size as usize;
                }
                if i < self.digits.len() - 1 {
                    space += self.digits[i + 1] as usize;
                }
                i += 2;
            }
        }

        sum
    }

    fn id(&self, idx: usize) -> u64 {
        idx as u64 / 2
    }

    fn file_sum(&mut self, id: u64, size: u64) -> u64 {
        let mut sum = 0;
        for _ in 0..size {
            sum += id * self.idx as u64;
            self.idx += 1;
        }
        sum
    }
}

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let digits = Digits::from(input);

    Ok(Files::from(digits).checksum().to_string())
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let digits = Digits::from(input);

    Ok(Files::from(digits).checksum_v2().to_string())
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
        "2333133121414131402",
        "1928"
    }

    test_solution! {
        part_2 part_two_default_case
        "2333133121414131402",
        "2858"
    }
}
