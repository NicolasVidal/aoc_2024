
#[derive(Debug)]
enum ConsumerState {
    M,
    U,
    L,
    OpenParenthesis,
    FirstNumber(u32),
    SecondNumber(u32, u32),
    Success(u32),
    Fail,
}

impl ConsumerState {

    #[inline(always)]
    fn consume_byte(self, byte: u8) -> Self {
        match (self, byte) {
            (ConsumerState::M, b'm') => ConsumerState::U,
            (ConsumerState::U, b'u') => ConsumerState::L,
            (ConsumerState::L, b'l') => ConsumerState::OpenParenthesis,
            (ConsumerState::OpenParenthesis, b'(') => ConsumerState::FirstNumber(0),
            (ConsumerState::FirstNumber(number), b'0'..=b'9') => {
                ConsumerState::FirstNumber(number * 10 + (byte - b'0') as u32)
            }
            (ConsumerState::FirstNumber(first), b',') => ConsumerState::SecondNumber(first, 0),
            (ConsumerState::SecondNumber(first, second), b'0'..=b'9') => {
                ConsumerState::SecondNumber(first, second * 10 + (byte - b'0') as u32)
            }
            (ConsumerState::SecondNumber(first, second), b')') => {
                ConsumerState::Success(first * second)
            }
            _ => ConsumerState::Fail,
        }
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut count = 0u32;

    let mut state = ConsumerState::M;
    for byte in bytes {
        state = state.consume_byte(*byte);
        if let ConsumerState::Success(multiplication) = state {
            count += multiplication;
            state = ConsumerState::M;
        } else if let ConsumerState::Fail = state {
            state = ConsumerState::M;
        }
    }

    count
}

#[derive(Debug)]
enum DoConsumer {
    O,
    OpenParenthesis,
    CloseParenthesis,
    Success,
    Fail,
}

impl DoConsumer {

    #[inline(always)]
    fn consume_byte(self, byte: u8) -> Self {
        match (self, byte) {
            (DoConsumer::O, b'o') => DoConsumer::OpenParenthesis,
            (DoConsumer::OpenParenthesis, b'(') => DoConsumer::CloseParenthesis,
            (DoConsumer::CloseParenthesis, b')') => DoConsumer::Success,
            _ => DoConsumer::Fail,
        }
    }
}

#[derive(Debug)]
enum DontConsumer {
    O,
    N,
    Apostrophe,
    T,
    OpenParenthesis,
    CloseParenthesis,
    Success,
    Fail,
}

impl DontConsumer {

    #[inline(always)]
    fn consume_byte(self, byte: u8) -> Self {
        match (self, byte) {
            (DontConsumer::O, b'o') => DontConsumer::N,
            (DontConsumer::N, b'n') => DontConsumer::Apostrophe,
            (DontConsumer::Apostrophe, b'\'') => DontConsumer::T,
            (DontConsumer::T, b't') => DontConsumer::OpenParenthesis,
            (DontConsumer::OpenParenthesis, b'(') => DontConsumer::CloseParenthesis,
            (DontConsumer::CloseParenthesis, b')') => DontConsumer::Success,
            _ => DontConsumer::Fail,
        }
    }
}

#[derive(Debug)]
enum BigConsumer {
    DOrM,
    D,
    ParsingDo(DoConsumer),
    ParsingDont(DontConsumer),
    ParsingConsumer(ConsumerState),
    Success(u32),
}

impl BigConsumer {

    #[inline(always)]
    fn consume_byte(self, byte: u8) -> Self {
        match self {
            BigConsumer::DOrM => {
                if byte == b'd' {
                    BigConsumer::ParsingDont(DontConsumer::O)
                } else if byte == b'm' {
                    BigConsumer::ParsingConsumer(ConsumerState::U)
                } else {
                    BigConsumer::DOrM
                }
            }
            BigConsumer::D => {
                if byte == b'd' {
                    BigConsumer::ParsingDo(DoConsumer::O)
                } else {
                    BigConsumer::D
                }
            }
            BigConsumer::ParsingDo(consumer) => match consumer.consume_byte(byte) {
                DoConsumer::Success => BigConsumer::DOrM,
                DoConsumer::Fail => BigConsumer::D,
                consumer => BigConsumer::ParsingDo(consumer),
            },
            BigConsumer::ParsingDont(consumer) => match consumer.consume_byte(byte) {
                DontConsumer::Success => BigConsumer::D,
                DontConsumer::Fail => BigConsumer::DOrM,
                consumer => BigConsumer::ParsingDont(consumer),
            },
            BigConsumer::ParsingConsumer(consumer) => match consumer.consume_byte(byte) {
                ConsumerState::Success(mul) => BigConsumer::Success(mul),
                ConsumerState::Fail => BigConsumer::DOrM,
                consumer => BigConsumer::ParsingConsumer(consumer),
            },
            BigConsumer::Success(_) => {
                panic!("BigConsumer::consume_byte called on a BigConsumer::Success")
            }
        }
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut count = 0u32;

    let mut state = BigConsumer::DOrM;
    for byte in bytes{
        state = state.consume_byte(*byte);
        if let BigConsumer::Success(multiplication) = state {
            count += multiplication;
            state = BigConsumer::DOrM;
        }
    }

    count
}
