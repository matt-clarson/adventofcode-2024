use adventofcode_2024::*;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "CLI for solving the Advent of Code 2024 puzzles."
)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    day: Day,
}

gen::days! {
    Day01: day_01::solution(),
    Day02: day_02::solution(),
    Day03: day_03::solution(),
    Day04: day_04::solution(),
    Day05: day_05::solution(),
    Day06: day_06::solution(),
    Day07: day_07::solution(),
    Day08: day_08::solution(),
    Day09: day_09::solution(),
    Day10: day_10::solution(),
    Day11: day_11::solution()
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.day.solve() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

mod gen {
    #[macro_export]
    macro_rules! days{
    ($($name:ident: $day:expr),+) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
        enum Part {
            /// Solve part one of the puzzle.
            One,
            /// Solve part two of the puzzle.
            Two,
        }
        #[derive(Subcommand)]
        enum Day {
            $(
                $name {
                    /// Which part of the puzzle to solve.
                    part: Part,
                },
            )+
        }

        impl Day {
            fn solve(&self) -> anyhow::Result<()> {
                match self {
                    $(
                        Self::$name { part: Part::One } => $day.solve_part_1(),
                        Self::$name { part: Part::Two } => $day.solve_part_2(),
                    )+
                }
            }
        }
    }
}

    pub use days;
}
