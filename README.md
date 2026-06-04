```markdown
# YPBank - Парсер финансовых данных на Rust

Библиотека для парсинга, сериализации и десериализации финансовых данных с поддержкой трех форматов: CSV, текстовый и бинарный.

## 📋 Содержание

- [Особенности](#особенности)
- [Форматы данных](#форматы-данных)
- [Команды для работы](#команды-для-работы)
- [Примеры использования](#примеры-использования)
- [Структура проекта](#структура-проекта)

## ✨ Особенности

- ✅ Поддержка трех форматов: CSV, текстовый (|), бинарный
- ✅ Использование трейтов `Read` и `Write` для абстракции ввода/вывода
- ✅ Статический полиморфизм через трейты
- ✅ Идиоматичная обработка ошибок без `unwrap`
- ✅ Модульные тесты для всех парсеров
- ✅ CLI утилиты для конвертации и сравнения
- ✅ Документированное публичное API

## 📊 Форматы данных

### CSV формат
```csv
id,date,amount,type,description,counterparty
1,2024-01-15,100.50,Income,Salary,Employer Inc
2,2024-01-16,25.75,Expense,Coffee,Starbucks
```

### Текстовый формат
```
1|15.01.2024|100.50|Income|Salary|Employer Inc
2|16.01.2024|25.75|Expense|Coffee|Starbucks
```

### Бинарный формат
- Магическое число: 4 байта (0x5950424B)
- Версия: 2 байта
- Количество записей: 4 байта
- Записи фиксированного размера (id, timestamp, amount, type)

## 🚀 Команды для работы

### Сборка проекта
```bash
cargo build                                         # Сборка в debug режиме
cargo build --release                              # Сборка в release режиме
cargo build --bin ypbank_converter                 # Сборка только конвертера
cargo build --bin ypbank_compare                   # Сборка только сравнителя
cargo check                                        # Проверка кода без компиляции
cargo tree                                         # Показать дерево зависимостей
```

### Запуск тестов
```bash
cargo test                                         # Запуск всех тестов
cargo test -- --nocapture                          # Запуск с выводом сообщений
cargo test -- --nocapture --show-output            # Запуск с отображением успешных тестов
cargo test test_csv_parse_and_serialize            # Тест CSV парсера
cargo test test_text_parse_and_serialize           # Тест текстового парсера
cargo test test_binary_parse_and_serialize         # Тест бинарного парсера
cargo test --lib                                   # Только тесты библиотеки
cargo test -- --test-threads=1                     # Запуск тестов последовательно
cargo test -- --ignored                            # Запуск игнорируемых тестов
```

### Конвертер (ypbank_converter)
```bash
cargo run --bin ypbank_converter -- --help         # Показать справку
./target/release/ypbank_converter --help           # Справка из release версии

# Конвертация CSV -> Текст
cargo run --bin ypbank_converter -- -f input.csv -i csv -o text

# Конвертация CSV -> Бинарный
cargo run --bin ypbank_converter -- -f input.csv -i csv -o binary > output.bin

# Конвертация Текст -> CSV
cargo run --bin ypbank_converter -- -f input.txt -i text -o csv

# Конвертация Текст -> Бинарный
cargo run --bin ypbank_converter -- -f input.txt -i text -o binary > output.bin

# Конвертация Бинарный -> CSV
cargo run --bin ypbank_converter -- -f input.bin -i binary -o csv

# Конвертация Бинарный -> Текст
cargo run --bin ypbank_converter -- -f input.bin -i binary -o text

# Чтение из stdin и вывод в stdout
cat input.csv | cargo run --bin ypbank_converter -- -i csv -o text

# Использование длинных имен флагов
cargo run --bin ypbank_converter -- --input input.csv --input-format csv --output-format text

# Использование release версии (быстрее)
cargo run --release --bin ypbank_converter -- -f input.csv -i csv -o text
```

### Сравнитель (ypbank_compare)
```bash
cargo run --bin ypbank_compare -- --help           # Показать справку
./target/release/ypbank_compare --help             # Справка из release версии

# Сравнение двух CSV файлов
cargo run --bin ypbank_compare -- -1 file1.csv -a csv -2 file2.csv -b csv

# Сравнение CSV и текстового файлов
cargo run --bin ypbank_compare -- -1 data.csv -a csv -2 data.txt -b text

# Сравнение CSV и бинарного файлов
cargo run --bin ypbank_compare -- -1 data.csv -a csv -2 data.bin -b binary

# Сравнение с игнорированием порядка транзакций
cargo run --bin ypbank_compare -- -1 file1.csv -a csv -2 file2.csv -b csv --ignore-order

# Сравнение текстовых файлов
cargo run --bin ypbank_compare -- -1 data1.txt -a text -2 data2.txt -b text

# Сравнение бинарных файлов
cargo run --bin ypbank_compare -- -1 data1.bin -a binary -2 data2.bin -b binary

