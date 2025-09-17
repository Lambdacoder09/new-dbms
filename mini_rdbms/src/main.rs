mod db;
mod commands;

use commands::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Mini-Rust-DBMS!");
    println!("Type SQL commands (end with ';'). Type 'exit;' to quit.");

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

        if input.is_empty() {
            continue;
        }

        // Route command to correct handler
        process_command(input);
    }
}

fn process_command(command: &str) {
    let cmd_upper = command.to_uppercase();
    if cmd_upper.starts_with("CREATE TABLE") {
        create::create(command);
    } else if cmd_upper.starts_with("INSERT INTO") {
        insert::insert(command);
    } else if cmd_upper.starts_with("SELECT") {
        select::select(command);
    } else if cmd_upper.starts_with("UPDATE") {
        update::update(command);
    } else if cmd_upper.starts_with("DELETE FROM") {
        delete::delete(command);
    } else if cmd_upper.starts_with("ALTER TABLE") {
        alter::alter(command);
    } else if cmd_upper.starts_with("TRUNCATE TABLE") {
        truncate::truncate(command);
    } else if cmd_upper.starts_with("DROP TABLE") {
        drop::drop(command); // <- match your drop.rs function
    } else {
        println!("Unsupported command: {}", command);
    }
}
