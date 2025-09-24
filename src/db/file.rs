use crate::db::schema::{Opensky, MaxDateRow, MinDateRow};
use chrono::{Duration, NaiveDate};
use csv::Writer;
use klickhouse::Client;
use std::fs;
use std::fs::File;
use std::iter::Iterator;
use std::path::Path;

// use std::path::PathBuf;
// use std::process::Command;

#[allow(dead_code)]
const CLICKHOUSE_USER_FILE_PATH: &str = "./clickhouse/click_data/user_files";
#[allow(dead_code)]
const PREFIX: &str = "flightlist_";

#[allow(dead_code)]
pub fn get_csvs_names_grouped_by_date() -> Vec<String> {
    fs::read_dir(CLICKHOUSE_USER_FILE_PATH)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            let binding = entry.file_name();
            let file_name_str = binding.to_string_lossy();
            let file_name = file_name_str.as_ref();
            file_name.starts_with(PREFIX) && file_name.ends_with(".csv.gz")
        })
        .map(|entry| {
            let binding = entry.file_name();
            let file_name_str = binding.to_string_lossy();
            let file_name = file_name_str.as_ref();
            file_name.to_string()
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub async fn split_table_by_day(
    client: &Client,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let query_max = format!("SELECT max(toDate(day)) FROM {}", table_name);
    let end_date_result: Vec<MaxDateRow> = client.query_collect(query_max).await?;
    let end_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        + Duration::days(end_date_result[0].max_date.0 as i64);

    let query_min = format!("SELECT min(toDate(day)) FROM {}", table_name);
    let start_date_result: Vec<MinDateRow> = client.query_collect(query_min).await?;
    let start_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        + Duration::days(start_date_result[0].min_date.0 as i64);

    println!("Start date: {:?}", start_date);
    println!("End date: {:?}", end_date);

    let mut current_date = start_date;
    while current_date <= end_date {
        let date_str = current_date.to_string();
        let query = format!(
            "SELECT * FROM {} WHERE day >= '{}' AND day < '{}' FORMAT CSV",
            table_name,
            date_str,
            current_date + Duration::days(1)
        );
        let _query_result: Vec<Opensky> = client.query_collect(&query).await?;

        // Example: Writing to a file
        let file_path = format!("./tmp/{}/{}.csv", date_str, date_str);
        println!("Writing to file: {}", file_path);
        let dir_path = Path::new("./tmp").join(&date_str);
        fs::create_dir_all(&dir_path)?;
        let file = File::create(file_path)?;
        let mut writer = Writer::from_writer(file);
        for row in _query_result {
            writer.serialize(row)?;
        }
        writer.flush()?;

        println!("Table split for date: {}", date_str);
        current_date += Duration::days(1);
    }
    Ok(())
}
