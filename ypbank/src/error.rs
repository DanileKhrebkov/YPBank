use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    #[error("Binary parsing error: {0}")]
    BinaryParse(String),
    
    #[error("Date parsing error: {0}")]
    DateParse(String),
    
    #[error("Number parsing error: {0}")]
    NumberParse(String),
    
    #[error("Missing field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, ParserError>;