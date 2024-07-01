#!/bin/bash

CONFIG=config

# Run setting.sh
source ./$CONFIG/setting.sh
source ./$CONFIG/build_db.sh

./clickhouse/clickhouse server --config-file=clickhouse_config.xml