# Использование длинных имен флагов
cargo run --bin ypbank_compare -- --file1 file1.csv --format1 csv --file2 file2.csv --format2 csv --ignore-order
```

### Очистка и пересборка
```bash
cargo clean                                       # Очистка собранных файлов
rm Cargo.lock                                     # Удаление Cargo.lock для обновления зависимостей
cargo clean && cargo build                        # Полная пересборка с нуля
cargo update                                      # Обновление зависимостей
cargo build --features "serde"                    # Пересборка с конкретными фичами
```

### Просмотр документации
```bash
cargo doc --open                                  # Сборка и открытие документации
cargo doc --lib --open                            # Сборка документации для библиотеки
cargo doc --no-deps --open                        # Просмотр документации без зависимостей
```

## 📚 Примеры использования

### Создание тестовых данных
```bash
# Создание CSV файла
cat > transactions.csv << 'EOF'
id,date,amount,type,description,counterparty
1,2024-01-15,100.50,Income,Salary,ACME Corp
2,2024-01-16,50.25,Expense,Lunch,Restaurant
3,2024-01-17,200.00,Transfer,Payment,John Doe
EOF

# Создание текстового файла
cat > transactions.txt << 'EOF'
1|15.01.2024|100.50|Income|Salary|ACME Corp
2|16.01.2024|50.25|Expense|Lunch|Restaurant
3|17.01.2024|200.00|Transfer|Payment|John Doe
EOF
```

### Полный рабочий процесс
```bash
# 1. Конвертация CSV в текст
cargo run --bin ypbank_converter -- -f transactions.csv -i csv -o text > output.txt

# 2. Конвертация CSV в бинарный
cargo run --bin ypbank_converter -- -f transactions.csv -i csv -o binary > output.bin

# 3. Сравнение исходного и сконвертированного файлов
cargo run --bin ypbank_compare -- -1 transactions.csv -a csv -2 output.txt -b text

# 4. Проверка бинарного формата
cargo run --bin ypbank_compare -- -1 transactions.csv -a csv -2 output.bin -b binary

# 5. Конвертация обратно из бинарного в CSV
cargo run --bin ypbank_converter -- -f output.bin -i binary -o csv > restored.csv

# 6. Проверка целостности
cargo run --bin ypbank_compare -- -1 transactions.csv -a csv -2 restored.csv -b csv
```

### Отладка и логирование
```bash
# Запуск с детальным выводом ошибок
RUST_BACKTRACE=1 cargo run --bin ypbank_converter -- -f test.csv -i csv -o text

# Запуск с логированием (если добавлено)
RUST_LOG=debug cargo run --bin ypbank_converter -- -f test.csv -i csv -o text

# Просмотр бинарного файла в шестнадцатеричном виде
hexdump -C output.bin | head -20
```

## 📁 Структура проекта
```
ypbank/
├── Cargo.toml                 # Конфигурация проекта и зависимости
├── README.md                  # Документация
├── src/
│   ├── lib.rs                 # Корневой модуль библиотеки
│   ├── error.rs               # Типы ошибок (ParserError)
│   ├── models.rs              # Модели данных (Transaction, TransactionRecord)
│   ├── parsers/               # Модуль парсеров
│   │   ├── mod.rs             # Трейты и объявление модулей
│   │   ├── csv_parser.rs      # CSV парсер/сериализатор
│   │   ├── text_parser.rs     # Текстовый парсер/сериализатор
│   │   └── bin_parser.rs      # Бинарный парсер/сериализатор
│   └── bin/                   # Исполняемые файлы
│       ├── converter.rs       # CLI конвертер
│       └── comparer.rs        # CLI сравнитель
```

## 🔧 Флаги команд

### Флаги конвертера (ypbank_converter)
| Короткий | Длинный | Описание | Обязательный |
|----------|---------|----------|--------------|
| `-f` | `--input` | Входной файл | Нет (по умолчанию stdin) |
| `-i` | `--input-format` | Формат входа (csv/text/binary) | Да |
| `-o` | `--output-format` | Формат выхода (csv/text/binary) | Да |

### Флаги сравнителя (ypbank_compare)
| Короткий | Длинный | Описание | Обязательный |
|----------|---------|----------|--------------|
| `-1` | `--file1` | Первый файл | Да |
| `-a` | `--format1` | Формат первого файла | Да |
| `-2` | `--file2` | Второй файл | Да |
| `-b` | `--format2` | Формат второго файла | Да |
| - | `--ignore-order` | Игнорировать порядок транзакций | Нет |

## 🐛 Устранение неполадок
```bash
# Ошибка "No such file or directory"
ls -la input.csv              # Проверьте существование файла
pwd                           # Проверьте текущую директорию

# Ошибка компиляции
cargo clean                   # Очистите кэш сборки
cargo update                  # Обновите зависимости

# Ошибка с флагами
cargo run --bin ypbank_converter -- --help  # Посмотрите правильные флаги

# Медленная сборка
cargo build --release         # Используйте release режим для оптимизации

# Ошибка парсинга даты
# Убедитесь что формат даты правильный:
# CSV: YYYY-MM-DD
# Текст: DD.MM.YYYY
```

## 📈 Производительность
```bash
# Замер времени конвертации 1 миллиона записей
time cargo run --release --bin ypbank_converter -- -f large.csv -i csv -o text

# Замер потребления памяти (Linux)
/usr/bin/time -v cargo run --release --bin ypbank_converter -- -f large.csv -i csv -o text
```
