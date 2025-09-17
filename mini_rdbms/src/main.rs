mod db;
mod commands;

use commands::*;
use std::io::{self, Write};
use colored::*;
use prettytable::{Table, Row, Cell}; // For table display
use figlet_rs::FIGfont;               // For ASCII banners

fn main() {
    // ASCII banner
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Mini-Rust-DBMS");
    if let Some(figure) = figure {
        println!("{}", figure.to_string().bright_green());
    }

    println!("{}", "Type SQL commands (end with ';'). Type 'exit;' to quit.".bright_blue());

    loop {
        // Stylish prompt
        print!("{}", "mini-rdbms> ".bright_yellow().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit;") {
            println!("{}", "Bye!".bright_red().bold());
            break;
        }

        if input.is_empty() {
            continue;
        }

        let first_word = input
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_uppercase();

        match first_word.as_str() {
            "CREATE" => crate::commands::create::create(input),
            "INSERT" => crate::commands::insert::insert(input),
            "SELECT" => select_with_table(input),
            "UPDATE" => crate::commands::update::update(input),
            "DELETE" => crate::commands::delete::delete(input),
            "ALTER" => crate::commands::alter::alter(input),
            "TRUNCATE" => crate::commands::truncate::truncate(input),
            "DROP" => crate::commands::drop::drop_table(input),
            _ => println!("{}", format!("Unsupported command: {}", input).bright_red()),
        }
    }
}

// Display SELECT results in a pretty table
fn select_with_table(query: &str) {
    // Load results using your existing select function (or modify to return Vec of rows)
    let results = crate::commands::select::select_return_rows(query);

    if results.is_empty() {
        println!("{}", "No records found.".bright_yellow());
        return;
    }

    let mut table = Table::new();

    // Add headers
    if let Some(headers) = results.first() {
        let header_cells: Vec<Cell> = headers.iter().map(|h| Cell::new(h)).collect();
        table.add_row(Row::new(header_cells));
    }

    // Add rows
    for row in results.iter().skip(1) {
        let row_cells: Vec<Cell> = row.iter().map(|c| Cell::new(c)).collect();
        table.add_row(Row::new(row_cells));
    }

    table.printstd();
}
