//! Модели данных для финансовых транзакций

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Тип транзакции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Income,   
    Expense,  
    Transfer, 
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Income => write!(f, "Income"),
            TransactionType::Expense => write!(f, "Expense"),
            TransactionType::Transfer => write!(f, "Transfer"),
        }
    }
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(TransactionType::Income),
            "expense" => Ok(TransactionType::Expense),
            "transfer" => Ok(TransactionType::Transfer),
            _ => Err(format!("Unknown transaction type: {}", s)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub date: NaiveDate,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub description: String,
    pub counterparty: String,
}

impl Transaction {
    pub fn new(
        id: u32,
        date: NaiveDate,
        amount: f64,
        transaction_type: TransactionType,
        description: String,
        counterparty: String,
    ) -> Self {
        Self {
            id,
            date,
            amount,
            transaction_type,
            description,
            counterparty,
        }
    }
}

/// Запись в бинарном формате
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryRecord {
    pub id: u32,
    pub timestamp: i64,
    pub amount: f64,
    pub transaction_type: u8,
}


#[derive(Debug, Clone, PartialEq)]
pub struct TransactionRecord {
    pub transactions: Vec<Transaction>,
}

impl TransactionRecord {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

impl Default for TransactionRecord {
    fn default() -> Self {
        Self::new()
    }
}