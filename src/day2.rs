use std::iter::Iterator;

#[derive(Debug, Default)]
struct State {
    prev_byte: Option<u8>,
    first: Option<u8>,
    second: Option<u8>,
}

impl State {
    #[inline(always)]
    fn consume_byte(&mut self, byte: u8) -> Result<(), ()> {
        match byte {
            b'0'..=b'9' => {
                let number = byte - b'0';
                if let Some(prev_byte) = self.prev_byte {
                    let number = prev_byte * 10 + number;
                    self.prev_byte = None;
                    self.consume_number(number)
                } else {
                    self.prev_byte = Some(number);
                    Ok(())
                }
            }
            b' ' => {
                if let Some(prev_byte) = self.prev_byte {
                    let number = prev_byte;
                    self.prev_byte = None;
                    self.consume_number(number)
                } else {
                    Ok(())
                }
            }
            _ => {
                panic!("Invalid byte: {}", byte);
            }
        }
    }

    #[inline(always)]
    fn consume_number(&mut self, number: u8) -> Result<(), ()> {
        match (self.first, self.second) {
            (None, None) => {
                self.first = Some(number);
                Ok(())
            }
            (Some(first), None) => {
                self.second = Some(number);
                let diff = first.abs_diff(number);
                if !(1..=3).contains(&diff) {
                    Err(())
                } else {
                    Ok(())
                }
            }
            (Some(first), Some(second)) => {
                if first.cmp(&second) != second.cmp(&number) {
                    return Err(());
                }

                self.first = Some(second);
                self.second = Some(number);
                let diff = second.abs_diff(number);
                if !(1..=3).contains(&diff) {
                    Err(())
                } else {
                    Ok(())
                }
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.prev_byte = None;
        self.first = None;
        self.second = None;
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_reports = 0u32;
    let input = input.as_bytes();
    let mut byte_iter = input.iter();

    let mut state: State = Default::default();

    loop {
        match byte_iter.next() {
            None => {
                if state.first.is_some() {
                    safe_reports += 1;
                }
                break;
            }
            Some(&b'\n') | Some(&b'\r') => {
                if state.first.is_some() {
                    safe_reports += 1;
                }
                state.reset();
            }
            Some(&byte) => {
                if let Err(()) = state.consume_byte(byte) {
                    state.reset();
                    for &byte in byte_iter.by_ref() {
                        if byte == b'\n' || byte == b'\r' {
                            break;
                        }
                    }
                }
            }
        }
    }

    safe_reports
}

#[derive(Debug, Default)]
struct State2 {
    prev_byte: Option<u8>,
    first: Option<u8>,
    second: Option<u8>,
    third: Option<u8>,
    dampened: bool,
}

impl State2 {
    #[inline(always)]
    fn consume_byte(&mut self, byte: u8) -> Result<(), ()> {
        match byte {
            b'0'..=b'9' => {
                let number = byte - b'0';
                if let Some(prev_byte) = self.prev_byte {
                    let number = prev_byte * 10 + number;
                    self.prev_byte = None;
                    self.consume_number(number)
                } else {
                    self.prev_byte = Some(number);
                    Ok(())
                }
            }
            b' ' => {
                if let Some(prev_byte) = self.prev_byte {
                    let number = prev_byte;
                    self.prev_byte = None;
                    self.consume_number(number)
                } else {
                    Ok(())
                }
            }
            _ => {
                panic!("Invalid byte: {}", byte);
            }
        }
    }

    #[inline(always)]
    fn consume_number(&mut self, number: u8) -> Result<(), ()> {
        match (self.first, self.second, self.third) {
            (None, None, None) => {
                self.first = Some(number);
                Ok(())
            }
            (Some(first), None, None) => {
                self.second = Some(number);

                if self.dampened {
                    let mut state = State::default();
                    state.consume_number(first)?;
                    state.consume_number(number)?;
                }

                Ok(())
            }
            (Some(first), Some(second), None) => {
                self.third = Some(number);

                if self.dampened {
                    let mut state = State::default();
                    state.consume_number(first)?;
                    state.consume_number(second)?;
                    state.consume_number(number)?;
                }

                Ok(())
            }
            (Some(first), Some(second), Some(third)) => {
                let fourth = number;

                if self.dampened {
                    let mut state = State::default();
                    state.consume_number(first)?;
                    state.consume_number(second)?;
                    state.consume_number(third)?;
                    state.consume_number(fourth)?;

                    self.first = Some(second);
                    self.second = Some(third);
                    self.third = Some(fourth);

                    return Ok(());
                }

                let mut all = State::default();
                if all.consume_number(first).is_ok()
                    && all.consume_number(second).is_ok()
                    && all.consume_number(third).is_ok()
                    && all.consume_number(fourth).is_ok()
                {
                    self.first = Some(second);
                    self.second = Some(third);
                    self.third = Some(fourth);

                    return Ok(());
                }

                self.dampened = true;

                let mut no_first = State::default();
                if no_first.consume_number(second).is_ok()
                    && no_first.consume_number(third).is_ok()
                    && no_first.consume_number(fourth).is_ok()
                {
                    self.first = Some(second);
                    self.second = Some(third);
                    self.third = Some(fourth);

                    return Ok(());
                }

                let mut no_second = State::default();
                if no_second.consume_number(first).is_ok()
                    && no_second.consume_number(third).is_ok()
                    && no_second.consume_number(fourth).is_ok()
                {
                    self.first = Some(first);
                    self.second = Some(third);
                    self.third = Some(fourth);

                    return Ok(());
                }

                let mut no_third = State::default();
                if no_third.consume_number(first).is_ok()
                    && no_third.consume_number(second).is_ok()
                    && no_third.consume_number(fourth).is_ok()
                {
                    self.first = Some(first);
                    self.second = Some(second);
                    self.third = Some(fourth);

                    return Ok(());
                }

                let mut no_fourth = State::default();
                if no_fourth.consume_number(first).is_ok()
                    && no_fourth.consume_number(second).is_ok()
                    && no_fourth.consume_number(third).is_ok()
                {
                    self.first = Some(first);
                    self.second = Some(second);
                    self.third = Some(third);

                    return Ok(());
                }

                Err(())
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.prev_byte = None;
        self.first = None;
        self.second = None;
        self.third = None;
        self.dampened = false;
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_reports = 0u32;
    let input = input.as_bytes();
    let mut byte_iter = input.iter();

    let mut state: State2 = Default::default();

    loop {
        match byte_iter.next() {
            None => {
                if state.first.is_some() {
                    safe_reports += 1;
                }
                break;
            }
            Some(&b'\n') | Some(&b'\r') => {
                if state.first.is_some() {
                    safe_reports += 1;
                }
                state.reset();
            }
            Some(&byte) => {
                if let Err(()) = state.consume_byte(byte) {
                    state.reset();
                    for &byte in byte_iter.by_ref() {
                        if byte == b'\n' || byte == b'\r' {
                            break;
                        }
                    }
                }
            }
        }
    }

    safe_reports
}
