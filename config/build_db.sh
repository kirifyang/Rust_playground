#!/bin/bash

DATA_DIR=./clickhouse/click_data/user_files

cd DATA_DIR

# Download the opensky data
wget -O- https://zenodo.org/records/5092942 | grep -oE 'https://zenodo.org/records/5092942/files/flightlist_[0-9]+_[0-9]+\.csv\.gz' | xargs wget

