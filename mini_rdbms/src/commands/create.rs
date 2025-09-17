use crate::db::{DB, Table};

pub fn create(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Invalid CREATE TABLE syntax!");
        return;
    }

    let table_name = parts[2];
    if DB::load_table(table_name).is_some() {
        println!("Table '{}' already exists!", table_name);
        return;
    }

    let cols_start = query.find('(').unwrap_or(0);
    let cols_end = query.find(')').unwrap_or(query.len());
    let cols_str = &query[cols_start + 1..cols_end];
    let columns = cols_str
        .split(',')
        .map(|c| c.trim().to_string())
        .collect::<Vec<_>>();

    let table = Table {
        name: table_name.to_string(),
        columns,
        rows: vec![],
    };

    DB::save_table(&table);
    println!("Table '{}' created successfully!", table_name);
}
