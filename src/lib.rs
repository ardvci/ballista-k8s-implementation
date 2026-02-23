use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

pub const DATA_PATH: &str = "/mnt/";

pub fn get_log_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("timestamp", DataType::Int64, false),
        Field::new("user_id", DataType::Int64, false),
        Field::new("response_time_ms", DataType::Float64, false),
        Field::new("status_code", DataType::Int32, false),
    ]))
}