use crate::db::DB;

pub fn truncate(query: &str) {
    let parts: Vec<&str> = query.trim().split_whitespace().collect();
    if parts.len() < 3 {
        println!("Invalid TRUNCATE syntax!");
        return;
    }

    let table_name = parts[2];

    let mut table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' does not exist!", table_name);
            return;
        }
    };

    table.rows.clear();

    // ✅ Corrected save
    DB::save_table(&table);
    println!("Table '{}' truncated successfully!", table_name);
}

