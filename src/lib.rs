mod storage;

pub use storage::storage::Storage;

use std::fs::File;
use std::{fs, io};
use std::io::BufRead;
use std::path::Path;


pub type Name = String;
pub type Balance = i64;

impl Storage {
    /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        // Проверяем, существует ли файл
        if Path::new(file).exists() {
            // Открываем файл
            let file = File::open(file).unwrap();

            // Оборачиваем файл в BufReader
            // BufReader читает данные блоками и хранит их в буфере,
            // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
            let reader = io::BufReader::new(file);

            // Читаем файл построчно
            for line in reader.lines() {
                // Каждая строка — это Result<String>, поэтому делаем if let Ok
                if let Ok(line) = line {
                    // Разделяем строку по запятой: "Name,Balance"
                    let parts: Vec<&str> = line.trim().split(',').collect();

                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        // Пробуем преобразовать баланс из строки в число
                        let balance: i64 = parts[1].parse().unwrap_or(0);

                        // Добавляем пользователя и выставляем баланс
                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance);
                    }
                }
            }
        } else {
            // если файла нет, создаём пользователей с нуля
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string());
            }
        }

        storage
    }

    /// Сохраняет текущее состояние Storage в CSV-файл
    pub fn save(&self, file: &str) {
        let mut data = String::new();

        // Собираем все данные в одну строку формата "Name,Balance"
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance));
        }

        // Записываем в файл
        // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storage::storage::Storage;

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(0)); // новый пользователь
        assert_eq!(storage.add_user("Alice".to_string()), None); // уже существует
    }

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100).unwrap();

        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(100)); // удаляем и получаем баланс
        assert_eq!(storage.remove_user(&"Bob".to_string()), None); // второй раз — не найден
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        // Депозит несуществующему пользователю
        assert!(storage.deposit(&"Dana".to_string(), 100).is_err());

        // Снятие у несуществующего пользователя
        assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());

        // Баланс у несуществующего пользователя
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }

    use std::io::{BufReader, BufWriter, Cursor, Write};

    #[test]
    fn test_load_data_existing_cursor() {
        // Создаём данные в памяти, как будто это CSV-файл
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        // Читаем данные из Cursor
        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let balance: i64 = parts[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None); // нет в данных
    }

    #[test]
    fn test_save_writes_to_cursor_correctly() {
        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в память через BufWriter
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            for (name, balance) in storage.get_all() {
                writeln!(writer, "{},{}", name, balance).unwrap();
            }
            writer.flush().unwrap();
        }

        // Читаем обратно из памяти
        cursor.set_position(0);
        let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
        lines.sort(); // сортируем для сравнения

        assert_eq!(lines, vec!["Alice,300", "John,150"]);
    }
}