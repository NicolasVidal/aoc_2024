use std::hint::unreachable_unchecked;

#[inline(always)]
unsafe fn handle_machine_part1(iter: &mut std::slice::Iter<u8>) -> u32 {
    let slice = iter.as_slice();
    let ptr = slice.as_ptr();

    let a_x = (*ptr.add(11) - b'0') * 10 + (*ptr.add(12) - b'0');
    let a_y = (*ptr.add(17) - b'0') * 10 + (*ptr.add(18) - b'0');

    let b_x = (*ptr.add(32) - b'0') * 10 + (*ptr.add(33) - b'0');
    let b_y = (*ptr.add(38) - b'0') * 10 + (*ptr.add(39) - b'0');

    let mut t_x = (*ptr.add(50) - b'0') as u32;

    let mut idx = 51;

    while *ptr.add(idx) != b',' {
        t_x = t_x * 10 + (*ptr.add(idx) - b'0') as u32;
        idx += 1;
    }

    idx += 4;

    let mut t_y = (*ptr.add(idx) - b'0') as u32;

    idx += 1;

    let slice_len = slice.len();
    while idx < slice_len && *ptr.add(idx) != b'\n' {
        t_y = t_y * 10 + (*ptr.add(idx) - b'0') as u32;
        idx += 1;
    }

    *iter = slice[idx..].iter();
    iter.next();
    iter.next();

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
unsafe fn handle_machine_part2(iter: &mut std::slice::Iter<u8>) -> u64 {
    let slice = iter.as_slice();
    let ptr = slice.as_ptr();

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

    let slice_len = slice.len();
    while idx < slice_len && *ptr.add(idx) != b'\n' {
        t_y = t_y * 10 + (*ptr.add(idx) - b'0') as u64;
        idx += 1;
    }

    *iter = slice[idx..].iter();
    iter.next();
    iter.next();

    t_x = t_x + 10000000000000;
    t_y = t_y + 10000000000000;

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
    let bytes = input.as_bytes();

    let mut it = bytes.iter();

    let mut total = 0u32;

    loop {
        match it.next() {
            Some(b'\n') => {
                break;
            }
            Some(b'B') => total += unsafe { handle_machine_part1(&mut it) },
            None => break,
            _ => unsafe {
                unreachable_unchecked();
            },
        };
    }

    total
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let mut it = bytes.iter();

    let mut total = 0u64;

    loop {
        match it.next() {
            Some(b'\n') => {
                break;
            }
            Some(b'B') => total += unsafe { handle_machine_part2(&mut it) },
            None => break,
            _ => unsafe {
                unreachable_unchecked();
            },
        };
    }

    total
}
