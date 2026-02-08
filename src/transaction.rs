pub mod transaction {
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use my_macros::Transaction;
    use crate::Storage;
    use crate::impl_add;

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
        pub t1: T1,
        pub t2: T2,
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

    #[derive(Transaction)]
    pub struct Deposit {
        pub account: String,
        pub amount: i64,
    }

    #[derive(Transaction)]
    #[transaction("transfer")]
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

    impl_add! {
        (Deposit, Transfer),
        (Transfer, Deposit),
        (Deposit, Deposit),
        (Transfer, Transfer)
    }

    #[derive(Transaction)]
    #[transaction("withdraw")]
    pub struct Withdraw {
        pub account: String,
        pub amount: i64,
    }

    #[macro_export]
    macro_rules! tx_chain {
        ( $first:expr $(, $rest:expr )* $(,)? ) => {{
            let tx = $first;
            $(
                let tx = $crate::TxCombinator { t1: tx, t2: $rest };
            )*
            tx
        }};
    }

    #[macro_export]
    macro_rules! impl_add {
        ( $( ($lhs:ty, $rhs:ty) ),* ) => {
            $(
                impl std::ops::Add<$rhs> for $lhs {
                    type Output = $crate::TxCombinator<$lhs, $rhs>;

                    fn add(self, rhs: $rhs) -> Self::Output {
                        $crate::TxCombinator { t1: self, t2: rhs }
                    }
                }
            )*
        };
    }
}