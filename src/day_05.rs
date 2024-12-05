use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Read},
};

use anyhow::anyhow;

use crate::{
    day::Day,
    parser::{BytesParser, Parser},
};

#[derive(Debug)]
enum SafetyUpdate {
    Ordering(i32, i32),
    Break,
    Instructions(Vec<i32>),
}

struct SafetyUpdates<R: Read> {
    parser: BytesParser<R>,
}

impl<R: Read> From<R> for SafetyUpdates<R> {
    fn from(value: R) -> Self {
        Self {
            parser: Parser::from(value),
        }
    }
}

impl<R: Read> SafetyUpdates<R> {
    fn take_break(&mut self) -> Option<anyhow::Result<SafetyUpdate>> {
        if self.parser.take_newline().is_some() {
            Some(Ok(SafetyUpdate::Break))
        } else {
            None
        }
    }

    fn take_ordering(&mut self) -> Option<anyhow::Result<SafetyUpdate>> {
        let is_ordering = self
            .parser
            .peek_n(5)
            .as_bytes()
            .get(2)
            .filter(|b| char::from(**b) == '|')
            .is_some();

        if !is_ordering {
            return None;
        }

        let left = self
            .parser
            .next_integer()
            .expect("ordering line: next characters should be integer");
        self.parser
            .next_if_eq('|')
            .expect("ordering line next character should be '|'");
        let right = self
            .parser
            .next_integer()
            .expect("ordering line: next characters should be integer");

        self.parser
            .take_newline()
            .or_else(|| self.parser.eof())
            .expect("ordering line: should end on newline/EOF");

        Some(Ok(SafetyUpdate::Ordering(left, right)))
    }

    /// Must be called _after_ take_ordering.
    fn take_instruction(&mut self) -> Option<anyhow::Result<SafetyUpdate>> {
        let mut v = if let Some(n) = self.parser.next_integer() {
            vec![n]
        } else {
            return None;
        };

        while self
            .parser
            .take_newline()
            .or_else(|| self.parser.eof())
            .is_none()
        {
            if let Some(n) = self
                .parser
                .next_if_eq(',')
                .and_then(|_| self.parser.next_integer())
            {
                v.push(n);
            } else {
                return Some(Err(anyhow!(
                    "instruction must be a sequence of integer and ',' pairs."
                )));
            }
        }

        Some(Ok(SafetyUpdate::Instructions(v)))
    }
}

impl<R: Read> Iterator for SafetyUpdates<R> {
    type Item = anyhow::Result<SafetyUpdate>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_break()
            .or_else(|| self.take_ordering())
            .or_else(|| self.take_instruction())
    }
}

#[derive(Debug)]
struct Ordering {
    map: HashMap<i32, HashSet<i32>>,
}

impl Ordering {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, (left, right): (i32, i32)) {
        if let Some(set) = self.map.get_mut(&left) {
            set.insert(right);
        } else {
            self.map.insert(left, HashSet::from([right]));
        }
    }

    fn compare(&self, left: i32, right: i32) -> std::cmp::Ordering {
        self.map
            .get(&left)
            .and_then(|set| {
                if set.contains(&right) {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    None
                }
            })
            .or_else(|| {
                self.map.get(&right).and_then(|set| {
                    if set.contains(&left) {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(std::cmp::Ordering::Equal)
    }

    fn is_sorted<V: AsRef<[i32]>>(&self, xs: V) -> bool {
        xs.as_ref()
            .windows(2)
            .all(|x| self.compare(x[0], x[1]).is_ge())
    }

    fn get_middle_if_sorted(&self, xs: Vec<i32>) -> Option<i32> {
        if !self.is_sorted(&xs) {
            return None;
        }

        Some(xs[xs.len() / 2])
    }

    fn get_middle_if_not_sorted(&self, mut xs: Vec<i32>) -> Option<i32> {
        if self.is_sorted(&xs) {
            return None;
        }

        xs.sort_unstable_by(|left, right| self.compare(*left, *right));
        Some(xs[xs.len() / 2])
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let mut safety_updates = SafetyUpdates::from(input);
    let mut ordering = Ordering::new();

    for update in safety_updates
        .by_ref()
        .take_while(|u| !matches!(u, Ok(SafetyUpdate::Break)))
    {
        match update? {
            SafetyUpdate::Ordering(left, right) => ordering.insert((left, right)),
            other => return Err(anyhow!("unexpected safety update entry: {other:?}")),
        };
    }

    safety_updates
        .try_fold(0, |acc, update| match update? {
            SafetyUpdate::Instructions(instructions) => {
                if let Some(n) = ordering.get_middle_if_sorted(instructions) {
                    Ok(acc + n)
                } else {
                    Ok(acc)
                }
            }
            other => Err(anyhow!("unexpected safety update entry: {other:?}")),
        })
        .map(|n| n.to_string())
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let mut safety_updates = SafetyUpdates::from(input);
    let mut ordering = Ordering::new();

    for update in safety_updates
        .by_ref()
        .take_while(|u| !matches!(u, Ok(SafetyUpdate::Break)))
    {
        match update? {
            SafetyUpdate::Ordering(left, right) => ordering.insert((left, right)),
            other => return Err(anyhow!("unexpected safety update entry: {other:?}")),
        };
    }

    safety_updates
        .try_fold(0, |acc, update| match update? {
            SafetyUpdate::Instructions(instructions) => {
                if let Some(n) = ordering.get_middle_if_not_sorted(instructions) {
                    Ok(acc + n)
                } else {
                    Ok(acc)
                }
            }
            other => Err(anyhow!("unexpected safety update entry: {other:?}")),
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
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        "143"
    }

    test_solution! {
        part_2 part_two_default_case
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        "123"
    }
}
