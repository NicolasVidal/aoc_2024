use std::hint::unreachable_unchecked;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

const ROWS: usize = 50;
const COLS: usize = 50;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let bytes_len = bytes.len();
    let bytes_ptr = bytes.as_ptr();
    let mut idx = 0usize;

    let mut robot_pos = (0, 0);
    let mut grid = [[Cell::Empty; COLS]; ROWS];

    let mut line = 0usize;
    'outer: loop {
        let mut col = 0usize;
        loop {
            let value = unsafe { *bytes_ptr.add(idx) };
            idx += 1;
            if value == b'\n' {
                if col == 0 {
                    break 'outer;
                }
                break;
            }

            match value {
                b'.' => {
                    (unsafe {*grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Empty});
                }
                b'#' => {
                    (unsafe {*grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Wall});
                }
                b'O' => {
                    (unsafe {*grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Box});
                }
                b'@' => {
                    (unsafe {*grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Robot});
                    robot_pos = (line, col);
                }
                _ => unsafe { unreachable_unchecked() },
            }
            col += 1;
        }
        line += 1;
    }

    // for _ in 0..5 {
    while idx < bytes_len {
        let value = unsafe { *bytes_ptr.add(idx) };
        idx += 1;
        if value == b'\n' {
            continue;
        }

        match value {
            b'^' => {
                // find next empty line
                let mut line = robot_pos.0 - 1;
                let mut found = false;
                loop {
                    if (unsafe {*grid.get_unchecked(line).get_unchecked(robot_pos.1)}) == Cell::Wall {
                        break;
                    }
                    if (unsafe {*grid.get_unchecked(line).get_unchecked(robot_pos.1)}) == Cell::Empty {
                        found = true;
                        break;
                    }
                    if line == 0 {
                        break;
                    }
                    line -= 1;
                }

                if !found {
                    continue;
                }

                while line != robot_pos.0 {
                    let next_pos = (line, robot_pos.1);
                    (unsafe {*grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0 + 1).get_unchecked(next_pos.1)});
                    line += 1;
                }

                (unsafe {*grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty});
                robot_pos = (robot_pos.0 - 1, robot_pos.1);
            }
            b'v' => {
                // find next empty line
                let mut line = robot_pos.0 + 1;
                let mut found = false;
                loop {
                    if (unsafe {*grid.get_unchecked(line).get_unchecked(robot_pos.1)}) == Cell::Wall {
                        break;
                    }
                    if (unsafe {*grid.get_unchecked(line).get_unchecked(robot_pos.1)}) == Cell::Empty {
                        found = true;
                        break;
                    }

                    if line == ROWS - 1 {
                        break;
                    }

                    line += 1;
                }

                if !found {
                    continue;
                }

                while line != robot_pos.0 {
                    let next_pos = (line, robot_pos.1);
                    (unsafe {*grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0 - 1).get_unchecked(next_pos.1)});
                    line -= 1;
                }

                (unsafe {*grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty});
                robot_pos = (robot_pos.0 + 1, robot_pos.1);
            }
            b'>' => {
                // find next empty line
                let mut col = robot_pos.1 + 1;
                let mut found = false;
                loop{
                    if unsafe {*grid.get_unchecked(robot_pos.0).get_unchecked(col)} == Cell::Wall {
                        break;
                    }
                    if unsafe {*grid.get_unchecked(robot_pos.0).get_unchecked(col)} == Cell::Empty {
                        found = true;
                        break;
                    }

                    if col == COLS - 1 {
                        break;
                    }

                    col += 1;
                }

                if !found {
                    continue;
                }

                while col != robot_pos.1 {
                    let next_pos = (robot_pos.0, col);
                    (unsafe {*grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0).get_unchecked(next_pos.1 - 1)});
                    col -= 1;
                }

                (unsafe {*grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty});
                robot_pos = (robot_pos.0, robot_pos.1 + 1);
            }
            b'<' => {
                // find next empty line
                let mut col = robot_pos.1 - 1;
                let mut found = false;
                loop {
                    if unsafe {*grid.get_unchecked(robot_pos.0).get_unchecked(col)} == Cell::Wall {
                        break;
                    }
                    if unsafe {*grid.get_unchecked(robot_pos.0).get_unchecked(col)} == Cell::Empty {
                        found = true;
                        break;
                    }

                    if col == 0 {
                        break;
                    }

                    col -= 1;
                }

                if !found {
                    continue;
                }

                while col != robot_pos.1 {
                    let next_pos = (robot_pos.0, col);
                    (unsafe {*grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0).get_unchecked(next_pos.1 + 1)});
                    col += 1;
                }

                (unsafe {*grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty});
                robot_pos = (robot_pos.0, robot_pos.1 - 1);
            }
            _ => unsafe { unreachable_unchecked() },
        }

        // for row in 0..ROWS {
        //     for col in 0..COLS {
        //         print!(
        //             "{}",
        //             match grid[row][col] {
        //                 Cell::Empty => ".",
        //                 Cell::Robot => "@",
        //                 Cell::Wall => "#",
        //                 Cell::Box => "O",
        //             }
        //         );
        //     }
        //     println!();
        // }
        // println!();

    }
    let mut total = 0u64;
    for row in 0..ROWS {
        for col in 0..COLS {
            if unsafe {*grid.get_unchecked(row).get_unchecked(col) == Cell::Box} {
                total += 100 * row as u64 + col as u64;
            }
        }
    }

    total
}

