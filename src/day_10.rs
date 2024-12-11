use std::io::BufRead;

use gxhash::{HashSet, HashSetExt};

use crate::{
    day::Day,
    grid::{Grid2D, Vec2},
    parser::Parser,
};

pub fn part_1<I: BufRead>(input: I) -> anyhow::Result<String> {
    let grid = Grid2D::from(Parser::from(input).chars());

    let mut stack: Vec<_> = grid
        .iter()
        .filter_map(|(p, c)| if *c == '0' { Some((p, *c)) } else { None })
        .collect();

    let directions: [Vec2<isize>; 4] = [Vec2(0, 1), Vec2(0, -1), Vec2(1, 0), Vec2(-1, 0)];

    let mut trails = vec![];

    while !stack.is_empty() {
        // SAFTEY: stack length check in while loop
        let (p, c) = unsafe { stack.pop().unwrap_unchecked() };

        if c == '0' {
            trails.push(HashSet::new());
        }

        for d in &directions {
            let cand = p
                .try_add(*d, grid.max())
                .and_then(|p0| grid.get(p0).map(|c0| (p0, c, *c0)));
            match cand {
                Some((p, '0', '1')) => stack.push((p, '1')),
                Some((p, '1', '2')) => stack.push((p, '2')),
                Some((p, '2', '3')) => stack.push((p, '3')),
                Some((p, '3', '4')) => stack.push((p, '4')),
                Some((p, '4', '5')) => stack.push((p, '5')),
                Some((p, '5', '6')) => stack.push((p, '6')),
                Some((p, '6', '7')) => stack.push((p, '7')),
                Some((p, '7', '8')) => stack.push((p, '8')),
                Some((p, '8', '9')) => {
                    // SAFETY: always push to trails when a new 0 position is popped from stack.
                    unsafe { trails.last_mut().unwrap_unchecked() }.insert(p);
                }
                _ => {}
            }
        }
    }

    Ok(trails.iter().fold(0, |acc, s| acc + s.len()).to_string())
}

pub fn part_2<I: BufRead>(input: I) -> anyhow::Result<String> {
    let grid = Grid2D::from(Parser::from(input).chars());

    let mut stack: Vec<_> = grid
        .iter()
        .filter_map(|(p, c)| if *c == '0' { Some((p, *c)) } else { None })
        .collect();

    let directions: [Vec2<isize>; 4] = [Vec2(0, 1), Vec2(0, -1), Vec2(1, 0), Vec2(-1, 0)];

    let mut trails = vec![];

    while !stack.is_empty() {
        // SAFTEY: stack length check in while loop
        let (p, c) = unsafe { stack.pop().unwrap_unchecked() };

        if c == '0' {
            trails.push(vec![]);
        }

        for d in &directions {
            let cand = p
                .try_add(*d, grid.max())
                .and_then(|p0| grid.get(p0).map(|c0| (p0, c, *c0)));
            match cand {
                Some((p, '0', '1')) => stack.push((p, '1')),
                Some((p, '1', '2')) => stack.push((p, '2')),
                Some((p, '2', '3')) => stack.push((p, '3')),
                Some((p, '3', '4')) => stack.push((p, '4')),
                Some((p, '4', '5')) => stack.push((p, '5')),
                Some((p, '5', '6')) => stack.push((p, '6')),
                Some((p, '6', '7')) => stack.push((p, '7')),
                Some((p, '7', '8')) => stack.push((p, '8')),
                Some((p, '8', '9')) => {
                    // SAFETY: always push to trails when a new 0 position is popped from stack.
                    unsafe { trails.last_mut().unwrap_unchecked() }.push(p);
                }
                _ => {}
            }
        }
    }

    Ok(trails.iter().fold(0, |acc, s| acc + s.len()).to_string())
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
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "36"
    }

    test_solution! {
        part_2 part_two_default_case
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "81"
    }
}
