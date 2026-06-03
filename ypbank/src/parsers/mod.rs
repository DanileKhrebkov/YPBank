use crate::error::Result;
use crate::models::TransactionRecord;
use std::io::{Read, Write};

pub mod csv_parser;
pub mod text_parser;
pub mod bin_parser;

pub use csv_parser::CsvParser;
pub use text_parser::TextParser;
pub use bin_parser::BinaryParser;

pub trait TransactionParser: Send + Sync {
    fn parse<R: Read>(&self, reader: &mut R) -> Result<TransactionRecord>;
}

pub trait TransactionSerializer: Send + Sync {
    fn serialize<W: Write>(
        &self,
        writer: &mut W,
        transactions: &TransactionRecord,
    ) -> Result<()>;
}