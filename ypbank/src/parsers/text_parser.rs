//! Текстовый формат парсера для банковских операций

use crate::error::{ParserError, Result};
use crate::models::{Transaction, TransactionRecord, TransactionType};
use crate::parsers::{TransactionParser, TransactionSerializer};
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Debug, Clone, Default)]
pub struct TextParser;

impl TransactionParser for TextParser {
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord> {
        let buf_reader = BufReader::new(reader);
        let mut transactions = TransactionRecord::new();
        
        for (line_num, line) in buf_reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 6 {
                return Err(ParserError::InvalidFormat(
                    format!("Line {}: expected 6 fields, found {}", line_num + 1, parts.len())
                ));
            }
            
            let id: u32 = parts[0].trim().parse()
                .map_err(|e| ParserError::NumberParse(format!("Invalid id at line {}: {}", line_num + 1, e)))?;
            
            let date = chrono::NaiveDate::parse_from_str(parts[1].trim(), "%d.%m.%Y")
                .map_err(|e| ParserError::DateParse(format!("Invalid date at line {}: {}", line_num + 1, e)))?;
            
            let amount: f64 = parts[2].trim().parse()
                .map_err(|e| ParserError::NumberParse(format!("Invalid amount at line {}: {}", line_num + 1, e)))?;
            
            let transaction_type: TransactionType = parts[3].trim().parse()
                .map_err(|e| ParserError::InvalidFormat(format!("Invalid type at line {}: {}", line_num + 1, e)))?;
            
            let description = parts[4].trim().to_string();
            let counterparty = parts[5].trim().to_string();
            
            transactions.add_transaction(Transaction::new(
                id, date, amount, transaction_type, description, counterparty
            ));
        }
        
        Ok(transactions)
    }
}

impl TransactionSerializer for TextParser {
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<()> {
        for transaction in &transactions.transactions {
            let line = format!(
                "{}|{}|{}|{}|{}|{}\n",
                transaction.id,
                transaction.date.format("%d.%m.%Y"),
                transaction.amount,
                transaction.transaction_type,
                transaction.description,
                transaction.counterparty
            );
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_parse_and_serialize() {
        let text_data = "1|15.01.2024|100.50|Income|Salary|Employer Inc\n\
                         2|16.01.2024|25.75|Expense|Coffee|Starbucks\n";
        
        let parser = TextParser;
        let mut reader = text_data.as_bytes();
        let result = parser.parse(&mut reader).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.transactions[0].date.format("%d.%m.%Y").to_string(), "15.01.2024");
        
        let mut output = Vec::new();
        parser.serialize(&mut output, &result).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("100.5"));
    }
}