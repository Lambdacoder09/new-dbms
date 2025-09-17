use crate::db::{DB, Table};
use std::collections::HashMap;

static mut TRANSACTION_BACKUP: Option<HashMap<String, Table>> = None;

pub fn begin_transaction() {
    unsafe {
        if TRANSACTION_BACKUP.is_some() {
            println!("Transaction already in progress!");
        } else {
            let mut backup = HashMap::new();
            for table_name in DB::list_tables() {
                if let Some(table) = DB::load_table(&table_name) {
                    backup.insert(table_name.clone(), table);
                }
            }
            TRANSACTION_BACKUP = Some(backup);
            println!("Transaction started.");
        }
    }
}

pub fn commit() {
    unsafe {
        if TRANSACTION_BACKUP.is_none() {
            println!("No transaction in progress!");
        } else {
            TRANSACTION_BACKUP = None;
            println!("Transaction committed.");
        }
    }
}

pub fn rollback() {
    unsafe {
        if let Some(backup) = &TRANSACTION_BACKUP {
            for (table_name, table) in backup.iter() {
                DB::save_table(table);
            }
            TRANSACTION_BACKUP = None;
            println!("Transaction rolled back.");
        } else {
            println!("No transaction in progress!");
        }
    }
}
