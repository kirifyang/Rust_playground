#!/bin/bash

# Install ClickHouse
if [ -d "clickhouse" ]; then
    killall clickhouse
    echo "ClickHouse already installed."
fi
mkdir clickhouse && cd clickhouse
curl https://clickhouse.com/ | sh

# Set environment

mkdir click_data && cd click_data
mkdir user_files

echo "ClickHouse installed successfully! "
