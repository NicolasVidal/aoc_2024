use std::iter::Iterator;

#[inline(always)]
pub fn fast_u32_parsing(line: &[u8]) -> u32 {
    let mut result = 0u32;

    const TO_REMOVE: u32 = (b'0' as u32) * 10000
    + (b'0' as u32) * 1000
    + (b'0' as u32) * 100
    + (b'0' as u32) * 10
    + (b'0' as u32);

    result += line[0] as u32 * 10000;
    result += line[1] as u32 * 1000;
    result += line[2] as u32 * 100;
    result += line[3] as u32 * 10;
    result += line[4] as u32;

    result - TO_REMOVE
}

#[inline(always)]
pub fn fast_line_parsing(line: &[u8]) -> (u32, u32) {
    let left = &line[0..5];
    let right = &line[8..13];

    let left = fast_u32_parsing(left);
    let right = fast_u32_parsing(right);

    (left, right)
}

#[inline(always)]
fn fill_columns(input: &str) -> (heapless::Vec<u32, 1024>, heapless::Vec<u32, 1024>) {
    let mut left: heapless::Vec<u32, 1024> = heapless::Vec::new();
    let mut right: heapless::Vec<u32, 1024> = heapless::Vec::new();

    let mut offset = 0usize;

    let input_bytes = input.as_bytes();
    let full_bytes_count = input_bytes.len();

    loop {
        if offset + 12 >= full_bytes_count {
            break;
        }

        let line_slice = &input_bytes[offset..(offset + 13)];

        let (left_value, right_value) = fast_line_parsing(line_slice);
        left.push(left_value).unwrap();
        right.push(right_value).unwrap();
        offset += 13;

        while offset < full_bytes_count && (input_bytes[offset] == b'\n' || input_bytes[offset] == b'\r') {
            offset += 1;
        }
    }
    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = fill_columns(input);

    left.sort_unstable();
    right.sort_unstable();

    let mut total = 0u32;
    for (a, b) in left.iter().zip(right.iter()) {
        total += a.abs_diff(*b);
    }
    total
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (left, right) = fill_columns(input);

    let mut count = [0u8; 100_000];

    for num in right {
        count[num as usize] += 1;
    }

    let mut similarity_score = 0usize;

    for num in left {
        similarity_score += num as usize * count[num as usize] as usize;
    }

    similarity_score as u32
}