use crate::db::{DB};

pub fn update(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 4 {
        println!("Invalid UPDATE syntax!");
        return;
    }

    let table_name = parts[1];
    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' not found!", table_name);
            return;
        }
    };

    let mut table = table;

    // Parse SET clause
    let set_pos = query.to_uppercase().find("SET").unwrap();
    let where_pos = query.to_uppercase().find("WHERE");
    let set_str = if let Some(wp) = where_pos {
        &query[set_pos + 3..wp].trim()
    } else {
        &query[set_pos + 3..].trim()
    };

    let mut updates = Vec::new();
    for assignment in set_str.split(',') {
        let kv: Vec<&str> = assignment.split('=').collect();
        if kv.len() == 2 {
            updates.push((kv[0].trim().to_string(), kv[1].trim().trim_matches('\'')));
        }
    }

    // Filter rows by WHERE clause if exists
    let mut filtered_rows = table.rows.clone();
    if let Some(wp) = where_pos {
        let cond_str = query[wp + 5..].trim();
        let cond_parts: Vec<&str> = cond_str.split_whitespace().collect();
        if cond_parts.len() == 3 {
            let col_name = cond_parts[0];
            let operator = cond_parts[1];
            let value = cond_parts[2].trim_matches('\'');

            if let Some(col_index) = table.columns.iter().position(|c| c == col_name) {
                filtered_rows = filtered_rows.into_iter().enumerate().filter(|(_, row)| {
                    let cell = &row[col_index];
                    match operator {
                        "=" => cell == value,
                        ">" => cell.parse::<f64>().unwrap_or(0.0) > value.parse::<f64>().unwrap_or(0.0),
                        "<" => cell.parse::<f64>().unwrap_or(0.0) < value.parse::<f64>().unwrap_or(0.0),
                        _ => false,
                    }
                }).map(|(i, _)| table.rows[i].clone()).collect();
            }
        }
    }

    // Apply updates
    for row in table.rows.iter_mut() {
        if filtered_rows.contains(row) {
            for (col, val) in updates.iter() {
                if let Some(idx) = table.columns.iter().position(|c| c == col) {
                    row[idx] = val.to_string();
                }
            }
        }
    }

    DB::save_table(&table);
    println!("Updated {} rows.", filtered_rows.len());
}