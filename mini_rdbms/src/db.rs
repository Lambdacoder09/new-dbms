use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub struct DB;

impl DB {
    pub fn init() {
        fs::create_dir_all("tables").unwrap();
    }

    pub fn save_table(table: &Table) {
        // Save metadata
        let meta_path = format!("tables/{}.meta", table.name);
        fs::write(&meta_path, table.columns.join(",")).unwrap();

        // Save rows
        let data_path = format!("tables/{}.data", table.name);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&data_path)
            .unwrap();

        for row in &table.rows {
            writeln!(file, "{}", row.join(",")).unwrap();
        }
    }

    pub fn load_table(name: &str) -> Option<Table> {
        let meta_path = format!("tables/{}.meta", name);
        let data_path = format!("tables/{}.data", name);

        if !Path::new(&meta_path).exists() || !Path::new(&data_path).exists() {
            return None;
        }

        let columns = fs::read_to_string(meta_path)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let file = OpenOptions::new().read(true).open(data_path).unwrap();
        let reader = BufReader::new(file);

        let mut rows = vec![];
        for line in reader.lines() {
            let row = line.unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            rows.push(row);
        }

        Some(Table {
            name: name.to_string(),
            columns,
            rows,
        })
    }

    pub fn list_tables() -> Vec<String> {
        let mut tables = vec![];
        if let Ok(entries) = fs::read_dir("tables") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "meta" {
                        if let Some(stem) = path.file_stem() {
                            tables.push(stem.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        tables
    }
}

impl DB {
    pub fn table_path(table_name: &str) -> String {
        format!("tables/{}.json", table_name)
    }
}
