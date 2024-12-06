# Advent of Code 2024

To run:

```sh
cargo make install

# see help
adventofcode-2024 --help

# e.g. part two for day 1
cat $puzzle_input | adventofcode-2024 day01 two
```
## Benchmarks

Some puzzles (the hard ones!) have benchmarks setup. Look at the [benchmark file](./benches/adventofcode-benchmark.rs) to see which.

These benchmarks will require loading puzzle input from disk. In general puzzle input should be stored in `.input/day*.txt`.

Benchmarks can be run with:

```sh
cargo make bench
```

Note that they will take quite a while to run!
