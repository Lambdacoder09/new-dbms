#!/bin/bash

# 1️⃣ Fix alter.rs
sed -i "s/DB::save_table(table_name, &table)/DB::save_table(&table)/" src/commands/alter.rs

# 2️⃣ Fix truncate.rs
sed -i "s/DB::save_table(table_name, &table)/DB::save_table(&table)/" src/commands/truncate.rs

# 3️⃣ Add table_path function to db.rs if not present
if ! grep -q "pub fn table_path" src/db.rs; then
    echo "" >> src/db.rs
    echo "impl DB {" >> src/db.rs
    echo "    pub fn table_path(table_name: &str) -> String {" >> src/db.rs
    echo "        format!(\"tables/{}.json\", table_name)" >> src/db.rs
    echo "    }" >> src/db.rs
    echo "}" >> src/db.rs
fi

# 4️⃣ Clean unused Table imports
for file in src/commands/insert.rs src/commands/select.rs src/commands/update.rs src/commands/delete.rs
do
    sed -i "s/, Table//" "$file"
done

echo "All fixes applied! Try running: cargo run"
