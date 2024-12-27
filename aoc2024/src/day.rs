use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

#[macro_export]
macro_rules! day_tests {
    ($day:expr, $part1_result:expr, $part2_result:expr) => {
        #[cfg(test)]
        mod test {
            use super::*;

            fn input() -> Input {
                crate::input::Input::from_file(format!("input/{}", $day).as_str()).unwrap()
            }

            #[test]
            fn part_one() {
                let result = run(input(), day::Part::One).unwrap();
                assert_eq!(result, $part1_result);
            }

            #[test]
            fn part_two() {
                let result = run(input(), day::Part::Two).unwrap();
                assert_eq!(result, $part2_result);
            }
        }
    };
}
