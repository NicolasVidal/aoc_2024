use rustc_hash::FxHashMap;
use std::iter::Iterator;
use std::str::FromStr;

#[inline(always)]
fn parse_two_digit_number(byte1: u8, byte2: u8) -> u8 {
    const OFFSET: u8 = b'0';
    (byte1 - OFFSET) * 10 + byte2 - OFFSET
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0u32;

    let mut order_map = FxHashMap::default();

    let input_bytes = input.as_bytes();

    let mut count_lines = 0;
    for i in 0.. {
        let start_offset = i * 6;
        count_lines += 1;
        if input_bytes[start_offset] == b'\n' {
            break;
        }

        let part1 =
            parse_two_digit_number(input_bytes[start_offset], input_bytes[start_offset + 1]);
        let part2 =
            parse_two_digit_number(input_bytes[start_offset + 3], input_bytes[start_offset + 4]);

        order_map.entry(part1).or_insert(Vec::new()).push(part2);
    }

    let mut line_iterator = input.lines().skip(count_lines);

    'lines: while let Some(line) = line_iterator.next() {
        let numbers = line
            .split(',')
            .map(|elt| u8::from_str(elt).unwrap())
            .collect::<Vec<_>>();

        for i in 0..numbers.len() {
            let should_be_before = numbers[i];
            for should_be_after in numbers.iter().skip(i + 1) {
                if let Some(should_not_be_befores) = order_map.get(&should_be_after) {
                    if should_not_be_befores.contains(&should_be_before) {
                        continue 'lines;
                    }
                }
            }
        }

        total += numbers[numbers.len() / 2] as u32
    }

    total
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let mut total = 0u32;

    let mut order_map = FxHashMap::default();

    let input_bytes = input.as_bytes();

    let mut count_lines = 0;
    for i in 0.. {
        let start_offset = i * 6;
        count_lines += 1;
        if input_bytes[start_offset] == b'\n' {
            break;
        }

        let part1 =
            parse_two_digit_number(input_bytes[start_offset], input_bytes[start_offset + 1]);
        let part2 =
            parse_two_digit_number(input_bytes[start_offset + 3], input_bytes[start_offset + 4]);

        order_map.entry(part1).or_insert(Vec::new()).push(part2);
    }

    let mut line_iterator = input.lines().skip(count_lines);

    while let Some(line) = line_iterator.next() {
        let mut numbers = line
            .split(',')
            .map(|elt| u8::from_str(elt).unwrap())
            .collect::<Vec<_>>();

        let mut found = false;

        'bimbo: for i in 0..numbers.len() {
            let should_be_before = numbers[i];
            for should_be_after in numbers.iter().skip(i + 1) {
                if let Some(should_not_be_befores) = order_map.get(&should_be_after) {
                    if should_not_be_befores.contains(&should_be_before) {
                        found = true;
                        break 'bimbo;
                    }
                }
            }
        }

        if found {
            numbers.sort_unstable_by(|a, b| match (order_map.get(a), order_map.get(b)) {
                (Some(a_after), Some(b_after)) => {
                    match (a_after.contains(b), b_after.contains(a)) {
                        (true, true) => panic!("Inconsistant contraints"),
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        (false, false) => std::cmp::Ordering::Equal,
                    }
                }
                (Some(a_after), None) => {
                    if a_after.contains(b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                (None, Some(b_after)) => {
                    if b_after.contains(a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                (None, None) => std::cmp::Ordering::Equal,
            });
            total += numbers[numbers.len() / 2] as u32
        }
    }

    total
}
