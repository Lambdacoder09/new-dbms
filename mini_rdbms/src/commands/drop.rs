use crate::db::DB;
use std::fs;

pub fn drop_table(query: &str) {
    let parts: Vec<&str> = query.trim().split_whitespace().collect();
    if parts.len() < 3 {
        println!("Invalid DROP syntax!");
        return;
    }

    let table_name = parts[2];
    let path = DB::table_path(table_name);
    if fs::remove_file(&path).is_ok() {
        println!("Table '{}' dropped successfully!", table_name);
    } else {
        println!("Table '{}' does not exist!", table_name);
    }
}
