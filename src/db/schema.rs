use futures::future::err;
use klickhouse::{Date, DateTime, Row};
use serde::{Deserialize, Serialize};

// Schema of the using table

#[derive(Debug, Deserialize, Row)]
pub(crate) struct MaxDateRow {
    #[serde(rename = "max(toDate(day))")]
    max_date: Date,
}
#[derive(Debug, Deserialize, Row)]
pub(crate) struct MinDateRow {
    #[serde(rename = "min(toDate(day))")]
    min_date: Date,
}

#[derive(Row, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Opensky {
    callsign: String,
    number: String,
    icao24: String,
    registration: String,
    typecode: String,
    origin: String,
    destination: String,
    firstseen: DateTime,
    lastseen: DateTime,
    day: DateTime,
    latitude_1: f64,
    longitude_1: f64,
    altitude_1: f64,
    latitude_2: f64,
    longitude_2: f64,
    altitude_2: f64,
}

impl Opensky {
    pub fn schema() -> String {
        "callsign String, number String, icao24 String, registration String, typecode String, origin String, destination String, firstseen DateTime, lastseen DateTime, day DateTime, latitude_1 Float64, longitude_1 Float64, altitude_1 Float64, latitude_2 Float64, longitude_2 Float64, altitude_2 Float64"
            .to_string()
    }
    pub fn table_name() -> &'static str {
        "Opensky"
    }
    pub fn find_target_column(target: &str) -> Vec<String> {
        match Opensky::column_names().iter().position(|x| x == target) {
            Some(i) => Opensky::column_names()[i].clone(),
            _ => vec![],
        }
        return vec![];
    }

        pub fn columns() -> Vec<String> {
            vec![
                "callsign".to_string(),
                "number".to_string(),
                "icao24".to_string(),
                "registration".to_string(),
                "typecode".to_string(),
                "origin".to_string(),
                "destination".to_string(),
                "firstseen".to_string(),
                "lastseen".to_string(),
                "day".to_string(),
                "latitude_1".to_string(),
                "longitude_1".to_string(),
                "altitude_1".to_string(),
                "latitude_2".to_string(),
                "longitude_2".to_string(),
                "altitude_2".to_string(),
            ]
        }
    }


    // Schema of the intergration job table
}

enum TransformVariant {
    Union,
    Freeform,
    UpsertCapture,
}

enum SinkVariant {
    BigQuery,
    Chikyu,
    Ma,
}

enum BigQueryWriteOption {
    truncate,
    append,
    empty,
}

enum BigQueryTimePartitioningType {
    day,
    hour,
    month,
    year,
}

enum ChikyuWriteSemantics {
    insert,
    upsert,
    update,
}

// key_search_option: id, value, display_name
// 空欄ならばデフォルト。ただし空欄にできるのはinsert_onlyの場合に限る
enum ChikyuFindRecordBy {
    id,
    value,
    display_name,
}
enum chikyu_find_record_matching_field {
    account_id,
    stock_cd,
}

pub struct IntergrationJob {
    // general
    integration_job_id: i64,
    task_order: i64,
    id: String,
    is_enabled: bool,
    comment: String,
    transform_variant: TransformVariant,
    freeform_select_query: Option<klickhouse::RawRow>,
    upsertcapture_old_source_ids: Option<Vec<String>>,
    sink_variant: SinkVariant,
    // bigquery configuration
    bigquery_gcs_connection_id: Option<i64>,
    bigquery_project: Option<String>,
    bigquery_dataset: Option<String>,
    bigquery_table: Option<String>,
    bigquery_write_option: Option<BigQueryWriteOption>,
    bigquery_time_partitioning_field: Option<String>,
    bigquery_time_partitioning_type: Option<BigQueryTimePartitioningType>,
    bigquery_time_partitioning_expiration: Option<i64>,
    bigquery_range_partitioning: Option<i64>,
    bigquery_require_partition_filter: Option<bool>,
    bigquery_clustering_fields: Option<Vec<String>>,
    // chikyu configuration
    chikyu_connection_id: Option<i64>,
    chikyu_organization: Option<String>,
    chikyu_collection: Option<String>,
    chikyu_write_semantics: Option<ChikyuWriteSemantics>,
    chikyu_find_record_by: Option<ChikyuFindRecordBy>,
    chikyu_find_record_matching_field: Option<chikyu_find_record_matching_field>,
    chikyu_create_list: Option<String>,
    chikyu_feature_omit_save_notify: Option<bool>,
    chikyu_feature_omit_all_validation_rules: Option<bool>,
    chikyu_feature_omit_validation_rules_by_id_set: Option<Vec<i64>>,
    chikyu_optimize_enable_all_speedup_options: Option<bool>,
    // ma configuration
    ma_aws_connection_id: Option<i64>,
    ma_source: Option<String>,
    ma_detail_type: Option<String>,
    ma_bus_name: Option<String>,
    ma_organ_id: Option<i64>,
    ma_kind: Option<String>,
    ma_unique_fields: Option<Vec<String>>,
}

impl IntergrationJob {
    pub fn schema() -> String {
        "integration_job_id Int64, task_order Int64, id String, is_enabled UInt8, comment String, transform_variant String, freeform_select_query String, upsertcapture_old_source_ids Array(String), sink_variant String, bigquery_gcs_connection_id Int64, bigquery_project String, bigquery_dataset String, bigquery_table String, bigquery_write_option String, bigquery_time_partitioning_field String, bigquery_time_partitioning_type String, bigquery_time_partitioning_expiration Int64, bigquery_range_partitioning Int64, bigquery_require_partition_filter UInt8, bigquery_clustering_fields Array(String), chikyu_connection_id Int64, chikyu_organization String, chikyu_collection String, chikyu_write_semantics String, chikyu_find_record_by String, chikyu_find_record_matching_field String, chikyu_create_list String, chikyu_feature_omit_save_notify UInt8, chikyu_feature_omit_all_validation_rules UInt8, chikyu_feature_omit_validation_rules_by_id_set Array(Int64), chikyu_optimize_enable_all_speedup_options UInt8, ma_aws_connection_id Int64, ma_source String, ma_detail_type String, ma_bus_name String, ma_organ_id Int64, ma_kind String, ma_unique_fields Array(String)"
            .to_string()
    }
    pub fn table_name() -> &'static str {
        "IntergrationJob"
    }
}
