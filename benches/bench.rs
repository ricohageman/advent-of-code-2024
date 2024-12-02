use advent_of_code_2024::day1;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT_DAY_1: &str = include_str!("../examples/2024/day1.txt");

pub fn day1(c: &mut Criterion) {
    c.bench_function("day1 generate", |b| {
        b.iter(|| day1::input_generator(INPUT_DAY_1))
    });

    let input = day1::input_generator(INPUT_DAY_1);
    c.bench_function("day1 part1", |b| b.iter(|| day1::part1(&input)));
    c.bench_function("day1 part2", |b| b.iter(|| day1::part2(&input)));
}

criterion_group!(benches, day1);
criterion_main!(benches);
