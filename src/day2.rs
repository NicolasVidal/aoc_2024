use std::cmp::Ordering;
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Unknown,
    Increasing,
    Decreasing,
}

fn invalid_report(line: &str, mut drop_index: Option<usize>) -> bool {
    let mut orientation = Orientation::Unknown;
    let mut tokens = line.split_whitespace();
    let mut index = 0;

    if let Some(dropped_index) = drop_index {
        if dropped_index == 0 {
            tokens.next();
            drop_index = None;
        }
    }
    let mut first = i32::from_str(tokens.next().unwrap()).unwrap();
    for token in tokens {
        index += 1;
        let second = i32::from_str(token).unwrap();

        if let Some(dropped_index) = drop_index {
            if index == dropped_index {
                drop_index = None;
                continue;
            }
        }

        let diff = first.abs_diff(second);
        if !(1..=3).contains(&diff) {
            return true;
        }
        match (orientation, first.cmp(&second)) {
            (_, Ordering::Equal) => {
                return true;
            }
            (Orientation::Unknown, Ordering::Less) => {
                orientation = Orientation::Increasing;
            }
            (Orientation::Unknown, Ordering::Greater) => {
                orientation = Orientation::Decreasing;
            }
            (Orientation::Increasing, Ordering::Greater) => {
                return true;
            }
            (Orientation::Decreasing, Ordering::Less) => {
                return true;
            }
            _ => {}
        };

        first = second;
    }
    false
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut safe_reports = 0u32;
    'outer: for line in input.lines() {
        if invalid_report(line, None) {
            continue 'outer;
        }
        safe_reports += 1;
    }

    safe_reports
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut safe_reports = 0u32;
    for line in input.lines() {
        let count = line.split_whitespace().count();
        let mut found = false;
        for i in 0..count {
            if !invalid_report(line, Some(i)) {
                found = true;
                break;
            }
        }
        if found {
            safe_reports += 1;
        }
    }

    safe_reports
}