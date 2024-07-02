#!/bin/bash

USERS_FILE_DIR='./clickhouse/click_data/user_files'
RECEIVE_DIR='./receive'

mkdir -p "$RECEIVE_DIR"
mkdir -p "_archive"

# Unzip the user files
find "$USERS_FILE_DIR" -name '*.csv.gz' -exec gunzip {} \;

# Move and split files by date
for file in "$USERS_FILE_DIR"/*.csv; do
    # Extract start and end dates from the filename
    filename=$(basename "$file")
    if [[ $filename =~ flightlist_([0-9]{8})_([0-9]{8})\.csv ]]; then
        start_date=${BASH_REMATCH[1]}
        end_date=${BASH_REMATCH[2]}
        
        # Convert dates to YYYY-MM-DD format for directory naming
        start_date_fmt=$(date -d "${start_date:0:4}-${start_date:4:2}-${start_date:6:2}" +%Y-%m-%d)
        end_date_fmt=$(date -d "${end_date:0:4}-${end_date:4:2}-${end_date:6:2}" +%Y-%m-%d)
        
        # Create a directory for the date range and move the file there
        date_dir="$RECEIVE_DIR/${start_date_fmt}_to_${end_date_fmt}"
        mkdir -p "$date_dir"
        mv "$file" "$date_dir"
    fi
done

