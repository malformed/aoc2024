use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

#[allow(unused_imports)]
use log::info;

#[allow(unreachable_code, unused_variables)]
pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let result = match part {
        day::Part::One => 0,
        day::Part::Two => 0,
    } as i64;

    Ok(result)
}

day_tests!("day_0-1.dat", 0, 0);
