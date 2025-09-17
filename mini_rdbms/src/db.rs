use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

const DB_FOLDER: &str = "data";

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub struct DB;

impl DB {
    pub fn init() {
        if !Path::new(DB_FOLDER).exists() {
            fs::create_dir(DB_FOLDER).unwrap();
        }
    }

    pub fn table_path(table_name: &str) -> String {
        format!("{}/{}.json", DB_FOLDER, table_name)
    }

    pub fn load_table(table_name: &str) -> Option<Table> {
        let path = DB::table_path(table_name);
        if !Path::new(&path).exists() {
            return None;
        }
        let mut file = File::open(&path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        Some(serde_json::from_str(&data).unwrap())
    }

    pub fn save_table(table_name: &str, table: &Table) {
        let path = DB::table_path(table_name);
        let data = serde_json::to_string_pretty(&table).unwrap();
        let mut file = File::create(&path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}
