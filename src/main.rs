mod day_01;
mod parser;
mod problem;
#[cfg(test)]
mod test_util;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "CLI for soliving the Advent of Code 2024 puzzles."
)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    problem: Problem,
}

gen::problems! {
    Day01: day_01::solution()
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.problem.solve() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

mod gen {
    #[macro_export]
    macro_rules! problems{
    ($($name:ident: $problem:expr),+) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
        enum Part {
            /// Solve part one of the puzzle.
            One,
            /// Solve part two of the puzzle.
            Two,
        }
        #[derive(Subcommand)]
        enum Problem {
            $(
                /// Reads puzzle input from STDIN.
                $name {
                    /// Which part of the puzzle to solve.
                    part: Part,
                }
            )+
        }

        impl Problem {
            fn solve(&self) -> anyhow::Result<()> {
                match self {
                    $(
                        Self::$name { part: Part::One } => $problem.solve_part_1(),
                        Self::$name { part: Part::Two } => $problem.solve_part_2(),
                    )+
                }
            }
        }
    }
}

    pub use problems;
}
