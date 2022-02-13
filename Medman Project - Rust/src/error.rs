use std::io;
use std::fmt;

#[derive(Debug)]

pub enum ComError {
  IoError(io::Error),
  NoDirectory,
  InvalidCommand,
  InvalidPath,
}

impl fmt::Display for ComError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      ComError::IoError(ref e) => e.fmt(formatter),
      ComError::NoDirectory => formatter.write_str("No Directory Found"),
      ComError::InvalidCommand =>formatter.write_str("The Command is Invalid"),
      ComError::InvalidPath =>formatter.write_str("The Path is not Valid"),
    }
  }
}

impl From<io::Error> for ComError {
  fn from(err: io::Error) -> ComError {
    ComError::IoError(err)
  }
}