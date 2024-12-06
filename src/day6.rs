#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline(always)]
    fn cycle(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0u32;
    let input_bytes = input.as_bytes();
    let mut array = [0u8; 150 * 150];

    let line_length = memchr::memchr(b'\n', input_bytes).unwrap() + 1;
    let target_line_length = line_length - 1;

    let lines_count = (input_bytes.len()
        + (if input_bytes[input_bytes.len() - 1] == b'\n' {
        0
    } else {
        1
    })) / line_length;

    for l in 0..lines_count {
        let start_origin = l * line_length;
        let end_origin = start_origin + target_line_length;
        let target_start = l * target_line_length;
        let target_end = target_start + target_line_length;

        array[target_start..target_end].copy_from_slice(&input_bytes[start_origin..end_origin]);
    }

    let line_length = target_line_length;

    let start_position = memchr::memchr(b'^', &array).unwrap();
    let mut position = start_position;
    let mut current_direction = Direction::Up;
    let total_size = lines_count * line_length;

    loop {
        if array[position] != b'X' {
            total += 1;
            array[position] = b'X';
        }

        let target_position = match current_direction {
            Direction::Up => {
                if position < line_length {
                    break;
                }
                position - line_length
            }
            Direction::Down => {
                if position >= total_size - line_length {
                    break;
                }
                position + line_length
            }
            Direction::Left => {
                if position % line_length == 0 {
                    break;
                }
                position - 1
            }
            Direction::Right => {
                if position % line_length == line_length - 1 {
                    break;
                }
                position + 1
            }
        };
        if array[target_position] == b'#' {
            current_direction = current_direction.cycle();
        } else {
            position = target_position;
        }
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let mut total = 0u32;
    let input_bytes = input.as_bytes();
    let mut array = [0u8; 150 * 150];

    let line_length = memchr::memchr(b'\n', input_bytes).unwrap() + 1;
    let target_line_length = line_length - 1;

    let lines_count = (input_bytes.len()
        + (if input_bytes[input_bytes.len() - 1] == b'\n' {
        0
    } else {
        1
    })) / line_length;

    for l in 0..lines_count {
        let start_origin = l * line_length;
        let end_origin = start_origin + target_line_length;
        let target_start = l * target_line_length;
        let target_end = target_start + target_line_length;

        array[target_start..target_end].copy_from_slice(&input_bytes[start_origin..end_origin]);
    }

    let line_length = target_line_length;

    let start_position = memchr::memchr(b'^', &array).unwrap();
    let mut position = start_position;
    let mut current_direction = Direction::Up;
    let total_size = lines_count * line_length;

    loop {
        if array[position] != b'X' {
            array[position] = b'X';

            if position != start_position {
                let mut arr_copy = array.clone();
                arr_copy[position] = b'#';
                let mut walked_array = [0u8; 150 * 150];

                let mut current_direction = Direction::Up;
                let mut position = start_position;
                loop {
                    let direction_mask = match current_direction {
                        Direction::Up => 0b0000_0001,
                        Direction::Down => 0b0000_0010,
                        Direction::Left => 0b0000_0100,
                        Direction::Right => 0b0000_1000,
                    };

                    if walked_array[position] & direction_mask == 1 {
                        total += 1;
                        break;
                    }
                    walked_array[position] |= direction_mask;

                    let target_position = match current_direction {
                        Direction::Up => {
                            if position < line_length {
                                break;
                            }
                            position - line_length
                        }
                        Direction::Down => {
                            if position >= total_size - line_length {
                                break;
                            }
                            position + line_length
                        }
                        Direction::Left => {
                            if position % line_length == 0 {
                                break;
                            }
                            position - 1
                        }
                        Direction::Right => {
                            if position % line_length == line_length - 1 {
                                break;
                            }
                            position + 1
                        }
                    };
                    if arr_copy[target_position] == b'#' {
                        current_direction = current_direction.cycle();
                    } else {
                        position = target_position;
                    }
                }
            }
        }

        let target_position = match current_direction {
            Direction::Up => {
                if position < line_length {
                    break;
                }
                position - line_length
            }
            Direction::Down => {
                if position >= total_size - line_length {
                    break;
                }
                position + line_length
            }
            Direction::Left => {
                if position % line_length == 0 {
                    break;
                }
                position - 1
            }
            Direction::Right => {
                if position % line_length == line_length - 1 {
                    break;
                }
                position + 1
            }
        };
        if array[target_position] == b'#' {
            current_direction = current_direction.cycle();
        } else {
            position = target_position;
        }
    }

    total
}
