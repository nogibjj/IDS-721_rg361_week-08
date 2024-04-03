extern crate csv;
extern crate env_logger;
extern crate log;
extern crate rusqlite;

use csv::Reader;
use rusqlite::{Connection, Result};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process;

#[allow(clippy::manual_flatten)]
fn main() -> Result<(), Box<dyn Error>> {
    // Get the SQLite database file path and CSV file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <database_file> <csv_file>", args[0]);
        process::exit(1);
    }

    let db_path = &args[1];
    let csv_file_path = &args[2];

    // Open the SQLite database connection
    let conn = Connection::open(db_path)?;

    //Delete the table if it already exists
    conn.execute("DROP TABLE IF EXISTS data", [])?;

    // Create a table to store the CSV data
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (column1 TEXT, column2 TEXT, column3 TEXT, column4 TEXT, column5 TEXT)",
        [],
    )?;

    // Open the CSV file and insert its data into the SQLite database
    let file = File::open(csv_file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO data (column1, column2, column3, column4, column5) VALUES (?1, ?2, ?3, ?4, ?5)",
            [&record[0], &record[1], &record[2], &record[3], &record[4]],
        )?;
    }

    println!("CSV data imported into the SQLite database.");

    // Rename the columns using SQL
    conn.execute("ALTER TABLE data RENAME TO data_old", [])?;
    conn.execute(
        "CREATE TABLE data AS SELECT column1 AS patient, column2 AS sex, column3 AS agegprp, column4 AS bp_before, column5 AS bp_after FROM data_old",
        [],
    )?;
    conn.execute("DROP TABLE data_old", [])?;

    // Allow SQL queries from the terminal
    loop {
        let mut input = String::new();
        print!("Enter an SQL query (or 'exit' to quit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut input)?;

        let query = input.trim();
        if query == "exit" {
            break;
        }

        match conn.execute_batch(query) {
            Ok(_) => {
                println!("Query executed successfully.");

                if query.to_lowercase().starts_with("select") {
                    let mut stmt = conn.prepare(query)?;

                    // Get the column names
                    let columns: Vec<String> = stmt
                        .column_names()
                        .iter()
                        .map(|col_name| col_name.to_string())
                        .collect();

                    let header = columns.join(" | ");
                    println!("{}", header);

                    let rows = stmt.query_map([], |row| {
                        let values: Vec<String> = columns
                            .iter()
                            .enumerate()
                            .map(|(i, _)| {
                                match row.get(i) {
                                    Ok(value) => value,
                                    Err(_) => "NULL".to_string(), // Handle NULL values if necessary.
                                }
                            })
                            .collect();
                        Ok(values)
                    })?;
                    for row in rows {
                        if let Ok(row_data) = row {
                            let row_str = row_data.join(" | ");
                            println!("{}", row_str);
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error executing query: {}", err),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use csv::Reader;
    use rusqlite::{Connection};

    #[test]
    fn test_main() {
        assert!(true);
    }

    #[test]
    fn test_csv_access() {
        let file_path = "../Data/blood_pressure.csv";
        let file = File::open(&file_path);
        assert!(file.is_ok(), "Should be able to open the file");

        let reader = file.map(|f| Reader::from_reader(f));
        assert!(reader.is_ok(), "Should be able to read the file");
    }

    #[test]
    fn test_db_connection() {
        let conn = Connection::open("Master.db");
        assert!(conn.is_ok(), "Should be able to connect to the database");
    }

    #[test]
    fn test_db_columns() {
        let conn = Connection::open("Master.db").unwrap();
        let stmt = conn.prepare("SELECT * FROM data LIMIT 1").unwrap();
        let columns = stmt.column_names();
        assert_eq!(columns.len(), 5, "Should have 5 columns");
    }
}