mod db;
mod commands;

use commands::create::create_table;
use commands::insert::insert;
use commands::select::select;
use commands::update::update;
use commands::delete::delete;
use db::DB;

use std::io::{self, Write};

fn main() {
    DB::init();

    println!("Welcome to Mini-Rust-DBMS!");
    println!("Type SQL commands (end with ;). Type 'exit;' to quit.");

    loop {
        print!("mini-rdbms> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit;") {
            println!("Bye!");
            break;
        }

        // Remove trailing semicolon
        let command = input.trim_end_matches(';');

        if command.to_uppercase().starts_with("CREATE TABLE") {
            create_table(command);
        } else if command.to_uppercase().starts_with("INSERT INTO") {
            insert(command);
        } else if command.to_uppercase().starts_with("SELECT") {
            select(command);
        } else if command.to_uppercase().starts_with("UPDATE") {
            update(command);
        } else if command.to_uppercase().starts_with("DELETE FROM") {
            delete(command);
        } else {
            println!("Unsupported command or syntax error!");
        }
    }
}
