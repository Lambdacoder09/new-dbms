use crate::db::{DB};

pub fn delete(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Invalid DELETE syntax!");
        return;
    }

    let table_name = parts[2];
    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' not found!", table_name);
            return;
        }
    };

    let mut table = table;

    // Filter rows by WHERE clause if exists
    let mut remaining_rows = table.rows.clone();
    if query.to_uppercase().contains("WHERE") {
        let where_pos = query.to_uppercase().find("WHERE").unwrap();
        let cond_str = query[where_pos + 5..].trim();
        let cond_parts: Vec<&str> = cond_str.split_whitespace().collect();
        if cond_parts.len() == 3 {
            let col_name = cond_parts[0];
            let operator = cond_parts[1];
            let value = cond_parts[2].trim_matches('\'');

            if let Some(col_index) = table.columns.iter().position(|c| c == col_name) {
                remaining_rows = remaining_rows.into_iter().filter(|row| {
                    let cell = &row[col_index];
                    match operator {
                        "=" => cell != value,
                        ">" => cell.parse::<f64>().unwrap_or(0.0) <= value.parse::<f64>().unwrap_or(0.0),
                        "<" => cell.parse::<f64>().unwrap_or(0.0) >= value.parse::<f64>().unwrap_or(0.0),
                        _ => true,
                    }
                }).collect();
            }
        }
    } else {
        remaining_rows.clear(); // DELETE all
    }

    let deleted_count = table.rows.len() - remaining_rows.len();
    table.rows = remaining_rows;
    DB::save_table(&table);
    println!("Deleted {} rows.", deleted_count);
}