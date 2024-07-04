#!/bin/bash
TARGET_DATE='2019-02-28'
TEST_FILE_DIR='./test'
RECEIVE_DIR='./receive'

mkdir -p "tmp"
mkdir -p "$RECEIVE_DIR"
mkdir -p "_archive"

mkdir -p "_archive/${TARGET_DATE}"


find_closest_previous_data_date() {
    local target_date=$1
    local target_month=$(date -j -f "%Y-%m-%d" "$target_date" "+%Y-%m")
    local previous_month=$(date -j -f "%Y-%m-%d" -v "-1m" "$target_date" "+%Y-%m")
    local closest_date=""

    # Get the list of dates in the target month and the previous month
    local target_month_dates=$(ls "./_archive/${target_month}-" | awk '{print $2}' | sed 's:/*$::')
    local previous_month_dates=$(ls "./_archive/${previous_month}-" | awk '{print $2}' | sed 's:/*$::')

    # Combine the list of dates in the target month and the previous month and sort them in descending order
    local all_dates=$(echo -e "${target_month_dates}\n${previous_month_dates}" | sort -r)

    # Find the closest date
    local closest_date=$(echo "$all_dates" | awk -v target_date="$target_date" '$0 < target_date {print; exit}')

    echo "$closest_date"
}

( echo "name,value"
  # [前日差分抽出用]
  # 通常時は以下のようなファイルを参照する。
  #   当日分：receive/<table>.utf-8.withheader.csv
  #   前回分：_archive/2024-02-02/<table>.utf-8.withheader.csv
  # 環境変数 TARGET_DATE が YYYY-MM-DD に指定されている場合は遡及実行を意味する。
  #   当日分：_archive/${TARGET_DATE}/<table>.utf-8.withheader.csv
  #   前回分：_archive/${TARGET_DATEの前回分}/<table>.utf-8.withheader.csv
if [ ! -z "${TARGET_DATE+x}" ]; then
    today_dir="_archive/${TARGET_DATE}"
    # TARGET_DATEより前の日付を探す
    closest_yesterday_dir=$(find_closest_previous_data_date "$TARGET_DATE")
    echo "today_receive_dir,${today_dir}"
    echo "yesterday_receive_dir,_archive/${closest_yesterday_dir}"
  else
    today_dir="receive"
    today=$(date +%Y-%m-%d)
    closest_yesterday_dir=$(find_closest_previous_data_date "$today")
    echo "today_receive_dir,${today_dir}"
    echo "yesterday_receive_dir,_archive/${closest_yesterday_dir}"
  fi
) > "/tmp/macro_${INTEGRATION_JOB_ID}.csv"
