use crate::day;
use crate::error::{Error, Result};
use crate::input::Input;

use log::info;

use std::io;

pub fn run(mut input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let line = input.read_line().ok_or(Error::InvalidInput)?;

    match part {
        day::Part::One => {
            writeln!(output, "{}", line.len())?;
        }
        day::Part::Two => {
            writeln!(output, "{}", 2 * line.len())?;
        }
    }

    info!("Day #{} done", 0);

    Ok(())
}
