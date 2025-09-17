use crate::db::{DB};

pub fn select(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 4 {
        println!("Invalid SELECT syntax!");
        return;
    }

    let table_name = parts[3];
    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' not found!", table_name);
            return;
        }
    };

    let columns_to_show = if parts[1] == "*" {
        table.columns.clone()
    } else {
        parts[1]
            .split(',')
            .map(|c| c.trim().to_string())
            .collect::<Vec<_>>()
    };

    // Print header
    println!("{}", columns_to_show.join(" | "));

    // Get column indexes
    let col_indexes: Vec<usize> = columns_to_show
        .iter()
        .filter_map(|c| table.columns.iter().position(|x| x == c))
        .collect();

    // Print rows
    for row in table.rows.iter() {
        let selected: Vec<String> = col_indexes.iter().map(|&i| row[i].clone()).collect();
        println!("{}", selected.join(" | "));
    }
}
