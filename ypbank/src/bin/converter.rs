//! Консольная утилита для конвертации финансовых данных между форматами

use std::fs::File;
use std::io::{self, stdin, BufReader, Read, Write};
use std::path::PathBuf;
use std::process;

use clap::{Parser, ValueEnum};
use ypbank_parser::error::ParserError;
use ypbank_parser::parsers::*;
use ypbank_parser::{TransactionParser, TransactionSerializer, TransactionRecord};

#[derive(Debug, ValueEnum, Clone)]
enum Format {
    Csv,
    Text,
    Binary,
}

#[derive(Parser)]
#[command(name = "ypbank_converter")]
#[command(about = "Конвертирует финансовые данные между различными форматами", long_about = None)]
struct Cli {
    /// Входной файл (если не указан, читает из stdin)
    #[arg(short, long)]
    input: Option<PathBuf>,
    
    /// Формат входного файла
    #[arg(short, long, value_enum)]
    input_format: Format,
    
    /// Формат выходного файла
    #[arg(short, long, value_enum)]
    output_format: Format,
}

// Используем enum вместо Box<dyn Trait>
enum ParserEnum {
    Csv(CsvParser),
    Text(TextParser),
    Binary(BinaryParser),
}

impl TransactionParser for ParserEnum {
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord, ParserError> {
        match self {
            ParserEnum::Csv(p) => p.parse(reader),
            ParserEnum::Text(p) => p.parse(reader),
            ParserEnum::Binary(p) => p.parse(reader),
        }
    }
}

enum SerializerEnum {
    Csv(CsvParser),
    Text(TextParser),
    Binary(BinaryParser),
}

impl TransactionSerializer for SerializerEnum {
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<(), ParserError> {
        match self {
            SerializerEnum::Csv(p) => p.serialize(writer, transactions),
            SerializerEnum::Text(p) => p.serialize(writer, transactions),
            SerializerEnum::Binary(p) => p.serialize(writer, transactions),
        }
    }
}

fn get_parser(format: Format) -> ParserEnum {
    match format {
        Format::Csv => ParserEnum::Csv(CsvParser::default()),
        Format::Text => ParserEnum::Text(TextParser::default()),
        Format::Binary => ParserEnum::Binary(BinaryParser::default()),
    }
}

fn get_serializer(format: Format) -> SerializerEnum {
    match format {
        Format::Csv => SerializerEnum::Csv(CsvParser::default()),
        Format::Text => SerializerEnum::Text(TextParser::default()),
        Format::Binary => SerializerEnum::Binary(BinaryParser::default()),
    }
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run_converter(&cli) {
        eprintln!("Ошибка: {}", e);
        process::exit(1);
    }
}

fn run_converter(cli: &Cli) -> Result<(), ParserError> {
    // Чтение данных
    let mut reader: Box<dyn Read> = if let Some(input_path) = &cli.input {
        let file = File::open(input_path)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin())
    };
    
    let parser = get_parser(cli.input_format.clone());
    let transactions = parser.parse(&mut *reader)?;
    
    if transactions.is_empty() {
        eprintln!("Предупреждение: не найдено ни одной транзакции");
    }
    
    // Запись данных
    let serializer = get_serializer(cli.output_format.clone());
    let mut stdout = io::stdout();
    serializer.serialize(&mut stdout, &transactions)?;
    
    eprintln!("Успешно сконвертировано {} транзакций", transactions.len());
    
    Ok(())
}