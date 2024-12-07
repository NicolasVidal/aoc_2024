#[derive(Debug)]
struct Line {
    total: u64,
    candidates: heapless::Vec<u64, 16>,
}

impl Line {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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

    let input = input.as_bytes();
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
    #[inline(always)]
    fn init(prev_total: Option<u64>, num: u64) -> Self {
        match (prev_total, num) {
            (Some(x1), x2) if x1 + x2 > x1 * x2 => Self::Multiply,
            _ => Self::Add,
        }
    }

    #[inline(always)]
    fn cycle(&mut self, prev_total: Option<u64>, num: u64) {
        *self = match (prev_total, num) {
            (Some(x1), x2) if x1 + x2 > x1 * x2 => match self {
                Self::Multiply => Self::Add,
                Self::Add => Self::Concatenate,
                Self::Concatenate => Self::End,
                Self::End => {
                    panic!("Do not cycle en end !")
                }
            },
            _ => match self {
                Self::Add => Self::Multiply,
                Self::Multiply => Self::Concatenate,
                Self::Concatenate => Self::End,
                Self::End => {
                    panic!("Do not cycle en end !")
                }
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Solution2 {
    number_digits: [u8; 16],
    operations: [Operation2; 16],
    total: u64,
    current: usize,
}

impl Solution2 {
    #[inline(always)]
    fn backtrack(&mut self, line: &Line) {
        self.current -= 1;
        self.total = match self.operations[self.current] {
            Operation2::Add => self.total - line.candidates[self.current],
            Operation2::Multiply => self.total / line.candidates[self.current],
            Operation2::Concatenate => {
                self.total / 10u64.pow(self.number_digits[self.current] as u32)
            }
            Operation2::End => {
                panic!("Invalid state in backtrack.");
            }
        };
        let prev = if self.current == 0 {
            None
        } else {
            Some(self.total)
        };
        self.operations[self.current].cycle(prev, line.candidates[self.current]);
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total = 0u64;

    let input = input.as_bytes();
    let mut bytes_enumerator = input.into_iter().copied();

    while let Some(line) = Line::parse(&mut bytes_enumerator) {
        let mut current_solution = Solution2 {
            number_digits: std::array::from_fn(|i| {
                if i >= line.candidates.len() {
                    return 0;
                }
                let mut number = line.candidates[i];
                let mut num_digits = 0;
                loop {
                    num_digits += 1;
                    number /= 10;
                    if number == 0 {
                        break;
                    }
                }
                num_digits
            }),
            operations: [Operation2::Add; 16],
            total: 0,
            current: 0,
        };
        current_solution.operations[0] = Operation2::init(None, line.candidates[0]);
        let max_length = line.candidates.len();
        let mut first_found = false;
        loop {
            // if current_solution.total > line.total {
            //     dbg!(line);
            //     dbg!(current_solution);
            //     exit(0);
            //     break;
            // }

            if current_solution.current >= max_length {
                if current_solution.total == line.total {
                    total += line.total;
                    break;
                }
                if current_solution.total < line.total {
                    first_found = true;
                }
                if !first_found {
                    break;
                }

                current_solution.backtrack(&line);
                continue;
            }

            if current_solution.total > line.total && !first_found {
                break;
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
                    sub_total *=
                        10u64.pow(current_solution.number_digits[current_solution.current] as u32);
                    current_solution.total = sub_total + line.candidates[current_solution.current];
                }
                Operation2::End => {
                    panic!("Invalid state");
                }
            }
            current_solution.current += 1;
            if current_solution.current < max_length {
                current_solution.operations[current_solution.current] = Operation2::init(
                    Some(current_solution.total),
                    line.candidates[current_solution.current],
                );
            }
        }
    }

    total
}
