use std::hint::unreachable_unchecked;

#[inline(always)]
fn handle_machine_part1(iter: &mut std::slice::Iter<u8>) -> Option<u32> {
    let a_x = (iter.skip(11).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';
    let a_y = (iter.skip(4).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';

    let b_x = (iter.skip(13).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';
    let b_y = (iter.skip(4).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';

    let mut t_x = (iter.skip(10).next().unwrap() - b'0') as u32;

    loop {
        match iter.next() {
            Some(b',') => {
                break;
            }
            Some(&x) => {
                t_x = t_x * 10 + (x - b'0') as u32;
            }
            _ => {
                unreachable!();
            }
        }
    }

    let mut t_y = (iter.skip(3).next().unwrap() - b'0') as u32;

    loop {
        match iter.next() {
            Some(b'\n') => {
                break;
            }
            Some(&y) => {
                t_y = t_y * 10 + (y - b'0') as u32;
            }
            None => {
                break;
            }
        }
    }

    iter.next();

    let d = a_x as i32 * b_y as i32 - a_y as i32 * b_x as i32;
    if d == 0 {
        return None;
    }

    let top_a = t_x as i32 * b_y as i32 - t_y as i32 * b_x as i32;
    let top_b = t_y as i32 * a_x as i32 - t_x as i32 * a_y as i32;

    if top_a % d != 0 || top_b % d != 0 {
        return None;
    }

    let a = top_a / d;
    let b = top_b / d;

    if a < 0 || b < 0 {
        return None;
    }

    Some((3 * a + b) as u32)
}

#[inline(always)]
fn handle_machine_part2(iter: &mut std::slice::Iter<u8>) -> Option<u64> {
    let a_x = (iter.skip(11).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';
    let a_y = (iter.skip(4).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';

    let b_x = (iter.skip(13).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';
    let b_y = (iter.skip(4).next().unwrap() - b'0') * 10 + iter.next().unwrap() - b'0';

    let mut t_x = (iter.skip(10).next().unwrap() - b'0') as u64;

    loop {
        match iter.next() {
            Some(b',') => {
                break;
            }
            Some(&x) => {
                t_x = t_x * 10 + (x - b'0') as u64;
            }
            _ => {
                unreachable!();
            }
        }
    }

    let mut t_y = (iter.skip(3).next().unwrap() - b'0') as u64;

    loop {
        match iter.next() {
            Some(b'\n') => {
                break;
            }
            Some(&y) => {
                t_y = t_y * 10 + (y - b'0') as u64;
            }
            None => {
                break;
            }
        }
    }

    iter.next();

    t_x = t_x + 10000000000000;
    t_y = t_y + 10000000000000;

    let d = a_x as i64 * b_y as i64 - a_y as i64 * b_x as i64;
    if d == 0 {
        return None;
    }

    let top_a = t_x as i64 * b_y as i64 - t_y as i64 * b_x as i64;
    let top_b = t_y as i64 * a_x as i64 - t_x as i64 * a_y as i64;

    if top_a % d != 0 || top_b % d != 0 {
        return None;
    }

    let a = top_a / d;
    let b = top_b / d;

    if a < 0 || b < 0 {
        return None;
    }

    Some((3 * a + b) as u64)
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
            Some(b'B') => total += handle_machine_part1(&mut it).unwrap_or_else(|| 0),
            None => break,
            _ => unsafe {
                unreachable_unchecked();
            }
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
            Some(b'B') => total += handle_machine_part2(&mut it).unwrap_or_else(|| 0),
            None => break,
            _ => unsafe {
                unreachable_unchecked();
            }
        };
    }

    total
}
