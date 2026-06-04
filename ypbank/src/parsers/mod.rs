//! Модуль парсеров для различных форматов данных

use crate::error::Result;
use crate::models::TransactionRecord;
use std::io::{Read, Write};

pub mod csv_parser;
pub mod text_parser;
pub mod bin_parser;

pub use csv_parser::CsvParser;
pub use text_parser::TextParser;
pub use bin_parser::BinaryParser;

/// Трейт для парсинга транзакций из потока чтения
/// Используем ассоциированный тип вместо generic для объектной безопасности
pub trait TransactionParser: Send + Sync {
    /// Парсит транзакции из источника данных
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord>;
}

/// Трейт для сериализации транзакций в поток записи
pub trait TransactionSerializer: Send + Sync {
    /// Сериализует транзакции в приемник данных
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<()>;
}