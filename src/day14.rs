use std::hint::unreachable_unchecked;

#[inline(always)]
fn parse_number_and_advance_u8(ptr: *const u8, original_idx: &mut usize, end_byte: u8) -> u8 {
    let mut number = 0u8;
    let mut idx = *original_idx;

    loop {
        let value = unsafe { *ptr.add(idx) };
        if value == end_byte {
            break;
        }
        number = number * 10 + (value - b'0');
        idx += 1;
    }

    *original_idx = idx + 1;
    number
}

#[inline(always)]
fn parse_number_and_advance_i8(ptr: *const u8, original_idx: &mut usize, end_byte: u8) -> i8 {
    let mut number = 0i8;
    let mut idx = *original_idx;
    let mut sign = 1i8;

    if unsafe { *ptr.add(idx) } == b'-' {
        sign = -1i8;
        idx += 1;
    }

    loop {
        let value = unsafe { *ptr.add(idx) };
        if value == end_byte {
            break;
        }
        number = number * 10 + (value - b'0') as i8;
        idx += 1;
    }

    *original_idx = idx + 1;
    number * sign
}

#[inline(always)]
fn parse_number_to_the_end_and_advance_i8(
    ptr: *const u8,
    original_idx: &mut usize,
    max_len: usize,
    end_byte: u8,
) -> i8 {
    let mut number = 0i8;
    let mut idx = *original_idx;
    let mut sign = 1i8;

    if unsafe { *ptr.add(idx) } == b'-' {
        sign = -1i8;
        idx += 1;
    }

    while idx < max_len {
        let value = unsafe { *ptr.add(idx) };
        if value == end_byte {
            break;
        }
        number = number * 10 + (value - b'0') as i8;
        idx += 1;
    }

    *original_idx = idx + 1;
    number * sign
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let bytes_len = bytes.len();
    let bytes_ptr = bytes.as_ptr();
    let mut idx = 0usize;

    const COLS: i32 = 101;
    const ROWS: i32 = 103;
    const SECONDS: i32 = 100;

    const SAFE_X_DISPLACEMENT: i32 = SECONDS * COLS;
    const SAFE_Y_DISPLACEMENT: i32 = SECONDS * ROWS;

    let mut top_left_quadrant = 0u32;
    let mut top_right_quadrant = 0u32;
    let mut bottom_left_quadrant = 0u32;
    let mut bottom_right_quadrant = 0u32;

    loop {
        if idx >= bytes_len {
            break;
        }

        idx += 2;
        let x = parse_number_and_advance_u8(bytes_ptr, &mut idx, b',');
        let y = parse_number_and_advance_u8(bytes_ptr, &mut idx, b' ');

        idx += 2;
        let dx = parse_number_and_advance_i8(bytes_ptr, &mut idx, b',');
        let dy = parse_number_to_the_end_and_advance_i8(bytes_ptr, &mut idx, bytes_len, b'\n');

        let x = x as i32;
        let y = y as i32;

        let dx = dx as i32;
        let dy = dy as i32;

        let final_x = (x + SAFE_X_DISPLACEMENT + SECONDS * dx) % COLS;
        let final_y = (y + SAFE_Y_DISPLACEMENT + SECONDS * dy) % ROWS;

        if final_x < COLS / 2 {
            if final_y < ROWS / 2 {
                top_left_quadrant += 1;
            } else if final_y > ROWS / 2 {
                bottom_left_quadrant += 1;
            }
        } else if final_x > COLS / 2 {
            if final_y < ROWS / 2 {
                top_right_quadrant += 1;
            } else if final_y > ROWS / 2 {
                bottom_right_quadrant += 1;
            }
        }
    }

    top_left_quadrant * top_right_quadrant * bottom_left_quadrant * bottom_right_quadrant
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let bytes_len = bytes.len();
    let bytes_ptr = bytes.as_ptr();

    const COLS: i32 = 101;
    const ROWS: i32 = 103;

    for seconds in 0.. {
        let safe_x_displacement: i32 = seconds * COLS;
        let safe_y_displacement: i32 = seconds * ROWS;

        let mut array = [0u8; (ROWS * COLS) as usize];

        let mut idx = 0usize;
        loop {
            if idx >= bytes_len {
                break;
            }

            idx += 2;
            let x = parse_number_and_advance_u8(bytes_ptr, &mut idx, b',');
            let y = parse_number_and_advance_u8(bytes_ptr, &mut idx, b' ');

            idx += 2;
            let dx = parse_number_and_advance_i8(bytes_ptr, &mut idx, b',');
            let dy = parse_number_to_the_end_and_advance_i8(bytes_ptr, &mut idx, bytes_len, b'\n');

            let x = x as i32;
            let y = y as i32;

            let dx = dx as i32;
            let dy = dy as i32;

            let final_x = (x + safe_x_displacement + seconds * dx) % COLS;
            let final_y = (y + safe_y_displacement + seconds * dy) % ROWS;

            array[(final_y * COLS) as usize + final_x as usize] = 1;
        }

        for pos in memchr::memrchr3_iter(1, 1, 1, &array[..(bytes_len - 31)]) {
            if array[pos + 3..pos + 20].iter().all(|&x| x == 1) {
                return seconds as u64;
            }
        }

    }

    unsafe { unreachable_unchecked() }
}
