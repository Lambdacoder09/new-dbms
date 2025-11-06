Here’s your refined and properly formatted version — clean, professional, and visually balanced for a README:

````markdown
<p align="center">
  <img src="assets/Marin_Kitagawa.webp" alt="Marin Kitagawa" width="180">
</p>

<h2 align="center">
  <em>Dedicated to my love for gyarus and anime waifus — for giving my life a new spark and purpose.</em>
</h2>

---

# Mini-Rust-DBMS

Mini-Rust-DBMS is a lightweight, terminal-based database management system written in **Rust**. It allows you to execute basic SQL-like commands such as `CREATE`, `INSERT`, `SELECT`, `UPDATE`, `DELETE`, `ALTER`, `TRUNCATE`, and `DROP` on flat-file-based tables. It’s perfect for learning, demos, or experimenting with database concepts.

---

## Features

- **Create tables** with any number of columns.
- **Insert rows** with simple SQL syntax.
- **Select queries** with optional column filtering.
- **Update and delete rows** with `WHERE` clauses.
- **Alter tables** to add new columns.
- **Truncate and drop tables**.
- **Pretty table display** for `SELECT` queries using `prettytable-rs`.
- **Colored terminal output** for better readability using `colored`.

---

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository:

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

The CLI prompts you to enter SQL-like commands:

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

* Commands must end with `;`.
* `exit;` quits the program.

---

## Project Structure

```
src/
 ├─ commands/      # All SQL command handlers
 │   ├─ create.rs
 │   ├─ insert.rs
 │   ├─ select.rs
 │   ├─ update.rs
 │   ├─ delete.rs
 │   ├─ alter.rs
 │   ├─ truncate.rs
 │   └─ drop.rs
 ├─ db.rs          # Table and database structure handling
 └─ main.rs        # Entry point / CLI
```

* **Tables** are stored in the `db/` folder as serialized files.
* **Transactions** are partially supported for rollback/commit (future improvement).

---

## Dependencies

* [`prettytable-rs`](https://crates.io/crates/prettytable-rs) – For displaying tables.
* [`colored`](https://crates.io/crates/colored) – For colored output in the terminal.

---

## Planned Improvements

* Add full transaction support with rollback.
* Implement `CREATE INDEX` and aggregate functions.
* Build a web-based UI for live demos.
* Add more advanced SQL features.

---

## Contribution

Contributions, suggestions, or bug reports are welcome!
Feel free to fork the repo and submit a pull request.

---

## License

MIT License © 2025 Zayed Khan

```

✨ This version:
- Centers and resizes your image neatly.  
- Italicizes your dedication for a soft aesthetic.  
- Keeps perfect Markdown formatting so it renders properly on GitHub or VS Code preview.  

Want me to make it look even more aesthetic (like a banner layout with title and image centered)?
```
