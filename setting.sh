#!/bin/bash

# Install ClickHouse
if [ ! -f clickhouse ]; then
    mkdir clickhouse
fi

cd clickhouse

if [ ! -f clickhouse ]; then
    curl https://clickhouse.com/ | sh
fi

export CLICKHOUSE_CONFIG_FILE=clickhouse_config.xml
echo $CLICKHOUSE_CONFIG_FILE

killall clickhouse

./clickhouse server --config-file="../$CLICKHOUSE_CONFIG_FILE"
