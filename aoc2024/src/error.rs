#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing argument")]
    MissingArgument(&'static str),

    #[error("Invalid day input: {0}")]
    InvalidDayInput(String),

    #[error("Invalid day: {0}")]
    InvalidDay(u8),

    #[error("Invalid part argument")]
    InvalidPartArgument,

    #[error("Solution for day {0} not implemented yet")]
    DayNotImplemented(u8),

    #[error("Invalid input")]
    InvalidInput,

    #[error("Input file not found: {0}")]
    InputFileNotFound(String),

    // derived errors
    #[error("I/O error: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
