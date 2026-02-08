/*use bank_system::balance::balance_manager::BalanceManager;
use bank_system::users::user_manager::UserManager;*/
use bank_system::{Transaction};
use bank_system::{Deposit, Name, Storage, Transfer, Withdraw};
use std::io::{self, BufRead, Write};

fn main() {
    let mut storage = Storage::load_data("balance.csv");

    println!("=== Bank CLI Utils ===");
    println!("Команды:");
    println!("  add <name> <balance>      - добавить пользователя");
    println!("  remove <name>             - удалить пользователя");
    println!("  deposit <name> <amount>   - пополнить баланс");
    println!("  withdraw <name> <amount>  - снять со счёта");
    println!("  transfer <name_from>\
                <name_to> <amount>        - перевести со счёта одного пользователя другому");
    println!("  balance <name>            - показать баланс");
    println!("  list                      - показать список пользователей");
    println!("  exit                      - выйти");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap(); // показываем приглашение

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break; // EOF
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        const FILE_NAME: &str = "balance.csv";

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример: add John 100");
                    continue;
                }
                let name: Name = args[1].to_string();
                let balance: i64 = match args[2].parse() {
                    Ok(b) => b,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                if storage.add_user(name.clone()).is_some() {
                    let _ = storage.deposit(&name, balance);
                    println!("Пользователь {} добавлен с балансом {}", name, balance);
                    storage.save(FILE_NAME);
                } else {
                    println!("Пользователь {} уже существует", name);
                }
            }
            "remove" => {
                if args.len() != 2 {
                    println!("Пример: remove John");
                    continue;
                }
                let name = args[1];
                if storage.remove_user(&name.to_string()).is_some() {
                    println!("Пользователь {} удалён", name);
                    storage.save(FILE_NAME);
                } else {
                    println!("Пользователь {} не найден", name);
                }
            }
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример: deposit John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let tx = Deposit {
                    account: name.clone(),
                    amount,
                };
                // Применяем транзакцию
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: депозит {} на {}", name, amount);
                        storage.save(FILE_NAME);
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            }
            "wd" => {
                if args.len() != 3 {
                    println!("Пример: wd John 100");
                    continue;
                }

                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let withdraw_tx = Withdraw::new(name, amount);

                match withdraw_tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Вывод средств прошел успешно.");
                        storage.save(FILE_NAME);
                    },
                    Err(e) => { eprintln!("Ошибка транзакции: {}", e) }
                }
            }
            "withdraw" => {
                if args.len() != 3 {
                    println!("Пример: withdraw John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                match storage.withdraw(&name, amount) {
                    Ok(_) => {
                        println!("С баланса пользователя {} снято {}", name, amount);
                        storage.save(FILE_NAME);
                    }
                    Err(e) => println!("Ошибка: {}", e),
                }
            }
            "balance" => {
                if args.len() != 2 {
                    println!("Пример: balance John");
                    continue;
                }
                // ваш код здесь
                let name = args[1].to_string();
                match storage.get_balance(&name.to_string()) {
                    Some(balance) => {
                        println!("Пользователь {} имеет на балансе следующую сумму: {}", name, balance);
                    }
                    None => println!("Данный пользователь не найден в БД"),
                }
            },
            "transfer" => {
                if args.len() != 4 {
                    println!("Пример: transfer Alice Bob 50");
                    continue;
                }
                let from = args[1].to_string();
                let to = args[2].to_string();
                let amount: i64 = match args[3].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let tx = Transfer::new(from, to, amount);
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("{}", tx);
                        storage.save(FILE_NAME);
                    },
                    Err(e) => { eprintln!("Ошибка транзакции: {}", e) }
                }
            },
            "list" => {
                if storage.accounts.len() == 0 {
                    println!("Пользователи отсутствуют");
                    continue;
                }

                println!("Список пользователей:");
                storage.accounts.iter().for_each(|(name, balance)| println!("{} --> {}", name, balance));
            },
            "+" => {
                if args.len() != 8 {
                    println!(
                        "Пример: + deposit Alice 100 transfer Alice Bob 30: cur {}",
                        args.len()
                    );
                    continue;
                }

                let deposit = Deposit {
                    account: args[2].to_string(),
                    amount: args[3].parse().unwrap_or(0),
                };

                let transfer = Transfer::new(args[5].to_string(), args[6].to_string(), args[7].parse().unwrap_or(0));
                // Здесь мы используем оператор +
                let combined_tx = deposit + transfer;

                match combined_tx.apply(&mut storage) {
                    Ok(_) => println!("Транзакции выполнены!"),
                    Err(e) => println!("Ошибка при выполнении: {:?}", e),
                }

                storage.save(FILE_NAME);
            },
            "exit" => break,
            _ => println!("Неизвестная команда"),
        }
    }

    println!("Выход из CLI, все изменения сохранены.");
}