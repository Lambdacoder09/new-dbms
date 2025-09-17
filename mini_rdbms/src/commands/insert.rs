use crate::db::{DB};

pub fn insert(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 4 {
        println!("Invalid INSERT syntax!");
        return;
    }

    let table_name = parts[2];
    let mut table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' not found!", table_name);
            return;
        }
    };

    // Parse values between VALUES (...)
    let vals_start = query.find('(').unwrap_or(0);
    let vals_end = query.find(')').unwrap_or(query.len());
    let vals_str = &query[vals_start + 1..vals_end];
    let values = vals_str
        .split(',')
        .map(|v| v.trim().to_string())
        .collect::<Vec<_>>();

    if values.len() != table.columns.len() {
        println!("Column count doesn't match value count!");
        return;
    }

    table.rows.push(values);
    DB::save_table(&table);
    println!("1 row inserted into '{}'", table_name);
}
