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
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord>;
}

pub trait TransactionSerializer: Send + Sync {
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<()>;
}