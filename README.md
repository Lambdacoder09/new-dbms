
```markdown
# Mini-Rust-DBMS

**Mini-Rust-DBMS** is a lightweight, terminal-based database management system written in **Rust**. It supports basic SQL-like operations and is designed for learning, experimentation, and small-scale demos. The system features a modern CLI with colorized output and table-like display for query results.

---

## Features

- **Create Tables:** Define tables with any number of columns.
- **Insert Rows:** Add rows using simple SQL-like syntax.
- **Select Queries:** Fetch specific or all columns with table-formatted output.
- **Update & Delete:** Modify or remove rows using `WHERE` clauses.
- **Alter Tables:** Add new columns dynamically.
- **Truncate & Drop Tables:** Clear or delete tables quickly.
- **Transaction Support:** Basic commit and rollback functionality.
- **Modern CLI:** Colored prompts and pretty table outputs for better readability.

---

## Screenshots

```

Welcome to Mini-Rust-DBMS!
Type SQL commands (end with ';'). Type 'exit;' to quit.

mini-rdbms> CREATE TABLE students (id, name, grade);
Table 'students' created successfully!

mini-rdbms> INSERT INTO students VALUES (1, 'Zayed', 'A');
1 row inserted into 'students'

mini-rdbms> SELECT \* FROM students;
+----+-------+-------+
\| id | name  | grade |
+----+-------+-------+
\| 1  | Zayed | A     |
+----+-------+-------+

````

---

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if not already installed.
2. Clone the repository:

```bash
git clone https://github.com/yourusername/mini-rust-dbms.git
cd mini-rust-dbms
````

3. Build and run the project:

```bash
cargo run
```

---

## Usage

* Commands must end with `;`.
* Example commands:

```sql
CREATE TABLE students (id, name, grade);
INSERT INTO students VALUES (1, 'Zayed', 'A');
SELECT * FROM students;
UPDATE students SET grade='A+' WHERE id=1;
DELETE FROM students WHERE id=2;
ALTER TABLE students ADD COLUMN email;
TRUNCATE TABLE students;
DROP TABLE students;
exit;
```

---

## Project Structure

```
src/
 ├─ commands/      # SQL command handlers
 │   ├─ create.rs
 │   ├─ insert.rs
 │   ├─ select.rs
 │   ├─ update.rs
 │   ├─ delete.rs
 │   ├─ alter.rs
 │   ├─ truncate.rs
 │   └─ drop.rs
 ├─ db.rs          # Table and database handling
 └─ main.rs        # CLI entry point
```

* Tables are stored as serialized files in the `db/` folder.

---

## Dependencies

* [`prettytable-rs`](https://crates.io/crates/prettytable-rs) – Table display for queries.
* [`colored`](https://crates.io/crates/colored) – Colorized terminal output.
* [`figlet-rs`](https://crates.io/crates/figlet-rs) – Optional ASCII banners for CLI branding.

---

## Roadmap / Planned Features

* Full transaction support.
* CREATE INDEX and aggregate functions.
* Web-based UI for live demos.
* Advanced SQL operations (JOINs, subqueries).

---

## Contribution

Contributions, bug reports, and feature suggestions are welcome!
Fork the repository and submit a pull request or open an issue.

---

## License

MIT License © 2025 Zayed Khan

```

---

If you want, I can also **write a shorter, visually appealing version** specifically for your **friend to make a web UI demo**, with step-by-step instructions and example screenshots embedded.  

Do you want me to do that version too?
```
