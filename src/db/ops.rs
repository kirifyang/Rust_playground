use crate::db::file::get_csvs_names_grouped_by_date;
use crate::db::schema::Opensky;
use flate2::read::GzDecoder;
use klickhouse::*;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

const CLICKHOUSE_USER_FILE_PATH: &str = "./clickhouse/click_data/user_files";
// semaphores are used to limit the number of concurrent tasks
const TASK_SEMAPHORE_LIMIT: usize = 3;
const TASK_WAIT_TIME: u64 = 5;

#[allow(dead_code)]
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
    let files = get_csvs_names_grouped_by_date(); 

    let semaphore = Arc::new(Semaphore::new(TASK_SEMAPHORE_LIMIT));
    let completed_tasks = Arc::new(AtomicUsize::new(0));
    let mut tasks = vec![];

    for file in files {
        let client = client.clone();
        let table_name = table_name.to_string();
        let semaphore_clone = semaphore.clone();
        let completed_clone = completed_tasks.clone();
        let task = tokio::spawn(async move {
            let _permit = semaphore_clone.acquire().await.expect("Failed to acquire semaphore permit");
            sleep(Duration::from_secs(TASK_WAIT_TIME)).await;
            println!("Processing file: {}", file);
            let file_path = format!("{}/{}", CLICKHOUSE_USER_FILE_PATH, file);
            let file = File::open(&file_path).expect("Failed to open file");
            let mut decoder = GzDecoder::new(file);
            let mut output_str = String::new();
            decoder
                .read_to_string(&mut output_str)
                .expect("Failed to read gzip file");

            let query = format!(
                "INSERT INTO {} FORMAT CSVWithNames\n{}",
                table_name, output_str
            );
            client.execute(&query).await.unwrap();
            completed_clone.fetch_add(1, Ordering::SeqCst);
        });
        tasks.push(task);
        if completed_tasks.load(Ordering::SeqCst) % TASK_SEMAPHORE_LIMIT == 0 {
            println!("Completed tasks: {}", completed_tasks.load(Ordering::SeqCst));
        }
    }

    for task in tasks {
        task.await.unwrap();
    }
}
