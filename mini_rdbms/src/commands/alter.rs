use crate::db::DB;

pub fn alter(query: &str) {
    let query_upper = query.to_uppercase();

    if query_upper.starts_with("ALTER TABLE") {
        let parts: Vec<&str> = query.trim().split_whitespace().collect();

        if parts.len() < 5 {
            println!("Invalid ALTER TABLE syntax!");
            return;
        }

        let table_name = parts[2];
        let action = parts[3].to_uppercase();

        if action == "ADD" && parts[4].to_uppercase() == "COLUMN" {
            if parts.len() < 6 {
                println!("Column name missing!");
                return;
            }

            let col_name = parts[5];

            let mut table = match DB::load_table(table_name) {
                Some(t) => t,
                None => {
                    println!("Table '{}' does not exist!", table_name);
                    return;
                }
            };

            // Add the new column
            table.columns.push(col_name.to_string());

            // Add empty value for all existing rows
            for row in table.rows.iter_mut() {
                row.push(String::new());
            }

            // Save the updated table
            DB::save_table(&table);

            println!("Column '{}' added to table '{}'", col_name, table_name);
        } else {
            println!("Only ADD COLUMN is supported for ALTER TABLE right now.");
        }
    } else {
        println!("Unsupported command: {}", query);
    }
}
