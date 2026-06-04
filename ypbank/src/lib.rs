// Либа для парсинга финансовых данных в разных форматах
pub mod error;
pub mod models;
pub mod parsers;

pub use error::ParserError;
pub use models::{Transaction, TransactionType, TransactionRecord, BinaryRecord};
pub use parsers::{
    CsvParser, TextParser, BinaryParser,
    TransactionParser, TransactionSerializer,
};