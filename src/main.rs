use klickhouse::*;
use std::fs;
use std::process::Command;

#[derive(Row, Debug, Default)]
pub struct Opensky {
    callsign: String,
    number: String,
    icao24: String,
    registration: String,
    typecode: String,
    origin: String,
    destination: String,
    firstseen: DateTime,
    lastseen: DateTime,
    day: DateTime,
    latitude_1: f64,
    longitude_1: f64,
    altitude_1: f64,
    latitude_2: f64,
    longitude_2: f64,
    altitude_2: f64,
}

#[tokio::main]
async fn main() {
    let client = Client::connect("127.0.0.1:9000", ClientOptions::default())
        .await
        .unwrap();
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

    client
        .execute("DROP TABLE IF EXISTS Opensky")
        .await
        .unwrap();
    client
        .execute(
            "CREATE TABLE Opensky (
            callsign String,
            number String,
            icao24 String,
            registration String,
            typecode String,
            origin String,
            destination String,
            firstseen DateTime,
            lastseen DateTime,
            day DateTime,
            latitude_1 Float64,
            longitude_1 Float64,
            altitude_1 Float64,
            latitude_2 Float64,
            longitude_2 Float64,
            altitude_2 Float64
        ) ENGINE = MergeTree()
        ORDER BY (origin, destination, callsign)",
        )
        .await
        .unwrap();

    let files = fs::read_dir("./clickhouse/click_data/user_files/")
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
        let query = format!("INSERT INTO Opensky FORMAT CSVWithNames\n{}", output_str);

        client.execute(&query).await.unwrap();
    }

    // client
    //     .execute("SELECT * FROM Opensky LIMIT 100")
    //     .await
    //     .unwrap();
    // Split Opensky into multiple subtables based on date
    let split_query = r#"
        CREATE TABLE IF NOT EXISTS Opensky_split
        (
            callsign String,
            number String,
            icao24 String,
            registration String,
            typecode String,
            origin String,
            destination String,
            firstseen DateTime,
            lastseen DateTime,
            day DateTime,
            latitude_1 Float64,
            longitude_1 Float64,
            altitude_1 Float64,
            latitude_2 Float64,
            longitude_2 Float64,
            altitude_2 Float64
        ) ENGINE = MergeTree()
        ORDER BY (origin, destination, callsign, day)
        PARTITION BY toYYYYMMDD(day)
    "#;

    client.execute(split_query).await.unwrap();

    let insert_query = r#"
        INSERT INTO Opensky_split
        SELECT *
        FROM Opensky
    "#;

    client.execute(insert_query).await.unwrap();

    drop(client);
    progress_task.await.unwrap();
}
