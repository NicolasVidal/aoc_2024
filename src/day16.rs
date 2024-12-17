use std::collections::HashSet;
use std::hint::unreachable_unchecked;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Empty,
    Wall,
    End,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ScoredCell {
    score : u64,
    cell: Cell,
}

impl Default for ScoredCell {
    fn default() -> Self {
        Self {
            score: u64::MAX,
            cell: Cell::Empty,
        }
    }
}

const ROWS: usize = 142;
const COLS: usize = 142;

const NORTH: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const EAST: usize = 3;

fn parse_grid(bytes: &[u8]) -> [[[ScoredCell; 4]; COLS]; ROWS] {
    let mut idx = 0;
    let mut row = 0;
    let mut col = 0;

    let mut grid = [[[ScoredCell::default(); 4]; COLS]; ROWS];

    loop {
        if idx == bytes.len() {
            break;
        }
        let byte = bytes[idx];
        idx += 1;
        match byte {
            b'.' => {
                col += 1;
            }
            b'#' => {
                for i in 0..4 {
                    grid[row][col][i].cell = Cell::Wall;
                }
                col += 1;
            }
            b'\n' => {
                row += 1;
                col = 0;
            }
            b'S' => {
                // Agent starts turning east
                grid[row][col][EAST].score = 0;
                col += 1;
            }
            b'E' => {
                for i in 0..4 {
                    grid[row][col][i].cell = Cell::End;
                }
                col += 1;
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    grid
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u64 {
    let mut grid = parse_grid(input.as_bytes());

    loop {
        let (row, col, direction) = {
            let mut min_score = u64::MAX;
            let mut min_row = 0;
            let mut min_col = 0;
            let mut min_direction = 0;

            for r in 0..ROWS {
                for c in 0..COLS {
                    for d in 0..4 {
                        if grid[r][c][d].cell != Cell::Wall && grid[r][c][d].score < min_score {
                            min_score = grid[r][c][d].score;
                            min_row = r;
                            min_col = c;
                            min_direction = d;
                        }
                    }
                }
            }

            (min_row, min_col, min_direction)
        };

        if grid[row][col][direction].cell == Cell::End {
            return grid[row][col][direction].score;
        }

        if grid[row][col][direction].score == u64::MAX {
            return u64::MAX;
        }

        // update neighbours cells

        match direction {
            NORTH => {
                let score = grid[row][col][direction].score + 1;
                if grid[row - 1][col][NORTH].cell != Cell::Wall && grid[row - 1][col][direction].score > score {
                    grid[row - 1][col][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][EAST].cell != Cell::Wall && grid[row][col][EAST].score > turning_score {
                    grid[row][col][EAST].score = turning_score;
                }
                if grid[row][col][WEST].cell != Cell::Wall && grid[row][col][WEST].score > turning_score {
                    grid[row][col][WEST].score = turning_score;
                }
            },
            SOUTH => {
                let score = grid[row][col][direction].score + 1;
                if grid[row + 1][col][SOUTH].cell != Cell::Wall && grid[row + 1][col][direction].score > score {
                    grid[row + 1][col][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][EAST].cell != Cell::Wall && grid[row][col][EAST].score > turning_score {
                    grid[row][col][EAST].score = turning_score;
                }
                if grid[row][col][WEST].cell != Cell::Wall && grid[row][col][WEST].score > turning_score {
                    grid[row][col][WEST].score = turning_score;
                }
            },
            WEST => {
                let score = grid[row][col][direction].score + 1;
                if grid[row][col - 1][WEST].cell != Cell::Wall && grid[row][col - 1][direction].score > score {
                    grid[row][col - 1][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][NORTH].cell != Cell::Wall && grid[row][col][NORTH].score > turning_score {
                    grid[row][col][NORTH].score = turning_score;
                }
                if grid[row][col][SOUTH].cell != Cell::Wall && grid[row][col][SOUTH].score > turning_score {
                    grid[row][col][SOUTH].score = turning_score;
                }
            },
            EAST => {
                let score = grid[row][col][direction].score + 1;
                if grid[row][col + 1][EAST].cell != Cell::Wall && grid[row][col + 1][direction].score > score {
                    grid[row][col + 1][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][NORTH].cell != Cell::Wall && grid[row][col][NORTH].score > turning_score {
                    grid[row][col][NORTH].score = turning_score;
                }
                if grid[row][col][SOUTH].cell != Cell::Wall && grid[row][col][SOUTH].score > turning_score {
                    grid[row][col][SOUTH].score = turning_score;
                }
            },
            _ => unsafe { unreachable_unchecked() },
        }
        grid[row][col][direction].cell = Cell::Wall;
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
    let mut grid = parse_grid(input.as_bytes());

    let mut end_founds = Vec::new();

    loop {
        let (row, col, direction) = {
            let mut min_score = u64::MAX;
            let mut min_row = 0;
            let mut min_col = 0;
            let mut min_direction = 0;

            for r in 0..ROWS {
                for c in 0..COLS {
                    for d in 0..4 {
                        if grid[r][c][d].cell != Cell::Wall && grid[r][c][d].score < min_score {
                            min_score = grid[r][c][d].score;
                            min_row = r;
                            min_col = c;
                            min_direction = d;
                        }
                    }
                }
            }

            (min_row, min_col, min_direction)
        };

        if grid[row][col][direction].cell == Cell::End {
            end_founds.push((grid[row][col][direction].score, row, col, direction));
            grid[row][col][direction].cell = Cell::Wall;
            continue;
        }

        if end_founds.iter().any(|(end_score, _, _, _)| *end_score < grid[row][col][direction].score) {
            break;
        }

        if grid[row][col][direction].score == u64::MAX {
            return u64::MAX;
        }

        // update neighbours cells

        match direction {
            NORTH => {
                let score = grid[row][col][direction].score + 1;
                if grid[row - 1][col][NORTH].cell != Cell::Wall && grid[row - 1][col][direction].score > score {
                    grid[row - 1][col][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][EAST].cell != Cell::Wall && grid[row][col][EAST].score > turning_score {
                    grid[row][col][EAST].score = turning_score;
                }
                if grid[row][col][WEST].cell != Cell::Wall && grid[row][col][WEST].score > turning_score {
                    grid[row][col][WEST].score = turning_score;
                }
            },
            SOUTH => {
                let score = grid[row][col][direction].score + 1;
                if grid[row + 1][col][SOUTH].cell != Cell::Wall && grid[row + 1][col][direction].score > score {
                    grid[row + 1][col][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][EAST].cell != Cell::Wall && grid[row][col][EAST].score > turning_score {
                    grid[row][col][EAST].score = turning_score;
                }
                if grid[row][col][WEST].cell != Cell::Wall && grid[row][col][WEST].score > turning_score {
                    grid[row][col][WEST].score = turning_score;
                }
            },
            WEST => {
                let score = grid[row][col][direction].score + 1;
                if grid[row][col - 1][WEST].cell != Cell::Wall && grid[row][col - 1][direction].score > score {
                    grid[row][col - 1][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][NORTH].cell != Cell::Wall && grid[row][col][NORTH].score > turning_score {
                    grid[row][col][NORTH].score = turning_score;
                }
                if grid[row][col][SOUTH].cell != Cell::Wall && grid[row][col][SOUTH].score > turning_score {
                    grid[row][col][SOUTH].score = turning_score;
                }
            },
            EAST => {
                let score = grid[row][col][direction].score + 1;
                if grid[row][col + 1][EAST].cell != Cell::Wall && grid[row][col + 1][direction].score > score {
                    grid[row][col + 1][direction].score = score;
                }

                let turning_score = grid[row][col][direction].score + 1000;
                if grid[row][col][NORTH].cell != Cell::Wall && grid[row][col][NORTH].score > turning_score {
                    grid[row][col][NORTH].score = turning_score;
                }
                if grid[row][col][SOUTH].cell != Cell::Wall && grid[row][col][SOUTH].score > turning_score {
                    grid[row][col][SOUTH].score = turning_score;
                }
            },
            _ => unsafe { unreachable_unchecked() },
        }
        grid[row][col][direction].cell = Cell::Wall;
    };

    let mut hs = HashSet::new();
    while let Some((score, row, col, direction)) = end_founds.pop() {

        hs.insert((row, col));
        if score == 0 {
            continue;
        }

        // Check neighbours for provenance
        match direction {
            NORTH => {
                if grid[row + 1][col][NORTH].score == score - 1 {
                    end_founds.push((score - 1, row + 1, col, NORTH));
                }
                if grid[row][col][EAST].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, EAST));
                }
                if grid[row][col][WEST].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, WEST));
                }
            },
            SOUTH => {
                if grid[row - 1][col][SOUTH].score == score - 1 {
                    end_founds.push((score - 1, row - 1, col, SOUTH));
                }
                if grid[row][col][EAST].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, EAST));
                }
                if grid[row][col][WEST].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, WEST));
                }
            },
            WEST => {
                if grid[row][col + 1][WEST].score == score - 1 {
                    end_founds.push((score - 1, row, col + 1, WEST));
                }
                if grid[row][col][NORTH].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, NORTH));
                }
                if grid[row][col][SOUTH].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, SOUTH));
                }
            },
            EAST => {
                if grid[row][col - 1][EAST].score == score - 1 {
                    end_founds.push((score - 1, row, col - 1, EAST));
                }
                if grid[row][col][NORTH].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, NORTH));
                }
                if grid[row][col][SOUTH].score == score - 1000 {
                    end_founds.push((score - 1000, row, col, SOUTH));
                }
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }

    hs.len() as u64
}
