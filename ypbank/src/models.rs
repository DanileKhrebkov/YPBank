//! Модели данных для финансовых транзакций

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Тип транзакции, определяющий направление движения средств
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Доход - поступление средств (например, зарплата)
    Income,
    /// Расход - списание средств (например, покупки)
    Expense,
    /// Перевод - перемещение между счетами
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

/// Запись о финансовой транзакции
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции
    pub id: u32,
    /// Дата совершения операции
    pub date: NaiveDate,
    /// Сумма операции (положительное число)
    pub amount: f64,
    /// Тип операции (доход/расход/перевод)
    pub transaction_type: TransactionType,
    /// Описание транзакции
    pub description: String,
    /// Контрагент (компания или человек)
    pub counterparty: String,
}

impl Transaction {
    /// Создаёт новую транзакцию с указанными параметрами
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

/// Запись в бинарном формате (упакованная структура для эффективного хранения)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryRecord {
    /// Уникальный идентификатор транзакции
    pub id: u32,
    /// Временная метка (Unix timestamp)
    pub timestamp: i64,
    /// Сумма операции
    pub amount: f64,
    /// Тип операции (0=Income, 1=Expense, 2=Transfer)
    pub transaction_type: u8,
}

/// Коллекция транзакций, представляющая полный отчет
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionRecord {
    /// Вектор всех транзакций в отчете
    pub transactions: Vec<Transaction>,
}

impl TransactionRecord {
    /// Создаёт пустую коллекцию транзакций
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    /// Добавляет транзакцию в коллекцию
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Возвращает количество транзакций в коллекции
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Проверяет, пуста ли коллекция
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

impl Default for TransactionRecord {
    fn default() -> Self {
        Self::new()
    }
}