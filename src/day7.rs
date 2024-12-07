use std::iter::Cycle;

#[derive(Debug)]
struct Line {
    total: u64,
    candidates: heapless::Vec<u64, 16>,
}

impl Line {
    fn parse(bytes: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let mut total = 0;
        let mut candidates = heapless::Vec::new();
        while let Some(byte) = bytes.next() {
            match byte {
                b'\n' => {
                    return None;
                }
                b':' => {
                    break;
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    total = total * 10 + (byte - b'0') as u64;
                }
                _ => {
                    panic!("Invalid byte while parsing total: {}", byte as char);
                }
            }
        }

        let mut current_number = None;
        loop {
            let Some(byte) = bytes.next() else {
                match current_number {
                    Some(number) => {
                        candidates.push(number).unwrap();
                    }
                    None => return None,
                }
                break;
            };
            match (current_number, byte) {
                (None, b' ') => {}
                (None, b'\n') => {
                    break;
                }
                (Some(number), b' ') => {
                    candidates.push(number).unwrap();
                    current_number = None;
                }
                (Some(number), b'\n') => {
                    candidates.push(number).unwrap();
                    break;
                }
                (None, b'0'..=b'9') => {
                    current_number = Some((byte - b'0') as u64);
                }
                (Some(number), b'0'..=b'9') => {
                    current_number = Some(number * 10 + (byte - b'0') as u64);
                }
                _ => {
                    panic!("Invalid byte: {}", byte as char);
                }
            }
        }

        Some(Self { total, candidates })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    End,
}

impl Operation {
    fn cycle(&mut self) {
        *self = match self {
            Self::Add => Self::Multiply,
            Self::Multiply => Self::End,
            Self::End => {
                panic!("Do not cycle en end !")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Solution {
    operations: [Operation; 16],
    total: u64,
    current: usize,
}

impl Solution {
    fn backtrack(&mut self, line: &Line) {
        self.operations[self.current] = Operation::Add;

        self.current -= 1;
        self.total = match self.operations[self.current] {
            Operation::Add => self.total - line.candidates[self.current],
            Operation::Multiply => self.total / line.candidates[self.current],
            Operation::End => {
                panic!("Invalid state in backtrack.");
            }
        };
        self.operations[self.current].cycle();
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total = 0u64;

    let mut input = input.as_bytes();
    let mut bytes_enumerator = input.into_iter().copied();

    while let Some(line) = Line::parse(&mut bytes_enumerator) {
        let mut current_solution = Solution {
            operations: [Operation::Add; 16],
            total: 0,
            current: 0,
        };
        let max_length = line.candidates.len();
        loop {
            if current_solution.current >= max_length {
                if current_solution.total == line.total {
                    total += line.total;
                    break;
                }
                current_solution.backtrack(&line);
                continue;
            }

            if current_solution.total > line.total
                || current_solution.operations[current_solution.current] == Operation::End
            {
                if current_solution.current == 0 {
                    break;
                }
                current_solution.backtrack(&line);
                continue;
            }

            match current_solution.operations[current_solution.current] {
                Operation::Add => {
                    current_solution.total += line.candidates[current_solution.current];
                }
                Operation::Multiply => {
                    current_solution.total *= line.candidates[current_solution.current];
                }
                Operation::End => {
                    panic!("Invalid state");
                }
            }
            current_solution.current += 1;
        }
    }

    total
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operation2 {
    Add,
    Multiply,
    Concatenate,
    End,
}

impl Operation2 {
    fn cycle(&mut self) {
        *self = match self {
            Self::Add => Self::Multiply,
            Self::Multiply => Self::Concatenate,
            Self::Concatenate => Self::End,
            Self::End => {
                panic!("Do not cycle en end !")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Solution2 {
    operations: [Operation2; 16],
    total: u64,
    current: usize,
}

impl Solution2 {
    fn backtrack(&mut self, line: &Line) {
        self.operations[self.current] = Operation2::Add;

        self.current -= 1;
        self.total = match self.operations[self.current] {
            Operation2::Add => self.total - line.candidates[self.current],
            Operation2::Multiply => self.total / line.candidates[self.current],
            Operation2::Concatenate => {
                let mut sub_total = self.total - line.candidates[self.current];
                let mut current_number = line.candidates[self.current];
                loop {
                    sub_total /= 10;
                    current_number /= 10;
                    if current_number == 0 {
                        break;
                    }
                }
                sub_total
            }
            Operation2::End => {
                panic!("Invalid state in backtrack.");
            }
        };
        self.operations[self.current].cycle();
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total = 0u64;

    let mut input = input.as_bytes();
    let mut bytes_enumerator = input.into_iter().copied();

    while let Some(line) = Line::parse(&mut bytes_enumerator) {
        let mut current_solution = Solution2 {
            operations: [Operation2::Add; 16],
            total: 0,
            current: 0,
        };
        let max_length = line.candidates.len();
        loop {
            if current_solution.current >= max_length {
                if current_solution.total == line.total {
                    total += line.total;
                    break;
                }
                current_solution.backtrack(&line);
                continue;
            }

            if current_solution.total > line.total
                || current_solution.operations[current_solution.current] == Operation2::End
            {
                if current_solution.current == 0 {
                    break;
                }
                current_solution.backtrack(&line);
                continue;
            }

            match current_solution.operations[current_solution.current] {
                Operation2::Add => {
                    current_solution.total += line.candidates[current_solution.current];
                }
                Operation2::Multiply => {
                    current_solution.total *= line.candidates[current_solution.current];
                }
                Operation2::Concatenate => {
                    let mut sub_total = current_solution.total;
                    let mut current_number = line.candidates[current_solution.current];
                    loop {
                        sub_total *= 10;
                        current_number /= 10;
                        if current_number == 0 {
                            break;
                        }
                    }
                    current_solution.total = sub_total + line.candidates[current_solution.current];
                }
                Operation2::End => {
                    panic!("Invalid state");
                }
            }
            current_solution.current += 1;
        }
    }

    total
}
