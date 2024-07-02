#!/bin/bash

# Install ClickHouse
if [ -d "clickhouse" ]; then
    killall clickhouse
    echo "ClickHouse already installed."
fi
mkdir clickhouse && cd clickhouse
curl https://clickhouse.com/ | sh

# Set environment

mkdir -p clickhouse/click_data/user_files

echo "ClickHouse installed successfully! "
