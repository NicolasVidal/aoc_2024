use std::iter::Iterator;
use std::str::FromStr;

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut left: heapless::Vec<u32, 1024> = heapless::Vec::new();
    let mut right: heapless::Vec<u32, 1024> = heapless::Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        left.push(u32::from_str(tokens.next().unwrap()).unwrap()).unwrap();
        right.push(u32::from_str(tokens.next().unwrap()).unwrap()).unwrap();
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut total = 0u32;
    for (a, b) in left.iter().zip(right.iter()) {
        total += a.abs_diff(*b);
    }
    total
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut left: heapless::Vec<usize, 1024> = heapless::Vec::new();
    let mut right: heapless::Vec<usize, 1024> = heapless::Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        left.push(usize::from_str(tokens.next().unwrap()).unwrap()).unwrap();
        right.push(usize::from_str(tokens.next().unwrap()).unwrap()).unwrap();
    }

    let mut count = [0usize; 100_000];

    for num in right {
        count[num] += 1;
    }

    let mut similarity_score = 0usize;

    for num in left {
        similarity_score += num * count[num];
    }

    similarity_score as u32
}