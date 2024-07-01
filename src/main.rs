use klickhouse::*;

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
    let process_task = tokio::task::spawn(async move {
        let mut current_query = Uuid::nil();
        let mut process_total = Progress::default();
        while let Ok((query, process)) = progress.recv().await {
            if query != current_query {
                current_query = query;
                process_total = Progress::default();
            }
            process_total += process;
            println!(
                "Progress on query {}: {}/{} {:.2}%",
                query,
                process_total.read_rows,
                process_total.new_total_rows_to_read,
                100.0 * process_total.read_rows as f64
                    / process_total.new_total_rows_to_read as f64
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
        ORDER BY (icao24, day)",
        )
        .await
        .unwrap();

    client
        .execute("SELECT * FROM Opensky LIMIT 100")
        .await
        .unwrap();

    drop(client);
    process_task.await.unwrap();
}
