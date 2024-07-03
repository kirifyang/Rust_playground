use crate::db::schema::Opensky;
use klickhouse::*;
use std::fs;
use std::process::Command;

const CLICKHOUSE_USER_FILE_PATH: &str = "./clickhouse/click_data/user_files";

pub async fn insert_table(client: &Client, table_name: &str, row: Vec<Opensky>) {
    let query = format!("INSERT INTO {} FORMAT native", table_name);

    client.insert_native_block(query, row).await.unwrap();
}

pub async fn drop_table(client: &Client, table_name: &str) {
    let query = format!("DROP TABLE IF EXISTS {}", table_name);
    client.execute(query).await.unwrap();
}

pub async fn create_table(client: &Client, table_name: &str, schema: &str) {
    drop_table(client, table_name).await;
    let query = format!(
        "CREATE TABLE {} ({}) ENGINE = MergeTree() ORDER BY (origin, destination, callsign)",
        table_name, schema
    );
    client.execute(query).await.unwrap();
}

pub async fn insert_table_from_files(client: &Client, table_name: &str) {
    let files = fs::read_dir(CLICKHOUSE_USER_FILE_PATH)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            let binding = entry.file_name();
            let file_name_str = binding.to_string_lossy();
            let file_name = file_name_str.as_ref();
            // file_name.starts_with("flightlist_") && file_name.ends_with(".csv.gz")
            // for testing purposes, only use the 20190131 file
            file_name.starts_with("flightlist_") && file_name.ends_with("20190131.csv.gz")
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    for file in files {
        let output = Command::new("gzip")
            .arg("-c")
            .arg("-d")
            .arg(file.to_str().unwrap())
            .output()
            .unwrap();

        let output_str = String::from_utf8_lossy(&output.stdout);
        let query = format!(
            "INSERT INTO {} FORMAT CSVWithNames\n{}",
            table_name, output_str
        );

        client.execute(&query).await.unwrap();
    }
}
