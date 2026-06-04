use crate::error::{ParserError, Result};
use crate::models::{BinaryRecord, Transaction, TransactionRecord, TransactionType};
use crate::parsers::{TransactionParser, TransactionSerializer};
use std::io::{Read, Write};
use std::mem;

const MAGIC_NUMBER: u32 = 0x5950424B; 
const VERSION: u16 = 1;

#[derive(Debug, Clone, Default)]
pub struct BinaryParser;

impl TransactionParser for BinaryParser {
    fn parse(&self, reader: &mut dyn Read) -> Result<TransactionRecord> {
        let mut transactions = TransactionRecord::new();

        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        let magic_num = u32::from_le_bytes(magic);
        
        if magic_num != MAGIC_NUMBER {
            return Err(ParserError::BinaryParse(
                format!("Invalid magic number: {:X}", magic_num)
            ));
        }
        
        let mut version = [0u8; 2];
        reader.read_exact(&mut version)?;
        let version_num = u16::from_le_bytes(version);
        
        if version_num != VERSION {
            return Err(ParserError::BinaryParse(
                format!("Unsupported version: {}", version_num)
            ));
        }
        
        let mut count = [0u8; 4];
        reader.read_exact(&mut count)?;
        let record_count = u32::from_le_bytes(count);

        for _ in 0..record_count {
            let mut record_buf = [0u8; mem::size_of::<BinaryRecord>()];
            reader.read_exact(&mut record_buf)?;
            
            let binary_record: BinaryRecord = unsafe {
                std::ptr::read(record_buf.as_ptr() as *const BinaryRecord)
            };

            let timestamp = chrono::DateTime::from_timestamp(binary_record.timestamp, 0)
    .ok_or_else(|| ParserError::BinaryParse("Invalid timestamp".to_string()))?
    .naive_local();
            let transaction_type = match binary_record.transaction_type {
                0 => TransactionType::Income,
                1 => TransactionType::Expense,
                2 => TransactionType::Transfer,
                _ => return Err(ParserError::BinaryParse("Invalid transaction type".to_string())),
            };
            
            transactions.add_transaction(Transaction::new(
                binary_record.id,
                timestamp.date(),
                binary_record.amount,
                transaction_type,
                format!("Binary transaction {}", binary_record.id),
                "Unknown".to_string(),
            ));
        }
        
        Ok(transactions)
    }
}

impl TransactionSerializer for BinaryParser {
    fn serialize(
        &self,
        writer: &mut dyn Write,
        transactions: &TransactionRecord,
    ) -> Result<()> {
        writer.write_all(&MAGIC_NUMBER.to_le_bytes())?;
        writer.write_all(&VERSION.to_le_bytes())?;
        writer.write_all(&(transactions.len() as u32).to_le_bytes())?;

        for transaction in &transactions.transactions {
            let transaction_type = match transaction.transaction_type {
                TransactionType::Income => 0u8,
                TransactionType::Expense => 1u8,
                TransactionType::Transfer => 2u8,
            };
            
            let datetime = transaction.date.and_hms_opt(0, 0, 0)
                .ok_or_else(|| ParserError::BinaryParse("Invalid datetime".to_string()))?;
            
            let binary_record = BinaryRecord {
                id: transaction.id,
                timestamp: datetime.and_utc().timestamp(),
                amount: transaction.amount,
                transaction_type,
            };
            
            let record_bytes = unsafe {
                std::slice::from_raw_parts(
                    &binary_record as *const BinaryRecord as *const u8,
                    std::mem::size_of::<BinaryRecord>()
                )
            };
            writer.write_all(record_bytes)?;
        }
        
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    #[test]
    fn test_binary_parse_and_serialize() {
        let mut original = TransactionRecord::new();
        original.add_transaction(Transaction::new(
            1,
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            100.50,
            TransactionType::Income,
            "Salary".to_string(),
            "Employer".to_string(),
        ));
        
        let parser = BinaryParser;
        let mut buffer = Vec::new();
        parser.serialize(&mut buffer, &original).unwrap();
        
        let mut reader = buffer.as_slice();
        let parsed = parser.parse(&mut reader).unwrap();
        
        assert_eq!(original.len(), parsed.len());
        assert_eq!(original.transactions[0].id, parsed.transactions[0].id);
        assert_eq!(original.transactions[0].amount, parsed.transactions[0].amount);
    }
}