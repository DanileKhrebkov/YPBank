//! Консольная утилита для сравнения финансовых данных из двух файлов

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process;

use clap::{Parser, ValueEnum};
use ypbank_parser::error::ParserError;
use ypbank_parser::parsers::*;
use ypbank_parser::{Transaction, TransactionParser, TransactionRecord};

#[derive(Debug, ValueEnum, Clone)]
enum Format {
    Csv,
    Text,
    Binary,
}

#[derive(Parser)]
#[command(name = "ypbank_compare")]
#[command(about = "Сравнивает финансовые данные из двух файлов", long_about = None)]
struct Cli {
    /// Первый файл для сравнения
    #[arg(short, long)]
    file1: PathBuf,
    
    /// Формат первого файла
    #[arg(short, long, value_enum)]
    format1: Format,
    
    /// Второй файл для сравнения
    #[arg(short, long)]
    file2: PathBuf,
    
    /// Формат второго файла
    #[arg(short, long, value_enum)]
    format2: Format,
    
    /// Игнорировать порядок транзакций при сравнении
    #[arg(long)]
    ignore_order: bool,
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

fn get_parser(format: Format) -> ParserEnum {
    match format {
        Format::Csv => ParserEnum::Csv(CsvParser::default()),
        Format::Text => ParserEnum::Text(TextParser::default()),
        Format::Binary => ParserEnum::Binary(BinaryParser::default()),
    }
}

fn compare_transactions(
    t1: &Transaction,
    t2: &Transaction,
) -> Vec<String> {
    let mut differences = Vec::new();
    
    if t1.id != t2.id {
        differences.push(format!("  ID: {} vs {}", t1.id, t2.id));
    }
    if t1.date != t2.date {
        differences.push(format!("  Date: {} vs {}", t1.date, t2.date));
    }
    if (t1.amount - t2.amount).abs() > 0.0001 {
        differences.push(format!("  Amount: {} vs {}", t1.amount, t2.amount));
    }
    if t1.transaction_type != t2.transaction_type {
        differences.push(format!("  Type: {:?} vs {:?}", t1.transaction_type, t2.transaction_type));
    }
    if t1.description != t2.description {
        differences.push(format!("  Description: '{}' vs '{}'", t1.description, t2.description));
    }
    if t1.counterparty != t2.counterparty {
        differences.push(format!("  Counterparty: '{}' vs '{}'", t1.counterparty, t2.counterparty));
    }
    
    differences
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run_comparer(&cli) {
        eprintln!("Ошибка: {}", e);
        process::exit(1);
    }
}

fn run_comparer(cli: &Cli) -> Result<(), ParserError> {
    // Чтение первого файла
    let file1 = File::open(&cli.file1)?;
    let mut reader1 = BufReader::new(file1);
    let parser1 = get_parser(cli.format1.clone());
    let transactions1 = parser1.parse(&mut reader1)?;
    
    // Чтение второго файла
    let file2 = File::open(&cli.file2)?;
    let mut reader2 = BufReader::new(file2);
    let parser2 = get_parser(cli.format2.clone());
    let transactions2 = parser2.parse(&mut reader2)?;
    
    // Сравнение
    if transactions1.len() != transactions2.len() {
        println!(
            "Файлы различаются: количество транзакций {} vs {}",
            transactions1.len(),
            transactions2.len()
        );
        return Err(ParserError::InvalidFormat(
            format!("Record count mismatch: {} vs {}", transactions1.len(), transactions2.len())
        ));
    }
    
    let mut differences_found = false;
    
    if cli.ignore_order {
        // Сортируем транзакции для сравнения без учета порядка
        let mut sorted1 = transactions1.transactions.clone();
        let mut sorted2 = transactions2.transactions.clone();
        sorted1.sort_by_key(|t| t.id);
        sorted2.sort_by_key(|t| t.id);
        
        for (t1, t2) in sorted1.iter().zip(sorted2.iter()) {
            let differences = compare_transactions(t1, t2);
            if !differences.is_empty() {
                differences_found = true;
                println!("Транзакция {} отличается:", t1.id);
                for diff in differences {
                    println!("{}", diff);
                }
            }
        }
    } else {
        // Сравниваем в том же порядке
        for (i, (t1, t2)) in transactions1.transactions.iter()
            .zip(transactions2.transactions.iter())
            .enumerate() {
            let differences = compare_transactions(t1, t2);
            if !differences.is_empty() {
                differences_found = true;
                println!("Транзакция #{} отличается:", i + 1);
                for diff in differences {
                    println!("{}", diff);
                }
            }
        }
    }
    
    if !differences_found {
        println!(
            "Транзакции в файлах '{}' и '{}' идентичны.",
            cli.file1.display(),
            cli.file2.display()
        );
        Ok(())
    } else {
        Err(ParserError::InvalidFormat("Найдены различия в транзакциях".to_string()))
    }
}