use std::collections::HashMap;

use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

type Pattern = Vec<u8>;
type Design = Vec<u8>;

struct InputReader {
    input: Input,
}

impl InputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn read_patterns(&mut self) -> Vec<Pattern> {
        self.input
            .read_line()
            .unwrap()
            .trim()
            .split(", ")
            .map(|s| s.as_bytes().to_vec())
            .collect()
    }

    fn skip_line(&mut self) {
        self.input.read_line().unwrap();
    }

    fn read_designs(self) -> Vec<Design> {
        self.input
            .lines()
            .map(|line| line.unwrap().as_bytes().to_vec())
            .collect()
    }
}

type Cache<'a> = HashMap<&'a [u8], i64>;

struct OnsenTowels {
    patterns: Vec<Pattern>,
    designs: Vec<Design>,
}

impl OnsenTowels {
    fn new(input: Input) -> Self {
        let mut reader = InputReader::new(input);

        let patterns = reader.read_patterns();
        reader.skip_line();
        let designs = reader.read_designs();

        Self { patterns, designs }
    }

    fn _print(slice: &[u8]) {
        for c in slice {
            print!("{}", *c as char);
        }
    }

    fn test_all<'a>(&self, design: &'a [u8], cache: &mut Cache<'a>) -> i64 {
        if design.is_empty() {
            return 1;
        }

        let mut total = 0;

        for pattern in &self.patterns {
            if design.len() < pattern.len() {
                continue;
            }

            let (head, tail) = design.split_at(pattern.len());

            if head == pattern {
                let count = if let Some(&x) = cache.get(tail) {
                    x
                } else {
                    let x = self.test_all(tail, cache);
                    if x > 0 {
                        cache.insert(tail, x);
                    }
                    x
                };

                total += count;
            }
        }

        total
    }

    fn match_designs(&self) -> Vec<usize> {
        self.designs
            .iter()
            .map(|design| {
                let mut cache = Cache::new();
                self.test_all(design, &mut cache) as usize
            })
            .collect::<Vec<_>>()
    }

    fn count_feasible_designs(&self) -> usize {
        self.match_designs().iter().filter(|&x| *x > 0).count()
    }

    fn count_all_arrangements(&self) -> usize {
        self.match_designs().iter().sum()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let towels = OnsenTowels::new(input);

    let result = match part {
        day::Part::One => towels.count_feasible_designs(),
        day::Part::Two => towels.count_all_arrangements(),
    } as i64;

    Ok(result)
}

day_tests!("day_19-1.dat", 358, 600639829400603);
