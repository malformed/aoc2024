#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Argument error: {0}")]
    Argument(#[from] ArgumentError),

    #[error("Solution for day {0} not implemented yet")]
    DayNotImplemented(u8),

    #[error("Input file not found: {0}")]
    InputFileNotFound(String),

    // derived errors
    #[error("I/O error: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
pub enum ArgumentError {
    #[error("Missing argument")]
    MissingArgument(&'static str),

    #[error("Invalid day input: {0}")]
    InvalidDayInput(String),

    #[error("Invalid day: {0}")]
    InvalidDay(u8),

    #[error("Invalid part argument: {0}")]
    InvalidPartArgument(String),

    #[error("Part out of range: {0}")]
    PartOutOfRange(u8),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
