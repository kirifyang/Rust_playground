#!/bin/bash

cd ./clickhouse/click_data/user_files

# Download the opensky data
wget -O- https://zenodo.org/records/5092942 | grep -oE 'https://zenodo.org/records/5092942/files/flightlist_[0-9]+_[0-9]+\.csv\.gz' | xargs wget

