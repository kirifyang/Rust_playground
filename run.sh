#!/bin/bash
export ROOT_DIR=$PWD
export USERS_FILE_DIR='./clickhouse/click_data/user_files'

# Run setting.sh
source ./config/setting.sh
if [ -z "$(ls -A ${USERS_FILE_DIR})" ]; then
    source ./config/build_db.sh
    echo "Data downloaded successfully!"
else
    echo "Data already exists!"
fi

# Run clickhouse
./clickhouse/clickhouse server --config-file=./config/clickhouse_config.xml
