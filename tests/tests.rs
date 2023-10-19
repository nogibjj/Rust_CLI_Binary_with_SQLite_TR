use sqlite_tr::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://github.com/fivethirtyeight/data/blob/master/tennis-time/serve_times.csv?raw=true";
    let file_path = "data/alcohol.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/alcohol.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "AlcoholDB.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM AlcoholDB WHERE id = 9;";
    let result = query(select_query);

    assert!(result.is_ok());
}