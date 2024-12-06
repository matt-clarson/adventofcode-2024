use std::{
    collections::HashSet,
    io::{BufRead, Read},
};

use anyhow::anyhow;

use crate::{day::Day, grid::Vec2};

struct Steps {
    width: usize,
    height: usize,
    positions: Vec<Vec2<usize>>,
    obstacle: Option<Vec2<usize>>,
    initial: Vec2<usize>,
    current: Option<Vec2<usize>>,
    direction: Vec2<isize>,
}

impl Steps {
    fn try_from<R: Read>(source: R) -> anyhow::Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut positions = vec![];
        let mut start = None;

        for (i, b) in source.bytes().enumerate() {
            match char::from(b?) {
                '\n' => {
                    width = if width == 0 { i } else { width };
                    height += 1;
                }
                '#' => positions.push(Vec2(i - (height * width) - height, height)),
                '^' => start = Some(Vec2(i - (height * width) - height, height)),
                _ => {}
            }
        }

        if positions.is_empty() {
            return Err(anyhow!("no block positions in map"));
        }

        start
            .ok_or(anyhow!("no start position in map"))
            .map(|start| Self {
                width,
                height,
                positions,
                initial: start,
                current: None,
                direction: Vec2(0, -1),
                obstacle: None,
            })
    }

    fn next_step(&self) -> Option<Vec2<usize>> {
        self.current
            .and_then(|p| p.try_add(self.direction, Vec2(self.width, self.height)))
    }

    fn reset_with_obstacle(&mut self, obstacle: Vec2<usize>) {
        self.current = None;
        self.direction = Vec2(0, -1);
        self.obstacle = Some(obstacle);
    }
}

impl Iterator for Steps {
    type Item = (Vec2<usize>, Vec2<isize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = Some(self.initial);
            return self.current.map(|p| (p, self.direction));
        }

        let next = self.next_step()?;
        if self.obstacle.is_some_and(|p| next == p) || self.positions.iter().any(|p| next == *p) {
            self.direction = self.direction.rotate_clockwise();
        } else {
            self.current = Some(next);
        }

        self.current.map(|p| (p, self.direction))
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    Steps::try_from(input).map(|s| s.map(|(p, _)| p).collect::<HashSet<_>>().len().to_string())
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let mut steps = Steps::try_from(input)?;
    let positions = steps.by_ref().map(|(p, _)| p).collect::<HashSet<_>>();

    let mut seen = HashSet::new();
    let mut n = 0;
    for p in &positions {
        seen.clear();
        steps.reset_with_obstacle(*p);
        for step in steps.by_ref() {
            if seen.contains(&step) {
                n += 1;
                break;
            }
            seen.insert(step);
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
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        "41"
    }

    test_solution! {
        part_2 part_two_default_case
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        "6"
    }

    test_solution! {
        part_2 loop_adjacent_to_edge
".#..
...#
....
.^#.",
        "1"
    }
}
