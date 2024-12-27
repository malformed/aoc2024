use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

use log::info;

type Columns = [u8; 5];

enum Item {
    Lock(Columns),
    Key(Columns),
}

impl Item {
    fn from_buffer(buffer: Vec<[u8; 5]>) -> Self {
        let mut columns = [0; 5];

        for i in 0..5 {
            let mut height = 0;
            for y in 0..5 {
                if buffer[y + 1][i] == b'#' {
                    height += 1;
                };
            }
            columns[i] = height;
        }

        if &buffer[0] == b"#####" {
            Item::Lock(columns)
        } else {
            Item::Key(columns)
        }
    }
}

struct LockKeyInputParser {
    input: Input,
}

impl LockKeyInputParser {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn parse_item(&mut self) -> Option<Item> {
        let mut buffer: Vec<[u8; 5]> = Vec::new();

        while let Some(line) = self.input.read_line() {
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            buffer.push(line.as_bytes().try_into().expect("5 bytes per input line"));
        }

        if buffer.is_empty() {
            None
        } else {
            Some(Item::from_buffer(buffer))
        }
    }
}

struct CodeChronicle {
    locks: Vec<Columns>,
    keys: Vec<Columns>,
}

impl CodeChronicle {
    fn from_input(input: Input) -> Self {
        let mut parser = LockKeyInputParser::new(input);

        let mut locks = Vec::new();
        let mut keys = Vec::new();

        while let Some(item) = parser.parse_item() {
            match item {
                Item::Lock(columns) => locks.push(columns),
                Item::Key(columns) => keys.push(columns),
            }
        }

        Self { locks, keys }
    }

    fn matches(key: &Columns, lock: &Columns) -> bool {
        key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
    }

    fn match_keys_and_locks(&self) -> usize {
        let mut matches = 0;
        for key in &self.keys {
            for lock in &self.locks {
                if Self::matches(key, lock) {
                    info!("Match: {:?} {:?}", key, lock);
                    matches += 1;
                }
            }
        }
        matches
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let chronicle = CodeChronicle::from_input(input);

    let result = match part {
        day::Part::One => chronicle.match_keys_and_locks(),
        day::Part::Two => 0,
    } as i64;

    Ok(result)
}

day_tests!("day_25-1.dat", 3021, 0);
