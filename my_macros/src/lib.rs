// bank-system/my_macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use proc_macro2::TokenStream as TokenStream2;

/// Наш процедурный макрос: принимает строку и печатает её в коде
#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    let msg = parse_macro_input!(input as syn::LitStr); // ожидаем строковый литерал
    let expanded = quote! {
        println!("{}", #msg);
    };
    expanded.into()
}

#[proc_macro_derive(Transaction, attributes(transaction))]
pub fn transaction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // По умолчанию — deposit
    let mut kind = "deposit";

    for attr in &input.attrs {
        if attr.path().is_ident("transaction") {
            // Разбираем атрибут как Meta
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
                let val = meta.value();
                if val == "withdraw" {
                    kind = "withdraw";
                } else if val == "transfer" {
                    kind = "transfer";
                }
            }
        }
    }

    let body = match kind {
        "deposit" => quote! {
            *storage.accounts.entry(self.account.clone()).or_insert(0) += self.amount;
        },
        "withdraw" => quote! {
            let bal = storage.accounts.entry(self.account.clone()).or_insert(0);
            if *bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *bal -= self.amount;
        },
        "transfer" => quote! {
            let from_bal = storage.accounts.entry(self.from.clone()).or_insert(0);
            if *from_bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *from_bal -= self.amount;
            *storage.accounts.entry(self.to.clone()).or_insert(0) += self.amount;
        },
        _ => panic!("Unknown transaction kind"),
    };

    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
                #body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToSql)]
pub fn to_sql_derive(input: TokenStream) -> TokenStream {
    // Парсим вход в proc_macro2 TokenStream
    let input: DeriveInput = parse_macro_input!(input);
    let name = input.ident;

    let (field_names, field_values): (Vec<_>, Vec<_>) = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    (ident, quote! { self.#ident })
                })
                .unzip(),
            _ => panic!("ToSql can only be derived for structs with named fields"),
        },
        _ => panic!("ToSql can only be derived for structs"),
    };

    // Генерация кода с proc_macro2 + quote
    let expanded: TokenStream2 = quote! {
        impl #name {
            pub fn to_sql(&self, table: &str) -> String {
                let columns = vec![#(stringify!(#field_names)),*].join(", ");
                let values = vec![#(format!("'{}'", #field_values)),*].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", table, columns, values)
            }
        }
    };

    println!("{expanded}",);

    // Преобразуем proc_macro2::TokenStream обратно в proc_macro::TokenStream
    TokenStream::from(expanded)
}
