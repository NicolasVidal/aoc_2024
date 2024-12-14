use std::hint::unreachable_unchecked;

#[inline(always)]
unsafe fn handle_machine_part1(ptr: *const u8, original_idx: &mut usize, max_len: usize) -> u32 {
    let ptr = ptr.add(*original_idx);
    let max_len = max_len - *original_idx;

    let a_x = (*ptr.add(11) - b'0') * 10 + (*ptr.add(12) - b'0');
    let a_y = (*ptr.add(17) - b'0') * 10 + (*ptr.add(18) - b'0');

    let b_x = (*ptr.add(32) - b'0') * 10 + (*ptr.add(33) - b'0');
    let b_y = (*ptr.add(38) - b'0') * 10 + (*ptr.add(39) - b'0');

    let mut t_x = (*ptr.add(50) - b'0') as u64;

    let mut idx = 51;

    while *ptr.add(idx) != b',' {
        t_x = t_x * 10 + (*ptr.add(idx) - b'0') as u64;
        idx += 1;
    }

    idx += 4;

    let mut t_y = (*ptr.add(idx) - b'0') as u64;

    idx += 1;

    while idx < max_len && *ptr.add(idx) != b'\n' {
        t_y = t_y * 10 + (*ptr.add(idx) - b'0') as u64;
        idx += 1;
    }

    *original_idx = *original_idx + idx + 2;

    let d = a_x as i32 * b_y as i32 - a_y as i32 * b_x as i32;
    if d == 0 {
        return 0;
    }

    let top_a = t_x as i32 * b_y as i32 - t_y as i32 * b_x as i32;
    let top_b = t_y as i32 * a_x as i32 - t_x as i32 * a_y as i32;

    if top_a % d != 0 || top_b % d != 0 {
        return 0;
    }

    let a = top_a / d;
    let b = top_b / d;

    if a < 0 || b < 0 {
        return 0;
    }

    (3 * a + b) as u32
}

#[inline(always)]
unsafe fn handle_machine_part2(ptr: *const u8, original_idx: &mut usize, max_len: usize) -> u64 {
    let ptr = ptr.add(*original_idx);
    let max_len = max_len - *original_idx;

    let a_x = (*ptr.add(11) - b'0') * 10 + (*ptr.add(12) - b'0');
    let a_y = (*ptr.add(17) - b'0') * 10 + (*ptr.add(18) - b'0');

    let b_x = (*ptr.add(32) - b'0') * 10 + (*ptr.add(33) - b'0');
    let b_y = (*ptr.add(38) - b'0') * 10 + (*ptr.add(39) - b'0');

    let mut t_x = (*ptr.add(50) - b'0') as u64;

    let mut idx = 51;

    while *ptr.add(idx) != b',' {
        t_x = t_x * 10 + (*ptr.add(idx) - b'0') as u64;
        idx += 1;
    }

    idx += 4;

    let mut t_y = (*ptr.add(idx) - b'0') as u64;

    idx += 1;

    while idx < max_len && *ptr.add(idx) != b'\n' {
        t_y = t_y * 10 + (*ptr.add(idx) - b'0') as u64;
        idx += 1;
    }

    *original_idx = *original_idx + idx + 2;

    t_x += 10000000000000;
    t_y += 10000000000000;

    let d = a_x as i64 * b_y as i64 - a_y as i64 * b_x as i64;
    if d == 0 {
        return 0;
    }

    let top_a = t_x as i64 * b_y as i64 - t_y as i64 * b_x as i64;
    let top_b = t_y as i64 * a_x as i64 - t_x as i64 * a_y as i64;

    if top_a % d != 0 || top_b % d != 0 {
        return 0;
    }

    let a = top_a / d;
    let b = top_b / d;

    if a < 0 || b < 0 {
        return 0;
    }

    (3 * a + b) as u64
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes_len = input.len();
    let ptr = input.as_bytes().as_ptr();
    let mut idx = 0usize;

    let mut total = 0u32;

    unsafe {
        while idx < bytes_len {
            match *ptr.add(idx) {
                b'\n' => {
                    break;
                }
                b'B' => {
                    idx += 1;
                    total += handle_machine_part1(ptr, &mut idx, bytes_len)
                },
                _ => unreachable_unchecked(),
            };
        }
    }

    total
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes_len = input.len();
    let ptr = input.as_bytes().as_ptr();
    let mut idx = 0usize;

    let mut total = 0u64;

    unsafe {
        while idx < bytes_len {
            match *ptr.add(idx) {
                b'\n' => {
                    break;
                }
                b'B' => {
                    idx += 1;
                    total += handle_machine_part2(ptr, &mut idx, bytes_len)
                },
                _ => unreachable_unchecked(),
            };
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/2024/day13.txt");
        assert_eq!(part1(input), 29023);
        assert_eq!(part2(input), 96787395375634);
    }
}