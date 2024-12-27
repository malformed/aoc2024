mod day;
mod error;
mod input;

mod day_0;

use crate::error::{Error, Result};
use crate::input::Input;

use env_logger;

fn validate_day(day: u8) -> Result<u8> {
    if day > 24 {
        Err(Error::InvalidDay(day))
    } else {
        Ok(day)
    }
}

fn parse_day(arg: impl AsRef<str>) -> Result<u8> {
    let day = arg.as_ref();
    day.parse::<u8>()
        .map_err(|_| error::Error::InvalidDayInput(day.to_string()))
}

fn validate_part(part: u8) -> Result<day::Part> {
    match part {
        1 => Ok(day::Part::One),
        2 => Ok(day::Part::Two),
        _ => Err(Error::InvalidPartArgument),
    }
}

fn parse_part(arg: String) -> Result<u8> {
    arg.parse::<u8>().map_err(|_| Error::InvalidPartArgument)
}

fn input_file(day: u8, part: day::Part) -> String {
    let part = match part {
        day::Part::One => "1",
        day::Part::Two => "2",
    };
    format!("input/day_{day}-{part}.dat")
}

fn run() -> Result<()> {
    env_logger::init();

    let mut args = std::env::args();

    let day = args
        .nth(1)
        .ok_or(Error::MissingArgument("day"))
        .and_then(parse_day)
        .and_then(validate_day)?;

    let part = args
        .next()
        .or(Some("1".to_string()))
        .ok_or(Error::MissingArgument("part"))
        .and_then(parse_part)
        .and_then(validate_part)?;

    let input = crate::Input::from_file(&input_file(day, part))
        .or_else(|_| crate::Input::from_file(&"/dev/stdin"))?;

    let output = std::io::stdout();

    match day {
        0 => day_0::run(input, output, part),
        1 => day_0::run(input, output, part),
        _ => Err(Error::DayNotImplemented(day)),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);

        std::process::exit(1);
    }
}
