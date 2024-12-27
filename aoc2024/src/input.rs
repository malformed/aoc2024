use std::io::BufRead;

use crate::error::{Error, Result};

pub struct Input {
    reader: std::io::BufReader<std::fs::File>,
}

impl Input {
    pub fn from_file(filename: &str) -> Result<Input> {
        let reader = std::fs::File::open(filename)
            .map_err(|_| Error::InputFileNotFound(filename.to_string()))?;

        Ok(Input {
            reader: std::io::BufReader::new(reader),
        })
    }

    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Err(_) => None,
            _ => Some(line),
        }
    }
}
