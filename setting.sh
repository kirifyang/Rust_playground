#!/bin/bash

# Install ClickHouse
if [ -d "clickhouse" ]; then
    killall clickhouse
    echo "ClickHouse already installed. Starting server..."
    sleep 3
    ./clickhouse/clickhouse server --config-file=clickhouse_config.xml
    exit 0
fi
mkdir clickhouse && cd clickhouse
curl https://clickhouse.com/ | sh

# Set environment

mkdir click_data && cd click_data
mkdir user_files

echo "ClickHouse installed successfully! Starting server..."
sleep 3
../clickhouse server --config-file=../../clickhouse_config.xml

