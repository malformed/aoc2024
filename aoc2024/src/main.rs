mod day;
mod error;
mod input;
mod util;

mod day_0;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use crate::error::{ArgumentError, Error, Result};
use crate::input::Input;
use crate::util::{construct_filename, parse_day, parse_part, validate_day, validate_part};

use env_logger;
use log::info;
use std::env;

fn run(day: u8, part: day::Part, input_file: Option<String>) -> Result<()> {
    let input = Input::from_file(&input_file.unwrap_or_else(|| construct_filename(day, part)))
        .or_else(|_| Input::from_file(&"/dev/stdin"))?;

    info!("Day {day}|{part} 🎄");

    let result = match day {
        0 => day_0::run(input, part),
        1 => day_1::run(input, part),
        2 => day_2::run(input, part),
        3 => day_3::run(input, part),
        4 => day_4::run(input, part),
        5 => day_5::run(input, part),
        6 => day_6::run(input, part),
        7 => day_7::run(input, part),
        8 => day_8::run(input, part),
        9 => day_9::run(input, part),
        _ => Err(Error::DayNotImplemented(day)),
    }?;
    println!("{}", result);

    info!("Day {day}|{part} done 🌟");

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let mut args = env::args().skip(1);

    let day = args
        .next()
        .ok_or(ArgumentError::MissingArgument("day").into())
        .and_then(parse_day)
        .and_then(validate_day)?;

    let part = args
        .next()
        .or(Some("0".to_string()))
        .ok_or(ArgumentError::MissingArgument("part").into())
        .and_then(parse_part)
        .and_then(validate_part)?;

    let infile = args.next();

    run(day, part, infile)
}
