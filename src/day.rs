use std::io::{stdin, BufRead, StdinLock};

pub type PartFn<I> = fn(input: I) -> anyhow::Result<String>;

pub struct Day<I: BufRead> {
    part_1_fn: PartFn<I>,
    part_2_fn: Option<PartFn<I>>,
}

impl<I: BufRead> Day<I> {
    pub fn part_1(part_1_fn: PartFn<I>) -> Self {
        Self {
            part_1_fn,
            part_2_fn: None,
        }
    }

    pub fn part_2(mut self, part_2_fn: PartFn<I>) -> Self {
        self.part_2_fn.replace(part_2_fn);
        self
    }
}

impl Day<StdinLock<'_>> {
    pub fn solve_part_1(&self) -> anyhow::Result<()> {
        Self::solve(self.part_1_fn)
    }

    pub fn solve_part_2(&self) -> anyhow::Result<()> {
        self.part_2_fn
            .ok_or(anyhow::anyhow!("part 2 not defined"))
            .and_then(Self::solve)
    }

    fn solve(part_fn: PartFn<StdinLock<'_>>) -> anyhow::Result<()> {
        let input = stdin();
        let handle = input.lock();

        (part_fn)(handle).map(|output| {
            println!("{output}");
        })
    }
}
