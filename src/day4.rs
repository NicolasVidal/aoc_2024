#[inline(always)]
fn is_match_last(
    first_index: usize,
    second_index: usize,
    third_index: usize,
    bytes: &[u8],
) -> bool {
    matches!(
        [bytes[first_index], bytes[second_index], bytes[third_index]],
        [b'M', b'A', b'S']
    )
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0u32;
    let bytes = input.as_bytes();

    let line_length = memchr::memchr(b'\n', bytes).unwrap();
    let real_length = line_length + 1;

    let lines_count = (bytes.len()
        + (if bytes[bytes.len() - 1] == b'\n' {
            0
        } else {
            1
        })) / real_length;

    for line in 0..lines_count {
        let line_start = line * real_length;
        let line_end = line_start + line_length;
        let line_bytes = &bytes[line_start..line_end];

        for x_pos in memchr::memchr_iter(b'X', line_bytes) {
            if x_pos + 3 < line_length {
                if is_match_last(x_pos + 1, x_pos + 2, x_pos + 3, line_bytes) {
                    total += 1;
                }
            }

            if x_pos >= 3 {
                if is_match_last(x_pos - 1, x_pos - 2, x_pos - 3, line_bytes) {
                    total += 1;
                }
            }

            if line + 3 < lines_count {
                let second_index = real_length * (line + 1) + x_pos;
                let third_index = real_length * (line + 2) + x_pos;
                let fourth_index = real_length * (line + 3) + x_pos;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }

            if line >= 3 {
                let second_index = real_length * (line - 1) + x_pos;
                let third_index = real_length * (line - 2) + x_pos;
                let fourth_index = real_length * (line - 3) + x_pos;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }

            if line + 3 < lines_count && x_pos + 3 < line_length {
                let second_index = real_length * (line + 1) + x_pos + 1;
                let third_index = real_length * (line + 2) + x_pos + 2;
                let fourth_index = real_length * (line + 3) + x_pos + 3;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }

            if line >= 3 && x_pos >= 3 {
                let second_index = (line - 1) * real_length + x_pos - 1;
                let third_index = (line - 2) * real_length + x_pos - 2;
                let fourth_index = (line - 3) * real_length + x_pos - 3;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }

            if line + 3 < lines_count && x_pos >= 3 {
                let second_index = real_length * (line + 1) + x_pos - 1;
                let third_index = real_length * (line + 2) + x_pos - 2;
                let fourth_index = real_length * (line + 3) + x_pos - 3;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }

            if line >= 3 && x_pos + 3 < line_length {
                let second_index = real_length * (line - 1) + x_pos + 1;
                let third_index = real_length * (line - 2) + x_pos + 2;
                let fourth_index = real_length * (line - 3) + x_pos + 3;

                if is_match_last(second_index, third_index, fourth_index, bytes) {
                    total += 1;
                }
            }
        }
    }

    total
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let mut total = 0u32;
    let bytes = input.as_bytes();

    let line_length = memchr::memchr(b'\n', bytes).unwrap();
    let real_length = line_length + 1;

    let lines_count = (bytes.len()
        + (if bytes[bytes.len() - 1] == b'\n' {
        0
    } else {
        1
    })) / real_length;

    for line in 1..(lines_count - 1) {
        let line_start = line * real_length;
        let line_end = line_start + line_length;
        let line_bytes = &bytes[line_start..(line_end - 1)];

        for x_pos in memchr::memchr_iter(b'A', line_bytes) {
            let top_left = real_length * (line - 1) + x_pos - 1;
            let top_right = real_length * (line - 1) + x_pos + 1;
            let bottom_left = real_length * (line + 1) + x_pos - 1;
            let bottom_right = real_length * (line + 1) + x_pos + 1;

            if bytes[top_left] == b'M'
                && bytes[top_right] == b'M'
                && bytes[bottom_left] == b'S'
                && bytes[bottom_right] == b'S'
                || bytes[top_left] == b'S'
                    && bytes[top_right] == b'S'
                    && bytes[bottom_left] == b'M'
                    && bytes[bottom_right] == b'M'
                || bytes[top_left] == b'M'
                    && bytes[top_right] == b'S'
                    && bytes[bottom_left] == b'M'
                    && bytes[bottom_right] == b'S'
                || bytes[top_left] == b'S'
                    && bytes[top_right] == b'M'
                    && bytes[bottom_left] == b'S'
                    && bytes[bottom_right] == b'M'
            {
                total += 1;
            }
        }
    }

    total
}
