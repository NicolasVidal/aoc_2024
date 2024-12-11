use rustc_hash::{FxBuildHasher, FxHashMap, FxHasher};

#[inline(always)]
fn count_digits(mut n: u64) -> u32 {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count as u32
}

#[inline(always)]
fn split_number_in_half(n: u64, digits: u32) -> (u64, u64) {
    let mut left = 0;
    let mut right = 0;

    for i in 0..digits {
        let num = n / 10u64.pow(digits - i - 1) % 10;
        if i < digits / 2 {
            left = left * 10 + num;
        } else {
            right = right * 10 + num;
        }
    }


    (left, right)
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let mut iterator = bytes.iter();

    let mut numbers = Vec::new();

    let mut id = 0u64;
    while let Some(&b) = iterator.next() {
        match b {
            b' ' => {
                numbers.push(id);
                id = 0;
            }
            b'\n' => {
                numbers.push(id);
                break;
            }
            n if (b'0'..=b'9').contains(&n) => {
                id = id * 10 + (n - b'0') as u64;
            }
            _ => {}
        }
    }

    numbers.push(id);

    let mut stack = Vec::new();

    for n in numbers {
        stack.push((n, 0));
    }

    let mut total = 0u64;

    let mut map = FxHashMap::with_capacity_and_hasher(2usize.pow(11), FxBuildHasher::default());

    while let Some((n, iteration)) = stack.pop() {
        total += recursive_count(n, iteration, &mut map, 25);
    }

    // dbg!(map.capacity());
    // dbg!(map.len());

    total
}

#[inline(always)]
pub fn recursive_count(n: u64, iteration: u64, map: &mut FxHashMap<(u64, i32), u64>, max_iter: u64) -> u64 {
    if iteration == max_iter {
        return 1;
    }

    if let Some(&result) = map.get(&(n, iteration as i32)) {
        return result;
    }

    let result = if n == 0 {
        recursive_count(1, iteration + 1, map, max_iter)
    } else {
        let digits = count_digits(n);
        if digits % 2 == 0 {
            let (left, right) = split_number_in_half(n, digits);
            recursive_count(left, iteration + 1, map, max_iter) + recursive_count(right, iteration + 1, map, max_iter)
        } else {
            recursive_count(n * 2024, iteration + 1, map, max_iter)
        }
    };

    map.insert((n, iteration as i32), result);

    result
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let mut iterator = bytes.iter();

    let mut numbers = Vec::new();

    let mut id = 0u64;
    while let Some(&b) = iterator.next() {
        match b {
            b' ' => {
                numbers.push(id);
                id = 0;
            }
            b'\n' => {
                numbers.push(id);
                break;
            }
            n if (b'0'..=b'9').contains(&n) => {
                id = id * 10 + (n - b'0') as u64;
            }
            _ => {}
        }
    }

    numbers.push(id);

    let mut stack = Vec::new();

    for n in numbers {
        stack.push((n, 0));
    }

    let mut total = 0u64;

    let mut map = FxHashMap::with_capacity_and_hasher(2usize.pow(18), FxBuildHasher::default());

    while let Some((n, iteration)) = stack.pop() {
        total += recursive_count(n, iteration, &mut map, 75);
    }

    // dbg!(map.capacity());
    // dbg!(map.len());

    total
}
