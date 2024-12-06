use std::{fs::File, io::BufReader, time::Duration};

use adventofcode_2024::day_06;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 06");

    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(50));

    group.bench_function("day06 part 2", |b| {
        b.iter_batched(
            || BufReader::new(File::open(".input/day6.txt").expect("can open day6.txt")),
            day_06::part_2,
            criterion::BatchSize::PerIteration,
        )
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
