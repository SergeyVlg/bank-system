pub mod storage {
    use std::collections::HashMap;
    use std::fs::File;
    use std::{fs, io};
    use std::io::BufRead;
    use std::path::Path;
    use crate::Balance;
    use crate::Name;

    pub struct Storage {
        accounts: HashMap<Name, Balance>,
    }

    impl Storage {
        /// Создаёт новый пустой банк
        pub fn new() -> Self {
            Storage {
                accounts: HashMap::new(),
            }
        }
        pub fn add_user(&mut self, name: Name) -> Option<Balance> {
            if self.accounts.contains_key(&name) {
                None
            } else {
                self.accounts.insert(name, 0);
                Some(0)
            }
        }

        pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
            self.accounts.remove(name)
        }

        pub fn get_balance(&self, name: &Name) -> Option<Balance> {
            self.accounts.get(name).copied()
        }

        pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
            if let Some(balance) = self.accounts.get_mut(name) {
                *balance += amount;
                Ok(())
            } else {
                Err("Пользователь не найден".into())
            }
        }

        pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
            if let Some(balance) = self.accounts.get_mut(name) {
                if *balance >= amount {
                    *balance -= amount;
                    Ok(())
                } else {
                    Err("Недостаточно средств".into())
                }
            } else {
                Err("Пользователь не найден".into())
            }
        }

        pub fn get_all(&self) -> Vec<(Name, i64)> {
            self.accounts.iter().map(|(n, b)| (n.clone(), *b)).collect()
        }

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
}