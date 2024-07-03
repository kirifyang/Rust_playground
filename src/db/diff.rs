use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};


pub fn split_csv_by_date(csv_file: &str, output_folder: &str) -> io::Result<()> {
    // Create the output folder if it doesn't exist
    fs::create_dir_all(output_folder)?;

    // Open the CSV file
    let file = fs::File::open(csv_file)?;
    let reader = io::BufReader::new(file);

    // Iterate over each line in the CSV file
    for line in reader.lines() {
        let line = line?;

        // Parse the date from the CSV line (assuming the date is in the first column)
        let date = line.split(',').next().unwrap_or("");

        // Create the output file path based on the date
        let output_file = Path::new(output_folder)
            .join(date)
            .with_extension("csv");

        // Open the output file in append mode
        let mut output = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)?;

        // Write the line to the output file
        writeln!(output, "{}", line)?;
    }

    Ok(())
}

pub fn split_csv_by_month(csv_file: &str, output_folder: &str) -> io::Result<()> {
    // Create the output folder if it doesn't exist
    fs::create_dir_all(output_folder)?;

    // Open the CSV file
    let file = fs::File::open(csv_file)?;
    let reader = io::BufReader::new(file);

    // Iterate over each line in the CSV file
    for line in reader.lines() {
        let line = line?;

        // Parse the date from the CSV line (assuming the date is in the first column)
        let date = line.split(',').next().unwrap_or("");

        // Extract the month from the date
        let month = date.split('-').nth(1).unwrap_or("");

        // Create the output file path based on the month
        let output_file = Path::new(output_folder)
            .join(month)
            .with_extension("csv");

        // Open the output file in append mode
        let mut output = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)?;

        // Write the line to the output file
        writeln!(output, "{}", line)?;
    }

    Ok(())
}