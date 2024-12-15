use std::hint::unreachable_unchecked;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Robot,
    Wall,
    Box,
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
                    grid[line][col] = Cell::Empty;
                }
                b'#' => {
                    grid[line][col] = Cell::Wall;
                }
                b'O' => {
                    grid[line][col] = Cell::Box;
                }
                b'@' => {
                    grid[line][col] = Cell::Robot;
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
                    if grid[line][robot_pos.1] == Cell::Wall {
                        break;
                    }
                    if grid[line][robot_pos.1] == Cell::Empty {
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
                    grid[next_pos.0][next_pos.1] = grid[next_pos.0 + 1][next_pos.1];
                    line += 1;
                }

                grid[robot_pos.0][robot_pos.1] = Cell::Empty;
                robot_pos = (robot_pos.0 - 1, robot_pos.1);
            }
            b'v' => {
                // find next empty line
                let mut line = robot_pos.0 + 1;
                let mut found = false;
                loop {
                    if grid[line][robot_pos.1] == Cell::Wall {
                        break;
                    }
                    if grid[line][robot_pos.1] == Cell::Empty {
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
                    grid[next_pos.0][next_pos.1] = grid[next_pos.0 - 1][next_pos.1];
                    line -= 1;
                }

                grid[robot_pos.0][robot_pos.1] = Cell::Empty;
                robot_pos = (robot_pos.0 + 1, robot_pos.1);
            }
            b'>' => {
                // find next empty line
                let mut col = robot_pos.1 + 1;
                let mut found = false;
                loop{
                    if grid[robot_pos.0][col] == Cell::Wall {
                        break;
                    }
                    if grid[robot_pos.0][col] == Cell::Empty {
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
                    grid[next_pos.0][next_pos.1] = grid[next_pos.0][next_pos.1 - 1];
                    col -= 1;
                }

                grid[robot_pos.0][robot_pos.1] = Cell::Empty;
                robot_pos = (robot_pos.0, robot_pos.1 + 1);
            }
            b'<' => {
                // find next empty line
                let mut col = robot_pos.1 - 1;
                let mut found = false;
                loop {
                    if grid[robot_pos.0][col] == Cell::Wall {
                        break;
                    }
                    if grid[robot_pos.0][col] == Cell::Empty {
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
                    grid[next_pos.0][next_pos.1] = grid[next_pos.0][next_pos.1 + 1];
                    col += 1;
                }

                grid[robot_pos.0][robot_pos.1] = Cell::Empty;
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
            if grid[row][col] == Cell::Box {
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
pub fn part2(_input: &str) -> u64 {
    42
}
