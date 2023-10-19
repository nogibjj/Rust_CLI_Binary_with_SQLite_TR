use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, i32};

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy");

    println!("Extraction success");
}

pub fn transform_load(dataset: &str) -> Result<String> {
    let conn = Connection::open("AlcoholDB.db")?;

    conn.execute("DROP TABLE IF EXISTS AlcoholDB", [])?;

    conn.execute(
        "CREATE TABLE AlcoholDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT, 
            beer_servings INTEGER,
            spirit_servings INTEGER,
            wine_servings INTEGER,
            total_pure_alcohol
       )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    let mut stmt = conn.prepare(
        "
        INSERT INTO AlcoholDB (
            country, 
            beer_servings,
            spirit_servings,
            wine_servings,
            total_pure_alcohol
          ) 
          VALUES (?, ?, ?, ?, ?)
        ",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[&record[0], &record[1], &record[2], &record[3], &record[4]])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("AlcoholDB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("AlcoholDB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, i32>(3)?,
                row.get::<usize, i32>(4)?,
                row.get::<usize, f64>(5)?,
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    country,
                    beer_serving,
                    spirit_servings,
                    wine_servings,
                    total_pure_alcohol,
                )) => {
                    println!(
                        "Result: id={}, country={}, beer_serving={}, 
                         spirit_serving={}, wine_serving={}, total_pure_alcohol={}",
                        id,
                        country,
                        beer_serving,
                        spirit_servings,
                        wine_servings,
                        total_pure_alcohol
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