// 1441031
// 2844500
// 2844500 too high

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    unsafe {
        let bytes = input.as_bytes();
        let bytes_len = bytes.len();
        let bytes_ptr = bytes.as_ptr();
        let mut idx = 0usize;

        let mut robot_pos = (0, 0);
        let mut grid = [[Cell::Empty; COLS * 2]; ROWS];

        let mut line = 0usize;
        'outer: loop {
            let mut col = 0usize;
            loop {
                let value = unsafe { *bytes_ptr.add(idx) };
                idx += 1;
                if value == b'\n' {
                    if col == 0 {
                        break 'outer;
                    }
                    break;
                }

                match value {
                    b'.' => {
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Empty;
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col + 1) = Cell::Empty;
                    }
                    b'#' => {
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Wall;
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col + 1) = Cell::Wall;
                    }
                    b'O' => {
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::BoxLeft;
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col + 1) = Cell::BoxRight;
                    }
                    b'@' => {
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col) = Cell::Robot;
                        *grid.get_unchecked_mut(line).get_unchecked_mut(col + 1) = Cell::Empty;
                        robot_pos = (line, col);
                    }
                    _ => unsafe { unreachable_unchecked() },
                }
                col += 2;
            }
            line += 1;
        }

        // for _ in 0..5 {
        while idx < bytes_len {
            let value = unsafe { *bytes_ptr.add(idx) };
            idx += 1;
            if value == b'\n' {
                continue;
            }

            match value {
                b'^' => {
                    // find next empty line
                    let mut line = robot_pos.0 - 1;
                    let mut found = false;
                    let mut current_cols_to_scan = FxHashSet::default();
                    current_cols_to_scan.insert(robot_pos.1);
                    let mut all_cols_to_scan = vec![current_cols_to_scan.clone()];
                    loop {
                        if current_cols_to_scan.iter().any(|&b| *grid.get_unchecked(line).get_unchecked(b) == Cell::Wall) {
                            break;
                        }
                        if current_cols_to_scan.iter().all(|&b| *grid.get_unchecked(line).get_unchecked(b) == Cell::Empty) {
                            found = true;
                            break;
                        }
                        let mut cloned = current_cols_to_scan.clone();
                        for &col in current_cols_to_scan.iter() {
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::Empty {
                                cloned.remove(&col);
                            }
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::BoxLeft {
                                cloned.insert(col + 1);
                            }
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::BoxRight {
                                cloned.insert(col - 1);
                            }
                        }
                        current_cols_to_scan = cloned.clone();
                        all_cols_to_scan.push(cloned);

                        if line == 0 {
                            break;
                        }
                        line -= 1;
                    }

                    if !found {
                        continue;
                    }

                    while line != robot_pos.0 {
                        for col in all_cols_to_scan.pop().unwrap() {
                            let next_pos = (line, col);
                            *grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0 + 1).get_unchecked(next_pos.1);
                            *grid.get_unchecked_mut(next_pos.0 + 1).get_unchecked_mut(next_pos.1) = Cell::Empty;
                        }
                        line += 1;
                    }

                    *grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty;
                    robot_pos = (robot_pos.0 - 1, robot_pos.1);
                }
                b'v' => {
                    // find next empty line
                    let mut line = robot_pos.0 + 1;
                    let mut found = false;
                    let mut current_cols_to_scan = FxHashSet::default();
                    current_cols_to_scan.insert(robot_pos.1);
                    let mut all_cols_to_scan = vec![current_cols_to_scan.clone()];
                    loop {
                        if current_cols_to_scan.iter().any(|&b| *grid.get_unchecked(line).get_unchecked(b) == Cell::Wall) {
                            break;
                        }
                        if current_cols_to_scan.iter().all(|&b| *grid.get_unchecked(line).get_unchecked(b) == Cell::Empty) {
                            found = true;
                            break;
                        }
                        let mut cloned = current_cols_to_scan.clone();
                        for &col in current_cols_to_scan.iter() {
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::Empty {
                                cloned.remove(&col);
                            }
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::BoxLeft {
                                cloned.insert(col + 1);
                            }
                            if *grid.get_unchecked(line).get_unchecked(col) == Cell::BoxRight {
                                cloned.insert(col - 1);
                            }
                        }
                        current_cols_to_scan = cloned.clone();
                        all_cols_to_scan.push(cloned);

                        if line == ROWS - 1 {
                            break;
                        }
                        line += 1;
                    }

                    if !found {
                        continue;
                    }

                    while line != robot_pos.0 {
                        for col in all_cols_to_scan.pop().unwrap() {
                            let next_pos = (line, col);
                            *grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0 - 1).get_unchecked(next_pos.1);
                            *grid.get_unchecked_mut(next_pos.0 - 1).get_unchecked_mut(next_pos.1) = Cell::Empty;
                        }
                        line -= 1;
                    }

                    *grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty;
                    robot_pos = (robot_pos.0 + 1, robot_pos.1);
                }
                b'>' => {
                    // find next empty line
                    let mut col = robot_pos.1 + 1;
                    let mut found = false;
                    loop {
                        if *grid.get_unchecked(robot_pos.0).get_unchecked(col) == Cell::Wall {
                            break;
                        }
                        if *grid.get_unchecked(robot_pos.0).get_unchecked(col) == Cell::Empty {
                            found = true;
                            break;
                        }

                        if col == (COLS * 2) - 1 {
                            break;
                        }

                        col += 1;
                    }

                    if !found {
                        continue;
                    }

                    while col != robot_pos.1 {
                        let next_pos = (robot_pos.0, col);
                        *grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0).get_unchecked(next_pos.1 - 1);
                        col -= 1;
                    }

                    *grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty;
                    robot_pos = (robot_pos.0, robot_pos.1 + 1);
                }
                b'<' => {
                    // find next empty line
                    let mut col = robot_pos.1 - 1;
                    let mut found = false;
                    loop {
                        if *grid.get_unchecked(robot_pos.0).get_unchecked(col) == Cell::Wall {
                            break;
                        }
                        if *grid.get_unchecked(robot_pos.0).get_unchecked(col) == Cell::Empty {
                            found = true;
                            break;
                        }

                        if col == 0 {
                            break;
                        }

                        col -= 1;
                    }

                    if !found {
                        continue;
                    }

                    while col != robot_pos.1 {
                        let next_pos = (robot_pos.0, col);
                        *grid.get_unchecked_mut(next_pos.0).get_unchecked_mut(next_pos.1) = *grid.get_unchecked(next_pos.0).get_unchecked(next_pos.1 + 1);
                        col += 1;
                    }

                    *grid.get_unchecked_mut(robot_pos.0).get_unchecked_mut(robot_pos.1) = Cell::Empty;
                    robot_pos = (robot_pos.0, robot_pos.1 - 1);
                }
                _ => unsafe { unreachable_unchecked() },
            }

            // for row in 0..ROWS {
            //     for col in 0..(COLS * 2) {
            //         print!(
            //             "{}",
            //             match grid[row][col] {
            //                 Cell::Empty => ".",
            //                 Cell::Robot => "@",
            //                 Cell::Wall => "#",
            //                 Cell::BoxLeft => "[",
            //                 Cell::BoxRight => "]",
            //                 _ => unreachable!(),
            //             }
            //         );
            //     }
            //     println!();
            // }
            // println!();
        }
        let mut total = 0u64;
        for row in 0..ROWS {
            for col in 0..(COLS * 2) {
                if *grid.get_unchecked(row).get_unchecked(col) == Cell::BoxLeft {
                    total += 100 * row as u64 + col as u64;
                }
            }
        }

        total
    }
}

// 2129612 Too high