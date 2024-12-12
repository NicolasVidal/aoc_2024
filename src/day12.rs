
#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let line_length = memchr::memchr(b'\n', bytes).unwrap() + 1;
    let bytes_len = bytes.len();

    let line_count = if bytes[bytes_len - 1] == b'\n' {
        bytes_len / line_length
    } else {
        (bytes_len + 1) / line_length
    };

    let mut visited = vec![false; bytes_len];
    let mut total_count = 0u64;

    for line in 0..line_count {
        let line_start = line * line_length;
        for col in 0..(line_length - 1) {
            let index = line_start + col;

            if visited[index] {
                continue;
            }

            let value = bytes[index];

            let mut stack = vec![index];
            let mut zone_count = 0u32;
            let mut fences_count = 0u32;

            while let Some(index) = stack.pop() {
                if visited[index] {
                    continue;
                }
                let line = index / line_length;
                let col = index % line_length;
                visited[index] = true;
                zone_count += 1;

                let neighbours = [
                    if col > 0 { Some(index - 1) } else { None },
                    if col + 1 < line_length - 1 { Some(index + 1) } else { None },
                    if line > 0 { Some(index - line_length) } else { None },
                    if line + 1 < line_count { Some(index + line_length) } else { None },
                ];

                let mut fences = 4u32;
                for &neighbour in neighbours.iter().flatten() {
                    let neighbour_value = bytes[neighbour];
                    if neighbour_value == value {
                        fences -= 1;
                        stack.push(neighbour);
                    }
                }
                fences_count += fences;
            }

            total_count += zone_count as u64 * fences_count as u64;
        }
    }

    total_count
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let line_length = memchr::memchr(b'\n', bytes).unwrap() + 1;
    let bytes_len = bytes.len();

    let line_count = if bytes[bytes_len - 1] == b'\n' {
        bytes_len / line_length
    } else {
        (bytes_len + 1) / line_length
    };

    let mut visited = vec![false; bytes_len];
    let mut total_count = 0u64;

    for line in 0..line_count {
        let line_start = line * line_length;
        for col in 0..(line_length - 1) {
            let index = line_start + col;

            if visited[index] {
                continue;
            }

            let value = bytes[index];

            let mut stack = vec![index];
            let mut zone_count = 0u32;
            let mut fences_count = 0u32;

            let mut top_sides = vec![false; bytes_len];
            let mut bottom_sides = vec![false; bytes_len];
            let mut left_sides = vec![false; bytes_len];
            let mut right_sides = vec![false; bytes_len];

            while let Some(index) = stack.pop() {
                if visited[index] {
                    continue;
                }
                let line = index / line_length;
                let col = index % line_length;
                visited[index] = true;
                zone_count += 1;

                let neighbours = [
                    (Side::Left, if col > 0 { Some(index - 1) } else { None }),
                    (Side::Right, if col + 1 < line_length - 1 { Some(index + 1) } else { None }),
                    (Side::Top, if line > 0 { Some(index - line_length) } else { None }),
                    (Side::Bottom, if line + 1 < line_count { Some(index + line_length) } else { None }),
                ];

                for (side, neighbour) in neighbours.iter() {
                    if let Some(neighbour) = neighbour {
                        let neighbour_value = bytes[*neighbour];
                        if neighbour_value == value {
                            stack.push(*neighbour);
                        } else {
                            match side {
                                Side::Top => {
                                    top_sides[index] = true;
                                }
                                Side::Bottom => {
                                    bottom_sides[index] = true;
                                }
                                Side::Left => {
                                    left_sides[index] = true;
                                }
                                Side::Right => {
                                    right_sides[index] = true;
                                }
                            }
                        }
                    } else {
                        match side {
                            Side::Top => {
                                top_sides[index] = true;
                            }
                            Side::Bottom => {
                                bottom_sides[index] = true;
                            }
                            Side::Left => {
                                left_sides[index] = true;
                            }
                            Side::Right => {
                                right_sides[index] = true;
                            }
                        }
                    }
                }
            }

            for line in 0..line_count {
                let mut top = false;
                let mut bottom = false;
                for col in 0..(line_length - 1) {
                    let index = line * line_length + col;
                    if !top_sides[index] {
                        top = false;
                    } else if !top {
                        top = true;
                        fences_count += 1;
                    }
                    if !bottom_sides[index] {
                        bottom = false;
                    } else if !bottom {
                        bottom = true;
                        fences_count += 1;
                    }
                }
            }

            for col in 0..(line_length - 1) {
                let mut left = false;
                let mut right = false;
                for line in 0..line_count {
                    let index = line * line_length + col;
                    if !left_sides[index] {
                        left = false;
                    } else if !left {
                        left = true;
                        fences_count += 1;
                    }
                    if !right_sides[index] {
                        right = false;
                    } else if !right {
                        right = true;
                        fences_count += 1;
                    }
                }
            }

            total_count += zone_count as u64 * fences_count as u64;
        }
    }

    total_count
}