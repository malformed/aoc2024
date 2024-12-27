use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

use std::collections::BTreeMap;

struct InputReader {
    input: Input,
}

impl InputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Iterator for InputReader {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.input.read_line()?;
        let mut parts = line.split_whitespace();
        let a = parts.next()?.parse().ok()?;
        let b = parts.next()?.parse().ok()?;
        Some((a, b))
    }
}

struct Locations {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Locations {
    fn new(input: Input) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for (a, b) in InputReader::new(input) {
            left.push(a);
            right.push(b);
        }

        left.sort();
        right.sort();

        Self { left, right }
    }

    // task #1
    fn lists_distance(&self) -> i64 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    // task #2
    fn similarity_score(&self) -> i64 {
        let freq = Self::build_frequency_map_2(&self.right);

        self.left
            .iter()
            .map(|a| *a * freq.get(a).unwrap_or(&0))
            .sum()
    }

    // build frequency map version 1
    #[allow(dead_code)]
    fn build_frequency_map_1(list: &Vec<i64>) -> BTreeMap<i64, i64> {
        list.iter()
            .fold(Vec::<(i64, i64)>::new(), |mut acc, x| {
                if let Some((value, freq)) = acc.last_mut() {
                    if *value == *x {
                        *freq += 1;
                    } else {
                        acc.push((*x, 1));
                    }
                } else {
                    acc.push((*x, 1));
                }
                return acc;
            })
            .into_iter()
            .collect()
    }

    // build frequency map version 2
    #[allow(dead_code)]
    fn build_frequency_map_2(list: &Vec<i64>) -> BTreeMap<i64, i64> {
        let mut freq = BTreeMap::new();
        for x in list {
            freq.entry(*x).and_modify(|e| *e += 1).or_insert(1);
        }
        freq
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let locations = Locations::new(input);

    let result = match part {
        day::Part::One => locations.lists_distance(),
        day::Part::Two => locations.similarity_score(),
    };

    Ok(result)
}

day_tests!("day_1-1.dat", 1938424, 22014209);
