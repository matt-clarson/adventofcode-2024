use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

use adventofcode_2024::{day_06, day_11, test_util::StringBufRead};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 06");

    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(50));

    group.bench_function("part 2", |b| {
        b.iter_batched(
            || BufReader::new(File::open(".input/day6.txt").expect("can open day6.txt")),
            day_06::part_2,
            criterion::BatchSize::PerIteration,
        )
    });

    group.finish();

    let mut group = c.benchmark_group("day 11");

    group.bench_function("part 1", |b| {
        let mut s = String::new();
        File::open(".input/day11.txt")
            .expect("can open day11.txt")
            .read_to_string(&mut s)
            .expect("can read day11.txt");
        b.iter_batched(
            || StringBufRead::from(s.as_str()),
            day_11::part_1,
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
