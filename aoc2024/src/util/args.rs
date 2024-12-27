use crate::day;
use crate::error::ArgumentError;

pub fn validate_day(day: u8) -> Result<u8, ArgumentError> {
    if day > 24 {
        Err(ArgumentError::InvalidDay(day))?
    } else {
        Ok(day)
    }
}

pub fn parse_day(arg: impl AsRef<str>) -> Result<u8, ArgumentError> {
    let day = arg.as_ref();
    day.parse::<u8>()
        .map_err(|_| ArgumentError::InvalidDayInput(day.to_string()))
}

pub fn validate_part(part: u8) -> Result<day::Part, ArgumentError> {
    match part {
        1 => Ok(day::Part::One),
        2 => Ok(day::Part::Two),
        _ => Err(ArgumentError::PartOutOfRange(part)),
    }
}

pub fn parse_part(arg: String) -> Result<u8, ArgumentError> {
    arg.parse::<u8>()
        .map_err(|_| ArgumentError::InvalidPartArgument(arg))
}

pub fn construct_filename(day: u8, part: day::Part) -> String {
    let part = match part {
        day::Part::One => "1",
        day::Part::Two => "2",
    };
    format!("input/day_{day}-{part}.dat")
}
