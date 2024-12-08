use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    let mut coordinates = FxHashMap::with_capacity_and_hasher(2500, FxBuildHasher::default());

    let bytes = input.as_bytes();

    let mut bytes_iter = bytes.iter();

    let mut col = 0;
    let mut row = 0;

    let cols_count = memchr::memchr(b'\n', bytes).unwrap();

    while let Some(byte) = bytes_iter.next() {
        match *byte {
            b'\n' => {
                row += 1;
                col = 0;
            }
            b'.' => {
                col += 1;
            }
            c => {
                coordinates
                    .entry(c)
                    .or_insert(Vec::<(u8, u8)>::new())
                    .push((row, col));
                col += 1;
            }
        }
    }

    let rows_count = if bytes[bytes.len() - 1] == b'\n' {
        row
    } else {
        row + 1
    };

    let mut resonance_points = FxHashSet::with_capacity_and_hasher(2500, FxBuildHasher::default());
    for (_, vec) in coordinates.into_iter() {
        let vec_len = vec.len();
        for i in 0..vec_len {
            for j in (i + 1)..vec_len {
                let (row1, col1) = vec[i];
                let (row2, col2) = vec[j];

                let target_row1 = row1 as i8 + (row1 as i8 - row2 as i8);

                if target_row1 < rows_count as i8 && target_row1 >= 0 {
                    let target_col1 = col1 as i8 + (col1 as i8 - col2 as i8);

                    if target_col1 < cols_count as i8 && target_col1 >= 0 {
                        resonance_points.insert((target_row1 as u8, target_col1 as u8));
                    }
                }

                let target_row2 = row2 as i8 + (row2 as i8 - row1 as i8);

                if target_row2 < rows_count as i8 && target_row2 >= 0 {
                    let target_col2 = col2 as i8 + (col2 as i8 - col1 as i8);

                    if target_col2 < cols_count as i8 && target_col2 >= 0 {
                        resonance_points.insert((target_row2 as u8, target_col2 as u8));
                    }
                }
            }
        }
    }

    resonance_points.len() as u32
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    let mut coordinates = FxHashMap::with_capacity_and_hasher(2500, FxBuildHasher::default());

    let bytes = input.as_bytes();

    let mut bytes_iter = bytes.iter();

    let mut col = 0;
    let mut row = 0;

    let cols_count = memchr::memchr(b'\n', bytes).unwrap();

    while let Some(byte) = bytes_iter.next() {
        match *byte {
            b'\n' => {
                row += 1;
                col = 0;
            }
            b'.' => {
                col += 1;
            }
            c => {
                coordinates
                    .entry(c)
                    .or_insert(Vec::<(u8, u8)>::new())
                    .push((row, col));
                col += 1;
            }
        }
    }

    let rows_count = if bytes[bytes.len() - 1] == b'\n' {
        row
    } else {
        row + 1
    };

    let mut resonance_points = FxHashSet::with_capacity_and_hasher(2500, FxBuildHasher::default());

    for (_, v) in coordinates.iter() {
        if v.len() < 2 {
            continue;
        }

        for p in v.iter().copied() {
            resonance_points.insert(p);
        }
    }

    for (_, vec) in coordinates.into_iter() {
        let vec_len = vec.len();
        for i in 0..vec_len {
            for j in (i + 1)..vec_len {
                let (row1, col1) = vec[i];
                let (row2, col2) = vec[j];

                let diff_row1 = row1 as i8 - row2 as i8;
                let diff_col1 = col1 as i8 - col2 as i8;
                let mut target_row1 = row1 as i8;
                let mut target_col1 = col1 as i8;
                loop {
                    target_row1 += diff_row1;

                    if target_row1 < rows_count as i8 && target_row1 >= 0 {
                        target_col1 += diff_col1;

                        if target_col1 < cols_count as i8 && target_col1 >= 0 {
                            resonance_points.insert((target_row1 as u8, target_col1 as u8));
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let diff_row2 = row2 as i8 - row1 as i8;
                let diff_col2 = col2 as i8 - col1 as i8;
                let mut target_row2 = row2 as i8;
                let mut target_col2 = col2 as i8;
                loop {
                    target_row2 += diff_row2;

                    if target_row2 < rows_count as i8 && target_row2 >= 0 {
                        target_col2 += diff_col2;

                        if target_col2 < cols_count as i8 && target_col2 >= 0 {
                            resonance_points.insert((target_row2 as u8, target_col2 as u8));
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    resonance_points.len() as u32
}