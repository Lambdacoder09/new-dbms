use crate::db::{DB, Table};
use regex::Regex;

pub fn insert(query: &str) {
    let re = Regex::new(r"(?i)INSERT INTO (\w+)\s+VALUES\s*\((.+)\)").unwrap();
    if let Some(cap) = re.captures(query) {
        let table_name = &cap[1];
        let row_values: Vec<String> = cap[2].split(',')
            .map(|s| s.trim().trim_matches('\'').to_string())
            .collect();

        if let Some(mut table) = DB::load_table(table_name) {
            if row_values.len() != table.columns.len() {
                println!("Column count doesn't match!");
                return;
            }
            table.rows.push(row_values);
            DB::save_table(table_name, &table);
            println!("Row inserted successfully!");
        } else {
            println!("Table '{}' does not exist!", table_name);
        }
    } else {
        println!("Invalid INSERT syntax!");
    }
}
