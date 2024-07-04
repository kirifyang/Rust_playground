mod db;
mod r#macro;

use crate::db::file::*;
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
    println!("Table created and data inserted");

    split_table_by_day(&client, "Opensky").await.unwrap();

    drop(client);
    progress_task.await.unwrap();
}
