use crate::db::file::get_csvs_names_grouped_by_date;
use crate::db::schema::Opensky;
use flate2::read::GzDecoder;
use klickhouse::*;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use tokio::{task, runtime::Runtime};

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

// need to improve this
pub async fn insert_table_from_files(client:&Client, table_name:&str ) {
    let files = get_csvs_names_grouped_by_date(); // この関数も非同期にする必要があるかもしれません

    let mut tasks = vec![];

    for file in files {
        let client = client.clone();
        let table_name = table_name.to_string();
        let task = tokio::spawn(async move {
            println!("Processing file: {}", file);
            let file_path = format!("{}/{}", CLICKHOUSE_USER_FILE_PATH, file);
            let mut file = File::open(&file_path).expect("Failed to open file");
            let mut decoder = GzDecoder::new(file);
            let mut output_str = String::new();
            decoder.read_to_string(&mut output_str).expect("Failed to read gzip file");

            let query = format!(
                "INSERT INTO {} FORMAT CSVWithNames\n{}",
                table_name, output_str
            );

            client.execute(&query).await.unwrap();
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
