//! CSV формат парсера для банковских операций

use crate::error::{ParserError, Result};
use crate::models::{Transaction, TransactionRecord, TransactionType};
use crate::parsers::{TransactionParser, TransactionSerializer};
use csv::{ReaderBuilder, WriterBuilder};
use std::io::{Read, Write};

#[derive(Debug, Clone, Default)]
pub struct CsvParser;

impl TransactionParser for CsvParser {
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);
        
        let mut transactions = TransactionRecord::new();
        
        for result in csv_reader.records() {
            let record = result?;
            
            if record.len() < 6 {
                return Err(ParserError::InvalidFormat(
                    "CSV record has insufficient columns".to_string()
                ));
            }
            
            let id: u32 = record[0].parse()
                .map_err(|e| ParserError::NumberParse(format!("Invalid id: {}", e)))?;
            
            let date = chrono::NaiveDate::parse_from_str(&record[1], "%Y-%m-%d")
                .map_err(|e| ParserError::DateParse(format!("Invalid date: {}", e)))?;
            
            let amount: f64 = record[2].parse()
                .map_err(|e| ParserError::NumberParse(format!("Invalid amount: {}", e)))?;
            
            let transaction_type: TransactionType = record[3].parse()
                .map_err(|e| ParserError::InvalidFormat(e))?;
            
            let description = record[4].to_string();
            let counterparty = record[5].to_string();
            
            transactions.add_transaction(Transaction::new(
                id, date, amount, transaction_type, description, counterparty
            ));
        }
        
        Ok(transactions)
    }
}

impl TransactionSerializer for CsvParser {
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<()> {
        let mut csv_writer = WriterBuilder::new()
            .has_headers(true)
            .from_writer(writer);
        
        // Записываем заголовки
        csv_writer.write_record(&["id", "date", "amount", "type", "description", "counterparty"])?;
        
        // Записываем транзакции
        for transaction in &transactions.transactions {
            csv_writer.write_record(&[
                transaction.id.to_string(),
                transaction.date.format("%Y-%m-%d").to_string(),
                transaction.amount.to_string(),
                transaction.transaction_type.to_string(),
                transaction.description.clone(),
                transaction.counterparty.clone(),
            ])?;
        }
        
        csv_writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_csv_parse_and_serialize() {
        let csv_data = "id,date,amount,type,description,counterparty\n\
                        1,2024-01-15,100.50,Income,Salary,Employer Inc\n\
                        2,2024-01-16,25.75,Expense,Coffee,Starbucks\n";
        
        let parser = CsvParser;
        let mut reader = csv_data.as_bytes();
        let result = parser.parse(&mut reader).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.transactions[0].id, 1);
        assert_eq!(result.transactions[0].amount, 100.50);
        
        // Тест сериализации
        let mut output = Vec::new();
        parser.serialize(&mut output, &result).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("100.5"));
        assert!(output_str.contains("Coffee"));
    }
}