mod db;
mod r#macro;

use crate::db::ops::*;
use crate::db::schema::*;
use klickhouse::*;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = tokio::time::timeout(
        Duration::from_secs(10),
        Client::connect("127.0.0.1:9000", ClientOptions::default()),
    )
    .await
    .unwrap_or_else(|err| panic!("Failed to connect to ClickHouse: {}", err))
    .unwrap_or_else(|_| panic!("Failed to connect to ClickHouse: timeout"));
    let mut progress = client.subscribe_progress();
    let progress_task = tokio::task::spawn(async move {
        let mut current_query = Uuid::nil();
        let mut progress_total = Progress::default();
        while let Ok((query, progress)) = progress.recv().await {
            if query != current_query {
                progress_total = Progress::default();
                current_query = query;
            }
            progress_total += progress;
            println!("{:?}", progress_total);
            println!(
                "Progress on query {}: {}/{} {:.2}%",
                query,
                progress_total.read_rows,
                progress_total.new_total_rows_to_read,
                100.0 * progress_total.read_rows as f64
                    / progress_total.new_total_rows_to_read as f64
            );
        }
    });

    let schema = Opensky::schema();
    create_table(&client, "Opensky", &schema).await;
    insert_table_from_files(&client, "Opensky").await;

    // let files = fs::read_dir("./clickhouse/click_data/user_files/")
    //     .unwrap()
    //     .filter_map(Result::ok)
    //     .filter(|entry| {
    //         let binding = entry.file_name();
    //         let file_name_str = binding.to_string_lossy();
    //         let file_name = file_name_str.as_ref();
    //         // file_name.starts_with("flightlist_") && file_name.ends_with(".csv.gz")
    //         // for testing purposes, only use the 20190131 file
    //         file_name.starts_with("flightlist_") && file_name.ends_with("20190131.csv.gz")
    //     })
    //     .map(|entry| entry.path())
    //     .collect::<Vec<_>>();

    // for file in files {
    //     let output = Command::new("gzip")
    //         .arg("-c")
    //         .arg("-d")
    //         .arg(file.to_str().unwrap())
    //         .output()
    //         .unwrap();

    //     let output_str = String::from_utf8_lossy(&output.stdout);
    //     let query = format!("INSERT INTO Opensky FORMAT CSVWithNames\n{}", output_str);

    //     client.execute(&query).await.unwrap();
    // }

    drop(client);
    progress_task.await.unwrap();
}
