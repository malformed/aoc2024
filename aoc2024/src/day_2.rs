use crate::day;
use crate::error::Result;
use crate::input::Input;

use log::info;

use std::io;
use std::iter::Peekable;

struct InputReader {
    input: Input,
}

impl InputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Iterator for InputReader {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.input
            .read_line()?
            .split_whitespace()
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<i64>>>()
    }
}

type Record = Vec<i64>;

struct PairwiseEnumerator<I: Iterator> {
    iter: Peekable<I>,
    index: usize,
}

impl<I: Iterator> PairwiseEnumerator<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            index: 0,
        }
    }
}

impl<I: Iterator<Item = i64>> Iterator for PairwiseEnumerator<I> {
    type Item = (usize, (i64, i64));

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.peek()?;
        let i = self.index;
        self.index += 1;
        Some((i, (first, *second)))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Monotonicity {
    Increasing,
    Decreasing,
}

impl TryFrom<(i64, i64)> for Monotonicity {
    type Error = ();

    fn try_from((a, b): (i64, i64)) -> Result<Self, Self::Error> {
        let diff = b - a;
        let dist = diff.abs();

        if dist < 1 || dist > 3 {
            return Err(());
        }

        if diff > 0 {
            Ok(Monotonicity::Increasing)
        } else {
            Ok(Monotonicity::Decreasing)
        }
    }
}

struct RecordCheckError {
    index: usize,
}

struct UnusualData {
    records: Vec<Record>,
}

impl UnusualData {
    fn new(input: Input) -> Self {
        let records = InputReader::new(input).collect();
        Self { records }
    }

    fn record_it_with_item_skip(
        record: &[i64],
        skip_index: usize,
    ) -> PairwiseEnumerator<impl Iterator<Item = i64> + '_> {
        let (left, right) = record.split_at(skip_index);
        let right = &right[1..];

        PairwiseEnumerator::new(left.iter().chain(right.iter()).copied())
    }

    fn check_record_inner<I>(
        mut iter: PairwiseEnumerator<I>,
    ) -> Result<Monotonicity, RecordCheckError>
    where
        I: Iterator<Item = i64>,
    {
        let err = |index| RecordCheckError { index };

        let (_, (first, second)) = iter.next().ok_or(RecordCheckError { index: 0 })?;

        let record_kind =
            Monotonicity::try_from((first, second)).map_err(|_| RecordCheckError { index: 0 })?;

        for (index, (a, b)) in iter {
            let next_kind = Monotonicity::try_from((a, b)).map_err(|_| err(index))?;

            if next_kind != record_kind {
                return Err(err(index));
            }
        }

        Ok(record_kind)
    }

    fn check_record(
        record: &Vec<i64>,
        skip_item: Option<usize>,
    ) -> Result<Monotonicity, RecordCheckError> {
        match skip_item {
            Some(index) => Self::check_record_inner(Self::record_it_with_item_skip(record, index)),
            None => Self::check_record_inner(PairwiseEnumerator::new(record.iter().copied())),
        }
    }

    fn check_record_simple(record: &Vec<i64>) -> bool {
        Self::check_record(record, None).is_ok()
    }

    fn check_record_fault_tolerant(record: &Vec<i64>) -> bool {
        let failure_index = match Self::check_record(record, None) {
            Ok(_) => return true,
            Err(RecordCheckError { index }) => {
                if index == record.len() - 1 {
                    return true;
                }
                index
            }
        };

        Self::check_record(record, Some(failure_index))
            .or_else(|_| Self::check_record(record, Some(failure_index + 1)))
            .or_else(|_| Self::check_record(record, Some(0)))
            .or_else(|_| Self::check_record(record, Some(1)))
            .is_ok()
    }

    // task #1
    fn count_valid_records(&self) -> usize {
        self.records.iter().fold(0, |acc, r| {
            acc + if Self::check_record_simple(r) { 1 } else { 0 }
        })
    }

    // task #2
    fn count_valid_records_with_fault_tolerance(&self) -> usize {
        self.records.iter().fold(0, |acc, r| {
            acc + if Self::check_record_fault_tolerant(r) {
                1
            } else {
                0
            }
        })
    }
}

#[allow(unreachable_code, unused_variables, unused_mut)]
pub fn run(mut input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let data = UnusualData::new(input);

    match part {
        day::Part::One => {
            writeln!(output, "{}", data.count_valid_records())?;
        }
        day::Part::Two => {
            writeln!(
                output,
                "{}",
                data.count_valid_records_with_fault_tolerance()
            )?;
        }
    }

    info!("Day done ✅");
    Ok(())
}
