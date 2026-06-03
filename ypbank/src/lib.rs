// Либа для парсинга финансовых данных в разных форматах
pub mod models;

pub use error::ParserError;
pub use models::{Transaction, TransactionType, TransactionRecord};
pub use parsers::{
    CsvParser, TextParser, BinaryParser,
    TransactionParser, TransactionSerializer,
};