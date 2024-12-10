use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_string = include_str!("../input/2024/day10.txt");

    c.bench_function("day10 p1", |b| {
        b.iter(|| rust_nrjv_aoc_2024::day10::part1(input_string))
    });
    c.bench_function("day10 p2", |b| {
        b.iter(|| {
            rust_nrjv_aoc_2024::day10::part2(black_box(input_string));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
