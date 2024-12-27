use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PebbleCacheKey {
    pebble: u64,
    blink_count: u64,
}

impl PebbleCacheKey {
    fn new(pebble: u64, blink_count: i64) -> Self {
        Self {
            pebble,
            blink_count: blink_count as u64,
        }
    }
}

struct PlutonianPebbles {
    pebbles: Vec<u64>,
    cache: HashMap<PebbleCacheKey, usize>, // pebble after n blinks -> count
}

impl PlutonianPebbles {
    fn new(mut input: Input) -> Self {
        let pebbles = input
            .read_line()
            .expect("valid input")
            .split_whitespace()
            .map(|s| s.parse().expect("a number"))
            .collect();

        Self {
            pebbles,
            cache: HashMap::new(),
        }
    }

    fn num_digits(n: u64) -> u32 {
        n.ilog10() + 1
    }

    fn apply_rules(pebble: u64) -> (Option<u64>, Option<u64>) {
        match pebble {
            // rule #1: 0 -> 1
            0 => (Some(1), None),

            // rule #2: even number -> split
            p if Self::num_digits(p) % 2 == 0 => {
                let shift = Self::num_digits(p) / 2;

                let left = p / 10u64.pow(shift);
                let right = p % 10u64.pow(shift);

                (Some(left), Some(right))
            }

            // rule #3 odd number -> double
            p => (Some(p * 2024), None),
        }
    }

    fn count_after_n_blinks(&mut self, pebble: u64, n: i64) -> usize {
        if n == 0 {
            return 1;
        }

        let cache_key = PebbleCacheKey::new(pebble, n);

        if let Some(&count) = self.cache.get(&cache_key) {
            return count;
        };

        let count = match Self::apply_rules(pebble) {
            (Some(p1), Some(p2)) => {
                self.count_after_n_blinks(p1, n - 1) + self.count_after_n_blinks(p2, n - 1)
            }
            (Some(p), None) => self.count_after_n_blinks(p, n - 1),
            _ => unreachable!(),
        };

        self.cache.insert(cache_key, count);

        count
    }

    fn count_pebbles_after_blinks(&mut self, blink_count: u64) -> usize {
        self.pebbles
            .clone()
            .iter()
            .map(|&pebble| self.count_after_n_blinks(pebble, blink_count as i64))
            .sum()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut pebbles = PlutonianPebbles::new(input);

    let result = match part {
        day::Part::One => pebbles.count_pebbles_after_blinks(25),
        day::Part::Two => pebbles.count_pebbles_after_blinks(75),
    } as i64;

    Ok(result)
}

day_tests!("day_11-1.dat", 216042, 255758646442399);
