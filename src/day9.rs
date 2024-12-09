#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let mut bytes_iter = bytes.iter();

    let mut v = Vec::new();
    let mut id = 0u64;

    loop {
        let Some(&b) = bytes_iter.next() else {
            break;
        };

        if b == b'\n' {
            break;
        }

        let num = b - b'0';
        for _ in 0..num {
            v.push(Some(id));
        }

        id += 1;

        let Some(&b) = bytes_iter.next() else {
            break;
        };

        if b == b'\n' {
            break;
        }

        let num = b - b'0';
        for _ in 0..num {
            v.push(None);
        }
    }

    let mut start = 0;
    let mut end = v.len() - 1;
    'outer: loop {
        while v[start].is_some() {
            start += 1;
            if end == start {
                break 'outer;
            }
        }

        while v[end].is_none() {
            end -= 1;
            if end == start {
                break 'outer;
            }
        }

        v.swap(start, end);
    }

    v.into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.map(|x| {
                x * i as u64
            })
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();

    let mut bytes_iter = bytes.iter();

    let mut v = Vec::new();
    let mut id = 0u64;

    loop {
        let Some(&b) = bytes_iter.next() else {
            break;
        };

        if b == b'\n' {
            break;
        }

        let num = b - b'0';
        for _ in 0..num {
            v.push(Some(id));
        }

        id += 1;

        let Some(&b) = bytes_iter.next() else {
            break;
        };

        if b == b'\n' {
            break;
        }

        let num = b - b'0';
        for _ in 0..num {
            v.push(None);
        }
    }

    'rev: for rev_id in (0..id).rev() {
        // println!(
        //     "{}",
        //     v.iter()
        //         .map(|x| {
        //             match x {
        //                 Some(x) => format!("{}", x),
        //                 None => format!("."),
        //             }
        //         })
        //         .collect::<Vec<_>>()
        //         .join("")
        // );

        let mut end = v.len() - 1;
        loop {
            match v[end] {
                Some(id) if id != rev_id => {
                    end -= 1;
                    if end == 0 {
                        continue 'rev;
                    }
                }
                None => {
                    end -= 1;
                    if end == 0 {
                        continue 'rev;
                    }
                }
                _ => break,
            }
        }

        let mut count = 0;
        while let Some(id) = v[end] {
            if id != rev_id {
                break;
            }
            end -= 1;
            count += 1;
            if end == 0 {
                continue 'rev;
            }
        }

        let mut start = 0;

        loop {
            if start == v.len() || start > end {
                continue 'rev;
            }
            while let Some(_) = v[start] {
                start += 1;
                if start == v.len() || start > end {
                    continue 'rev;
                }
            }

            let mut count_spaces = 0;
            while let None = v[start] {
                start += 1;
                count_spaces += 1;
                if count_spaces == count {
                    break;
                }
                if start == v.len() || start > end {
                    continue 'rev;
                }
            }


            if count_spaces == count {
                for c in 0..count_spaces {
                    v.swap(start - c - 1, end + c + 1);
                }
            }
        }
    }

    v.into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.map(|x| {
                x * i as u64
            })
        })
        .sum()
}
