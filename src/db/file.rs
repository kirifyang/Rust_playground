use std::process::Command;
use std::path::PathBuf;
use std::fs;
use klickhouse::Client;

const CLICKHOUSE_USER_FILE_PATH: &str = "./clickhouse/click_data/user_files";

pub async fn process_file(client: &Client, table_name: &str, file_path: PathBuf) {
    let output = Command::new("gzip")
        .arg("-c")
        .arg("-d")
        .arg(file_path.to_str().unwrap())
        .output()
        .unwrap();

    let output_str = String::from_utf8_lossy(&output.stdout);
    let query = format!(
        "INSERT INTO {} FORMAT CSVWithNames\n{}",
        table_name, output_str
    );

    client.execute(&query).await.unwrap();
}

pub async fn insert_table_from_files(client: &Client, table_name: &str) {
    let mut files = fs::read_dir(CLICKHOUSE_USER_FILE_PATH)
        .await
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            let binding = entry.file_name();
            let file_name_str = binding.to_string_lossy();
            let file_name = file_name_str.as_ref();
            file_name.starts_with("flightlist_") && file_name.ends_with("20190131.csv.gz")
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    while let Some(file) = files.pop() {
        process_file(client, table_name, file).await;
    }
}