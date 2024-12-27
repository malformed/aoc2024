use std::io::BufRead;
use std::io::Read;

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

    pub fn read_all(&mut self) -> String {
        let mut buffer = String::new();
        self.reader.read_to_string(&mut buffer).unwrap();
        buffer
    }

    pub fn read_line_as_bytes_into(&mut self, buffer: &mut Vec<u8>) -> Option<()> {
        match self.reader.read_until(b'\n', buffer) {
            Ok(0) => None,
            Err(_) => None,
            _ => Some(()),
        }
    }
}
