use std::io::{BufRead, Read};

use gxhash::{HashMapExt, HashSetExt};

use crate::{day::Day, grid::Vec2};

struct Map {
    antennas: gxhash::HashMap<char, Vec<Vec2<usize>>>,
    width: usize,
    height: usize,
}

impl Map {
    fn try_from<R: Read>(source: R) -> anyhow::Result<Self> {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 1;

        let mut antennas: gxhash::HashMap<char, Vec<_>> = gxhash::HashMap::new();

        macro_rules! incr_x {
            () => {{
                x += 1;
                width = if height > 1 { width } else { width + 1 };
            }};
        }

        for c in source.bytes().map(|b| b.map(char::from)) {
            let c = c?;

            match c {
                '.' => incr_x!(),
                '\n' => {
                    x = 0;
                    y += 1;
                    height += 1;
                }
                _ => {
                    if let Some(xs) = antennas.get_mut(&c) {
                        xs.push(Vec2(x, y));
                    } else {
                        antennas.insert(c, vec![Vec2(x, y)]);
                    }
                    incr_x!();
                }
            }
        }

        Ok(Self {
            antennas,
            width,
            height,
        })
    }

    fn max_pos(&self) -> Vec2<usize> {
        Vec2(self.width - 1, self.height - 1)
    }

    fn antenna_pairs(&self) -> impl Iterator<Item = (Vec2<usize>, Vec2<usize>)> + '_ {
        self.antennas.values().flat_map(|xs| {
            xs.iter()
                .enumerate()
                .flat_map(|(i, x0)| xs[i + 1..].iter().map(|x1| (*x0, *x1)))
        })
    }
}

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let map = Map::try_from(input)?;

    let mut positions = gxhash::HashSet::new();

    for (a, b) in map.antenna_pairs() {
        let d = a.subtract(b);
        if let Some(p) = a.try_add(d, map.max_pos()) {
            positions.insert(p);
        }
        if let Some(p) = b.try_subtract(d, map.max_pos()) {
            positions.insert(p);
        }
    }

    Ok(positions.len().to_string())
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let map = Map::try_from(input)?;

    let mut positions = gxhash::HashSet::new();

    for (a, b) in map.antenna_pairs() {
        positions.insert(a);
        positions.insert(b);

        let mut a = a;
        let mut b = b;

        let d = a.subtract(b);
        while let Some(p) = a.try_add(d, map.max_pos()) {
            positions.insert(p);
            a = p;
        }
        while let Some(p) = b.try_subtract(d, map.max_pos()) {
            positions.insert(p);
            b = p;
        }
    }

    Ok(positions.len().to_string())
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
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        "14"
    }

    test_solution! {
        part_2 part_two_default_case
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        "34"
    }
}
