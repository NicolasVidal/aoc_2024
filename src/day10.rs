use rustc_hash::{FxBuildHasher, FxHashSet};

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let iterator = bytes.iter();

    let mut v = Vec::new();

    let mut line_length = None;
    for (i, &b) in iterator.enumerate() {
        if b == b'\n' {
            if line_length.is_none() {
                line_length = Some(i);
            }
            continue;
        }

        v.push(b - b'0');
    }

    let line_length = line_length.unwrap();

    let line_count = if bytes[bytes.len() - 1] == b'\n' {
        v.len() / line_length
    } else {
        (v.len() + 1) / line_length
    };

    let mut stack = Vec::new();
    let mut total = 0u64;
    let mut hs = FxHashSet::default();
    for row in 0..line_count {
        for col in 0..line_length {
            if v[row * line_length + col] != 0 {
                continue;
            }

            stack.clear();
            hs.clear();

            stack.push((row, col));

            while let Some((row, col)) = stack.pop() {
                let v_index = row * line_length + col;
                let v_value = v[v_index];

                if v_value == 9 {
                    hs.insert(v_index);
                    continue;
                }

                if row > 0 {
                    let up_index = v_index - line_length;
                    if v[up_index] == v_value + 1 {
                        stack.push((row - 1, col));
                    }
                }

                if row + 1 < line_count {
                    let down_index = v_index + line_length;
                    if v[down_index] == v_value + 1 {
                        stack.push((row + 1, col));
                    }
                }

                if col > 0 {
                    let left_index = v_index - 1;
                    if v[left_index] == v_value + 1 {
                        stack.push((row, col - 1));
                    }
                }

                if col + 1 < line_length {
                    let right_index = v_index + 1;
                    if v[right_index] == v_value + 1 {
                        stack.push((row, col + 1));
                    }
                }
            }

            total += hs.len() as u64;
        }
    }

    total
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let iterator = bytes.iter();

    let mut v = Vec::with_capacity(3600);

    let mut line_length = None;
    for (i, &b) in iterator.enumerate() {
        if b == b'\n' {
            if line_length.is_none() {
                line_length = Some(i);
            }
            continue;
        }

        v.push(b - b'0');
    }

    let line_length = line_length.unwrap();

    let line_count = if bytes[bytes.len() - 1] == b'\n' {
        v.len() / line_length
    } else {
        (v.len() + 1) / line_length
    };

    let mut stack = Vec::new();
    let mut total = 0u64;
    for row in 0..line_count {
        for col in 0..line_length {
            if v[row * line_length + col] != 0 {
                continue;
            }

            stack.clear();

            stack.push((row, col));

            while let Some((row, col)) = stack.pop() {
                let v_index = row * line_length + col;
                let v_value = v[v_index];

                if v_value == 9 {
                    total += 1;
                    continue;
                }

                if row > 0 {
                    let up_index = v_index - line_length;
                    if v[up_index] == v_value + 1 {
                        stack.push((row - 1, col));
                    }
                }

                if row + 1 < line_count {
                    let down_index = v_index + line_length;
                    if v[down_index] == v_value + 1 {
                        stack.push((row + 1, col));
                    }
                }

                if col > 0 {
                    let left_index = v_index - 1;
                    if v[left_index] == v_value + 1 {
                        stack.push((row, col - 1));
                    }
                }

                if col + 1 < line_length {
                    let right_index = v_index + 1;
                    if v[right_index] == v_value + 1 {
                        stack.push((row, col + 1));
                    }
                }
            }
        }
    }

    total
}
