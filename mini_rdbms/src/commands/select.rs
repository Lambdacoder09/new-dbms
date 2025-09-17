use crate::db::DB;
use prettytable::{Table, Row, Cell};
use colored::*;

pub fn select(query: &str) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.len() < 4 {
        println!("{}", "Invalid SELECT syntax!".bright_red().bold());
        return;
    }

    let table_name = parts[3];
    let table = match DB::load_table(table_name) {
        Some(t) => t,
        None => {
            println!("{}", format!("Table '{}' not found!", table_name).bright_red().bold());
            return;
        }
    };

    let columns_to_show: Vec<String> = if parts[1] == "*" {
        table.columns.clone()
    } else {
        parts[1]
            .split(',')
            .map(|c| c.trim().to_string())
            .filter(|c| table.columns.contains(c))
            .collect()
    };

    if columns_to_show.is_empty() {
        println!("{}", "No valid columns found to display!".bright_yellow().bold());
        return;
    }

    let col_indexes: Vec<usize> = columns_to_show
        .iter()
        .filter_map(|c| table.columns.iter().position(|x| x == c))
        .collect();

    let mut pretty_table = Table::new();

    // Add header with bold and colored style
    let header_cells: Vec<Cell> = columns_to_show
        .iter()
        .map(|c| Cell::new(c).style_spec("bFc")) // bold + foreground color cyan
        .collect();
    pretty_table.add_row(Row::new(header_cells));

    // Add rows
    for (i, row) in table.rows.iter().enumerate() {
        let row_cells: Vec<Cell> = col_indexes
            .iter()
            .map(|&idx| {
                let val = &row[idx];
                if i % 2 == 0 {
                    Cell::new(val).style_spec("Fr") // odd rows red
                } else {
                    Cell::new(val).style_spec("Fw") // even rows white
                }
            })
            .collect();
        pretty_table.add_row(Row::new(row_cells));
    }

    pretty_table.printstd();
}
