use crate::db::DB;
use std::collections::HashMap;

// Simple in-memory index: column value -> row positions
pub fn create_index(query: &str) {
    let parts: Vec<&str> = query.trim().split_whitespace().collect();
    if parts.len() < 6 {
        println!("Invalid CREATE INDEX syntax!");
        return;
    }

    let table_name = parts[4];
    let column_name = parts[5].trim_matches(|c| c == '(' || c == ')');

    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' does not exist!", table_name);
            return;
        }
    };

    let col_index = match table.columns.iter().position(|c| c == column_name) {
        Some(idx) => idx,
        None => {
            println!("Column '{}' does not exist in table '{}'", column_name, table_name);
            return;
        }
    };

    let mut index_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, row) in table.rows.iter().enumerate() {
        index_map.entry(row[col_index].clone()).or_default().push(i);
    }

    println!("Index on column '{}' of table '{}' created!", column_name, table_name);
    // Currently in-memory only; can be extended to persist indexes
}
