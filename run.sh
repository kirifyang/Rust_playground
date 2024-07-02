#!/bin/bash

CONFIG_DIR=config

# Run setting.sh
source ./$CONFIG_DIR/setting.sh
if [ -z "$(ls -A ./clickhouse/click_data/user_files)" ]; then
    source ./$CONFIG_DIR/build_db.sh
    echo "Data downloaded successfully!"
fi

# Run clickhouse
./clickhouse/clickhouse server --config-file=./$CONFIG_DIR/clickhouse_config.xml
