pub mod transaction {
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::ops::Add;
    use crate::Storage;

    #[derive(Debug)]
    pub enum TxError {
        InsufficientFunds,
        InvalidAccount,
    }

    impl Display for TxError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TxError::InsufficientFunds => { write!(f, "Не хватает денег на балансе") },
                TxError::InvalidAccount => {write!(f, "Неверный аккаунт") }
            }
        }
    }

    impl Error for TxError {}

    pub struct TxCombinator<T1: Transaction, T2: Transaction> {
        t1: T1,
        t2: T2,
    }

    pub trait Transaction {
        fn apply(&self, accounts: &mut Storage) -> Result<(), TxError>;
    }

    impl<T1: Transaction, T2: Transaction> Transaction for TxCombinator<T1, T2> {
        fn apply(&self, accounts: &mut Storage) -> Result<(), TxError> {
            self.t1.apply(accounts)?;
            self.t2.apply(accounts)?;
            Ok(())
        }
    }

    pub struct Deposit {
        pub account: String,
        pub amount: i64,
    }

    impl Transaction for Deposit {
        fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
            *storage.accounts.entry(self.account.clone()).or_insert(0) += self.amount;
            Ok(())
        }
    }

    impl Add<Transfer> for Deposit {
        type Output = TxCombinator<Deposit, Transfer>;

        fn add(self, rhs: Transfer) -> Self::Output {
            TxCombinator { t1: self, t2: rhs }
        }
    }

    pub struct Transfer {
        pub from: String,
        pub to: String,
        pub amount: i64,
    }

    impl Display for Transfer {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Транзакция: перевод {} -> {} на {}", self.from, self.to, self.amount)
        }
    }

    impl Transaction for Transfer {
        fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
            let Some(_) = storage.accounts.get(&self.to) else { return Err(TxError::InvalidAccount) };
            let Some(from_balance) = storage.accounts.get_mut(&self.from) else { return Err(TxError::InvalidAccount) };
            if *from_balance < self.amount { return Err(TxError::InsufficientFunds); }

            *from_balance -= self.amount;
            *storage.accounts.get_mut(&self.to).unwrap() += self.amount;
            Ok(())
        }
    }

    impl Add<Deposit> for Transfer {
        type Output = TxCombinator<Transfer, Deposit>;

        fn add(self, rhs: Deposit) -> Self::Output {
            TxCombinator { t1: self, t2: rhs }
        }
    }

    pub struct Withdraw {
        pub account: String,
        pub amount: i64,
    }

    impl Transaction for Withdraw {
        fn apply(&self, accounts: &mut Storage) -> Result<(), TxError> {
            let Some(balance) = accounts.accounts.get_mut(&self.account) else { return Err(TxError::InvalidAccount) };
            if *balance < self.amount { return Err(TxError::InsufficientFunds); }

            *balance -= self.amount;
            Ok(())
        }
    }

    impl Withdraw {
        pub fn new(account: String, amount: i64) -> Withdraw {
            Withdraw { account, amount}
        }
    }
}