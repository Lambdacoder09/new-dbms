use crate::db::DB;
use regex::Regex;

pub fn update(query: &str) {
    // Regex: UPDATE table SET column=value WHERE condition
    let re = Regex::new(r"(?i)UPDATE (\w+)\s+SET\s+(\w+)\s*=\s*'?(.*?)'?\s+WHERE\s+(.+)").unwrap();
    if let Some(cap) = re.captures(query) {
        let table_name = &cap[1];
        let set_col = &cap[2];
        let set_value = &cap[3];
        let where_clause = &cap[4];

        let mut table = match DB::load_table(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist!", table_name);
                return;
            }
        };

        let col_index = match table.columns.iter().position(|c| c == set_col) {
            Some(idx) => idx,
            None => {
                println!("Column '{}' does not exist!", set_col);
                return;
            }
        };

        let filtered_rows = apply_where(&table, where_clause);
        for row in table.rows.iter_mut() {
            if filtered_rows.contains(row) {
                row[col_index] = set_value.to_string();
            }
        }

        DB::save_table(table_name, &table);
        println!("Update completed.");
    } else {
        println!("Invalid UPDATE syntax!");
    }
}

// Reuse WHERE logic
fn apply_where(table: &crate::db::Table, clause: &str) -> Vec<Vec<String>> {
    let operators = ["<=", ">=", "<", ">", "="];
    let mut col_name = "";
    let mut op = "";
    let mut value = "";

    for &operator in &operators {
        if let Some(pos) = clause.find(operator) {
            col_name = clause[..pos].trim();
            op = operator;
            value = clause[pos + operator.len()..].trim().trim_matches('\'');
            break;
        }
    }

    let col_index = match table.columns.iter().position(|c| c == col_name) {
        Some(idx) => idx,
        None => return vec![],
    };

    table.rows
        .iter()
        .filter(|row| {
            let cell = &row[col_index];
            match op {
                "=" => cell == value,
                ">" => cell.parse::<f64>().ok() > value.parse::<f64>().ok(),
                "<" => cell.parse::<f64>().ok() < value.parse::<f64>().ok(),
                ">=" => cell.parse::<f64>().ok() >= value.parse::<f64>().ok(),
                "<=" => cell.parse::<f64>().ok() <= value.parse::<f64>().ok(),
                _ => false,
            }
        })
        .cloned()
        .collect()
}
