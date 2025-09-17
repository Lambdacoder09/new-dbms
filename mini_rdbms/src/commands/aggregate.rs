use crate::db::DB;

pub fn aggregate(query: &str) {
    let query = query.trim().trim_end_matches(';');
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 4 { return; }

    let func_part = parts[1];
    let func_name = func_part.split('(').next().unwrap().to_uppercase();
    let col_name = func_part.split('(').nth(1).unwrap().trim_end_matches(')');

    let table_name = parts[3];
    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => { println!("Table '{}' not found", table_name); return; }
    };

    let col_index = match table.columns.iter().position(|c| c == col_name) {
        Some(idx) => idx,
        None => { println!("Column '{}' not found", col_name); return; }
    };

    let values: Vec<f64> = table.rows.iter()
        .filter_map(|r| r[col_index].parse::<f64>().ok())
        .collect();

    match func_name.as_str() {
        "COUNT" => println!("COUNT({}) = {}", col_name, values.len()),
        "SUM" => println!("SUM({}) = {}", col_name, values.iter().sum::<f64>()),
        "AVG" => {
            let avg = if !values.is_empty() { values.iter().sum::<f64>() / values.len() as f64 } else { 0.0 };
            println!("AVG({}) = {}", col_name, avg);
        },
        "MIN" => println!("MIN({}) = {}", col_name, values.iter().cloned().fold(f64::INFINITY, f64::min)),
        "MAX" => println!("MAX({}) = {}", col_name, values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
        _ => println!("Unsupported function: {}", func_name),
    }
}
