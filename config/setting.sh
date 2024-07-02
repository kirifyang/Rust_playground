#!/bin/bash

# Function to stop ClickHouse process
stop_clickhouse() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Stopping ClickHouse on macOS..."
        pkill -9 clickhouse
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Stopping ClickHouse on Linux..."
        killall clickhouse
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi
}

# Install ClickHouse
if [ -d "clickhouse" ]; then
    stop_clickhouse
    echo "ClickHouse already installed and stopped."
else
    mkdir clickhouse && cd clickhouse || exit
    curl https://clickhouse.com/ | sh
    cd ..
fi

# Set environment
mkdir -p clickhouse/click_data/user_files

echo "ClickHouse installed successfully!"
