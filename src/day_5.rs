use crate::error::{Error, Result};
use crate::input::Input;
use crate::{day, day_tests};

use std::borrow::Cow;
use std::num::ParseIntError;

type Rules = Vec<Vec<u8>>;
type Pages = Vec<u8>;

struct PrintIntstructions {
    rules: Rules,
    updates: Vec<Pages>,
}

impl PrintIntstructions {
    fn new(mut input: Input) -> Result<PrintIntstructions> {
        let mut rules = vec![Vec::new(); 100];
        let mut updates = Vec::new();

        while let Some(line) = input.read_line() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let mut parts = line.split("|");

            let left = parts.next().ok_or(Error::InvalidInput())?.parse::<u8>()?;
            let right = parts.next().ok_or(Error::InvalidInput())?.parse::<u8>()?;

            rules[left as usize].push(right);
        }

        for line in input.read_all().lines() {
            let pages = line
                .trim()
                .split(",")
                .map(|page| page.parse::<u8>())
                .collect::<Result<Pages, ParseIntError>>()?;

            updates.push(pages);
        }

        Ok(PrintIntstructions { rules, updates })
    }

    fn validate_page_order<'a>(&self, pages: &'a Pages, do_fix: bool) -> (bool, Cow<'a, Pages>) {
        let mut pages = Cow::Borrowed(pages);
        let mut valid = true;

        for i in 0..pages.len() {
            let page = pages[i] as usize;
            // rules where page shows up on the left side
            let rules = &self.rules[page];

            // check pages up to i, if any is in the rule at the right side
            for j in 0..i {
                let left_page = pages[j] as u8;
                if rules.contains(&left_page) {
                    valid = false;

                    if do_fix {
                        pages.to_mut().swap(i, j);
                    } else {
                        return (valid, pages);
                    }
                }
            }
        }
        (valid, pages)
    }

    fn middle_page(pages: &Pages) -> u64 {
        let middle = pages.len() / 2;
        pages[middle] as u64
    }

    // Task #1
    fn valid_pages_metric(&self) -> u64 {
        self.updates
            .iter()
            .map(|update| match self.validate_page_order(update, false) {
                (true, _) => Self::middle_page(update),
                (false, _) => 0,
            })
            .sum()
    }

    // Task #2
    fn fixed_invalid_pages_metric(&self) -> u64 {
        self.updates
            .iter()
            .map(|update| match self.validate_page_order(update, true) {
                (true, _) => 0,
                (false, fixed_pages) => Self::middle_page(&fixed_pages),
            })
            .sum()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let instructions = PrintIntstructions::new(input)?;

    let result = match part {
        day::Part::One => instructions.valid_pages_metric(),
        day::Part::Two => instructions.fixed_invalid_pages_metric(),
    } as i64;

    Ok(result)
}

day_tests!("day_5-1.dat", 5275, 6191);
