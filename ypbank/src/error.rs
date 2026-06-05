//! Определение типов ошибок для библиотеки парсинга

use std::io;
use thiserror::Error;

/// Основной тип ошибки для операций парсинга и сериализации
#[derive(Error, Debug)]
pub enum ParserError {
    /// Ошибка ввода-вывода (файл не найден, недостаточно прав и т.д.)
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// Ошибка парсинга CSV (неправильный формат, отсутствие заголовков)
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    /// Ошибка конвертации UTF-8 (неверная кодировка)
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    
    /// Данные имеют неправильный формат (не хватает полей, неверный разделитель)
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
    
    /// Ошибка при чтении/записи бинарных данных (неверное магическое число, версия)
    #[error("Binary parsing error: {0}")]
    BinaryParse(String),
    
    /// Ошибка парсинга даты (неверный формат)
    #[error("Date parsing error: {0}")]
    DateParse(String),
    
    /// Ошибка парсинга числа (неверный числовой формат)
    #[error("Number parsing error: {0}")]
    NumberParse(String),
    
    /// Отсутствует обязательное поле в данных
    #[error("Missing field: {0}")]
    MissingField(String),
}

/// Упрощённый тип результата для операций с парсерами
pub type Result<T> = std::result::Result<T, ParserError>;