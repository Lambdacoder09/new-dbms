use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use regex::Regex;

const DB_FOLDER: &str = "data";

#[derive(Serialize, Deserialize)]
struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

struct MiniRDBMS;

impl MiniRDBMS {
    fn new() -> Self {
        if !Path::new(DB_FOLDER).exists() {
            fs::create_dir(DB_FOLDER).unwrap();
        }
        MiniRDBMS
    }

    fn execute(&self, query: &str) {
        let query = query.trim();
        if query.to_uppercase().starts_with("CREATE TABLE") {
            self.create_table(query);
        } else if query.to_uppercase().starts_with("INSERT INTO") {
            self.insert(query);
        } else if query.to_uppercase().starts_with("SELECT") {
            self.select(query);
        } else {
            println!("Unsupported query!");
        }
    }

    fn create_table(&self, query: &str) {
        let re = Regex::new(r"(?i)CREATE TABLE (\w+)\s*\((.+)\)").unwrap();
        if let Some(cap) = re.captures(query) {
            let table_name = &cap[1];
            let columns: Vec<String> = cap[2].split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let table_path = format!("{}/{}.json", DB_FOLDER, table_name);
            if Path::new(&table_path).exists() {
                println!("Table '{}' already exists!", table_name);
                return;
            }
            let table = Table { columns, rows: vec![] };
            let data = serde_json::to_string_pretty(&table).unwrap();
            let mut file = File::create(&table_path).unwrap();
            file.write_all(data.as_bytes()).unwrap();
            println!("Table '{}' created!", table_name);
        } else {
            println!("Invalid CREATE TABLE syntax!");
        }
    }

    fn insert(&self, query: &str) {
        let re = Regex::new(r"(?i)INSERT INTO (\w+)\s+VALUES\s*\((.+)\)").unwrap();
        if let Some(cap) = re.captures(query) {
            let table_name = &cap[1];
            let row_values: Vec<String> = cap[2].split(',')
                .map(|s| s.trim().trim_matches('\'').to_string())
                .collect();

            let table_path = format!("{}/{}.json", DB_FOLDER, table_name);
            if !Path::new(&table_path).exists() {
                println!("Table '{}' does not exist!", table_name);
                return;
            }

            let mut file = File::open(&table_path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let mut table: Table = serde_json::from_str(&data).unwrap();

            if row_values.len() != table.columns.len() {
                println!("Column count doesn't match!");
                return;
            }

            table.rows.push(row_values);

            let data = serde_json::to_string_pretty(&table).unwrap();
            let mut file = File::create(&table_path).unwrap();
            file.write_all(data.as_bytes()).unwrap();

            println!("Row inserted successfully!");
        } else {
            println!("Invalid INSERT syntax!");
        }
    }

    fn select(&self, query: &str) {
        let re = Regex::new(r"(?i)SELECT \* FROM (\w+)").unwrap();
        if let Some(cap) = re.captures(query) {
            let table_name = &cap[1];
            let table_path = format!("{}/{}.json", DB_FOLDER, table_name);
            if !Path::new(&table_path).exists() {
                println!("Table '{}' does not exist!", table_name);
                return;
            }

            let mut file = File::open(&table_path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let table: Table = serde_json::from_str(&data).unwrap();

            println!("{}", table.columns.join(" | "));
            println!("{}", "-".repeat(40));
            for row in table.rows {
                println!("{}", row.join(" | "));
            }
        } else {
            println!("Invalid SELECT syntax!");
        }
    }
}

fn main() {
    let db = MiniRDBMS::new();

    db.execute("CREATE TABLE students (id, name, age)");
    db.execute("INSERT INTO students VALUES (1, 'Zayed', 25)");
    db.execute("INSERT INTO students VALUES (2, 'Ali', 22)");
    db.execute("SELECT * FROM students");
}
