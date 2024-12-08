use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_string = include_str!("../input/2024/day8.txt");

    c.bench_function("day8 p1", |b| {
        b.iter(|| rust_nrjv_aoc_2024::day8::part1(input_string))
    });
    c.bench_function("day8 p2", |b| {
        b.iter(|| {
            rust_nrjv_aoc_2024::day8::part2(black_box(input_string));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
