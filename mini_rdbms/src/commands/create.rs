use crate::db::{DB, Table};
use regex::Regex;

pub fn create_table(query: &str) {
    let re = Regex::new(r"(?i)CREATE TABLE (\w+)\s*\((.+)\)").unwrap();
    if let Some(cap) = re.captures(query) {
        let table_name = &cap[1];
        let columns: Vec<String> = cap[2].split(',')
            .map(|s| s.trim().to_string())
            .collect();
        if DB::table_path(table_name).as_str().is_empty() {
            println!("Invalid table path!");
            return;
        }
        if DB::load_table(table_name).is_some() {
            println!("Table '{}' already exists!", table_name);
            return;
        }
        let table = Table { columns, rows: vec![] };
        DB::save_table(table_name, &table);
        println!("Table '{}' created!", table_name);
    } else {
        println!("Invalid CREATE TABLE syntax!");
    }
}
