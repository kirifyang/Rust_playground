#!/bin/bash

# Install ClickHouse
if [ ! -f click_data ]; then
    mkdir click_data
fi

cd click_data

if [ ! -f clickhouse ]; then
    curl https://clickhouse.com/ | sh
fi

export CLICKHOUSE_CONFIG_FILE=clickhouse_config.xml
echo $CLICKHOUSE_CONFIG_FILE

killall clickhouse

./clickhouse server --config-file="../$CLICKHOUSE_CONFIG_FILE"
