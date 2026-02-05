pub mod storage {
    use std::collections::HashMap;
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
    }
}