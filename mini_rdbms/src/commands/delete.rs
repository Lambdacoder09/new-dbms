use crate::db::DB;
use regex::Regex;

pub fn delete(query: &str) {
    let re = Regex::new(r"(?i)DELETE FROM (\w+)\s+WHERE\s+(.+)").unwrap();
    if let Some(cap) = re.captures(query) {
        let table_name = &cap[1];
        let where_clause = &cap[2];

        let mut table = match DB::load_table(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist!", table_name);
                return;
            }
        };

        let filtered_rows = apply_where(&table, where_clause);
        table.rows.retain(|row| !filtered_rows.contains(row));

        DB::save_table(table_name, &table);
        println!("Delete completed.");
    } else {
        println!("Invalid DELETE syntax!");
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
