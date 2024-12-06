use std::io::{BufRead, Read};

use anyhow::anyhow;
use gxhash::HashSetExt;

use crate::{day::Day, grid::Vec2};

struct Steps {
    width: usize,
    height: usize,
    positions: gxhash::HashSet<Vec2<usize>>,
    obstacle: Option<Vec2<usize>>,
    initial: Vec2<usize>,
    current: Option<Vec2<usize>>,
    direction: Vec2<isize>,
}

impl Steps {
    fn try_from<R: Read>(source: R) -> anyhow::Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut positions = gxhash::HashSet::new();
        let mut start = None;

        for (i, b) in source.bytes().enumerate() {
            match char::from(b?) {
                '\n' => {
                    width = if width == 0 { i } else { width };
                    height += 1;
                }
                '#' => {
                    positions.insert(Vec2(i - (height * width) - height, height));
                }
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
        self.positions.insert(obstacle);
        if let Some(prev) = self.obstacle.replace(obstacle) {
            self.positions.remove(&prev);
        }
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
        if self.positions.contains(&next) {
            self.direction = self.direction.rotate_clockwise();
        } else {
            self.current = Some(next);
        }

        self.current.map(|p| (p, self.direction))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.width * self.height))
    }
}

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    Steps::try_from(input).map(|s| {
        s.map(|(p, _)| p)
            .collect::<gxhash::HashSet<_>>()
            .len()
            .to_string()
    })
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let mut steps = Steps::try_from(input)?;
    let positions = steps
        .by_ref()
        .map(|(p, _)| p)
        .collect::<gxhash::HashSet<_>>();

    let mut seen = gxhash::HashSet::with_capacity(positions.len());

    let num_loops = positions.iter().fold(0, |acc, p| {
        seen.clear();
        steps.reset_with_obstacle(*p);
        if steps.by_ref().any(|step| !seen.insert(step)) {
            return acc + 1;
        }
        acc
    });

    Ok(num_loops.to_string())
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
