use std::{
    fmt::Debug,
    io::{BufRead, Read},
};

use crate::day::Day;

#[derive(Copy, Clone, Debug)]
struct Vec2<I: Copy + Clone + Debug>(I, I);

impl Vec2<usize> {
    fn try_add(&self, d: Vec2<isize>, max: Vec2<usize>) -> Option<Vec2<usize>> {
        if self.0 == 0 && d.0 == -1
            || self.0 == max.0 && d.0 == 1
            || self.1 == 0 && d.1 == -1
            || self.1 == max.1 && d.1 == 1
        {
            None
        } else {
            Some(Vec2(
                (self.0 as isize + d.0) as usize,
                (self.1 as isize + d.1) as usize,
            ))
        }
    }
}

struct Crossword {
    grid: Vec<Vec<char>>,
}

enum Mas {
    Fwd,
    Bwd,
}

impl Crossword {
    fn try_from<R: Read>(value: R) -> anyhow::Result<Self> {
        let mut grid = vec![vec![]];
        for b in value.bytes() {
            match char::from(b?) {
                '\n' => grid.push(vec![]),
                // SAFTEY: grid is initialised with an empty child vec
                c => unsafe { grid.last_mut().unwrap_unchecked() }.push(c),
            }
        }
        Ok(Self { grid })
    }

    fn iter_xmas_start(&self) -> impl Iterator<Item = Vec2<usize>> + '_ {
        self.grid.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| if *c == 'X' { Some(Vec2(i, j)) } else { None })
        })
    }

    fn iter_mas_cross_start(&self) -> impl Iterator<Item = (Vec2<usize>, Mas)> + '_ {
        self.grid.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, c)| {
                if *c == 'M' {
                    Some((Vec2(i, j), Mas::Fwd))
                } else if *c == 'S' {
                    Some((Vec2(i, j), Mas::Bwd))
                } else {
                    None
                }
            })
        })
    }

    fn max(&self) -> Vec2<usize> {
        Vec2(self.grid.len(), self.grid[0].len())
    }

    fn possible_xmas_directions(
        &self,
        x_pos: Vec2<usize>,
    ) -> impl Iterator<Item = (Vec2<usize>, Vec2<isize>)> + '_ {
        let directions = [
            Vec2(0, 1),
            Vec2(1, 1),
            Vec2(1, 0),
            Vec2(1, -1),
            Vec2(0, -1),
            Vec2(-1, -1),
            Vec2(-1, 0),
            Vec2(-1, 1),
        ];

        directions
            .into_iter()
            .filter_map(move |d| self.try_get_next(x_pos, d, 'M').map(|p| (p, d)))
    }

    fn try_get_next(&self, pos: Vec2<usize>, d: Vec2<isize>, c: char) -> Option<Vec2<usize>> {
        pos.try_add(d, self.max())
            .and_then(|p| self.get(p).filter(|c0| *c0 == c).and(Some(p)))
    }

    fn get(&self, d: Vec2<usize>) -> Option<char> {
        self.grid.get(d.0).and_then(|row| row.get(d.1)).copied()
    }
}

fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let crossword = Crossword::try_from(input)?;

    let mut count = 0;
    for x_pos in crossword.iter_xmas_start() {
        for (m_pos, d) in crossword.possible_xmas_directions(x_pos) {
            let a_pos = if let Some(pos) = crossword.try_get_next(m_pos, d, 'A') {
                pos
            } else {
                continue;
            };
            if crossword.try_get_next(a_pos, d, 'S').is_some() {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}

fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let crossword = Crossword::try_from(input)?;

    let mut count = 0;

    let up_right: Vec2<isize> = Vec2(-1, 1);
    let down_left: Vec2<isize> = Vec2(1, -1);
    let down_right: Vec2<isize> = Vec2(1, 1);

    for (start_pos, dir) in crossword.iter_mas_cross_start() {
        let centre = if let Some(pos) = crossword.try_get_next(start_pos, down_right, 'A') {
            pos
        } else {
            continue;
        };

        let is_cross = match dir {
            Mas::Fwd => crossword
                .try_get_next(centre, down_right, 'S')
                .and_then(|_| {
                    crossword
                        .try_get_next(centre, up_right, 'M')
                        .and_then(|_| crossword.try_get_next(centre, down_left, 'S'))
                        .or_else(|| {
                            crossword
                                .try_get_next(centre, up_right, 'S')
                                .and_then(|_| crossword.try_get_next(centre, down_left, 'M'))
                        })
                })
                .is_some(),
            Mas::Bwd => crossword
                .try_get_next(centre, down_right, 'M')
                .and_then(|_| {
                    crossword
                        .try_get_next(centre, up_right, 'M')
                        .and_then(|_| crossword.try_get_next(centre, down_left, 'S'))
                        .or_else(|| {
                            crossword
                                .try_get_next(centre, up_right, 'S')
                                .and_then(|_| crossword.try_get_next(centre, down_left, 'M'))
                        })
                })
                .is_some(),
        };

        if is_cross {
            count += 1;
        }
    }

    Ok(count.to_string())
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
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        "18"
    }

    test_solution! {
        part_2 part_two_default_case
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        "9"
    }
}
