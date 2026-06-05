//! Библиотека для парсинга финансовых данных в различных форматах
//! 
//! Поддерживаемые форматы:
//! - CSV (YPBankCsv)
//! - Текстовый (YPBankText)
//! - Бинарный (YPBankBin)

pub mod error;
pub mod models;
pub mod parsers;

/// Тип ошибки парсера
pub use error::ParserError;
/// Структуры данных для транзакций
pub use models::{Transaction, TransactionType, TransactionRecord, BinaryRecord};
/// Парсеры и трейты для работы с форматами
pub use parsers::{
    CsvParser, TextParser, BinaryParser,
    TransactionParser, TransactionSerializer,
};