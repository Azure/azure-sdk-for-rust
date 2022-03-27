use crate::operations::query::*;
use arrow::{
    array::{
        ArrayRef, BooleanArray, DurationNanosecondArray, Float64Array, Int32Array, Int64Array,
        StringArray,
    },
    compute::cast,
    datatypes::{DataType, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};
use std::sync::Arc;

const SECOND_TO_NANOSECONDS: i64 = 1000000000;
const MINUTES_TO_SECONDS: i64 = 60;
const HOURS_TO_SECONDS: i64 = 60 * MINUTES_TO_SECONDS;
const DAYS_TO_SECONDS: i64 = 24 * HOURS_TO_SECONDS;
const TICK_TO_NANOSECONDS: i64 = 100;

#[inline]
fn to_nanoseconds(days: i64, hours: i64, minutes: i64, seconds: i64, ticks: i64) -> i64 {
    let d_secs = days * DAYS_TO_SECONDS;
    let h_secs = hours * HOURS_TO_SECONDS;
    let m_secs = minutes * MINUTES_TO_SECONDS;
    let total_secs = d_secs + h_secs + m_secs + seconds;
    let rest_in_ns = ticks * TICK_TO_NANOSECONDS;

    total_secs * SECOND_TO_NANOSECONDS + rest_in_ns
}

fn parse_segment(seg: &str) -> i64 {
    let trimmed = seg.trim_start_matches('0');
    if !trimmed.is_empty() {
        trimmed.parse::<i64>().unwrap()
    } else {
        0
    }
}

fn destructure_time(dur: &str) -> (i64, i64, i64) {
    let parts = dur.split(':').collect::<Vec<_>>();
    match parts.as_slice() {
        [hours, minutes, seconds] => (
            parse_segment(hours),
            parse_segment(minutes),
            parse_segment(seconds),
        ),
        _ => (0, 0, 0),
    }
}

/// The timespan format Kusto returns is 'd.hh:mm:ss.ssssss' or 'hh:mm:ss.ssssss' or 'hh:mm:ss'
/// Kusto also stores fractions in ticks: 1 tick = 100 ns
pub fn string_to_duration_i64(dur: Option<&str>) -> Option<i64> {
    let dur = dur?;
    let factor = if dur.starts_with('-') { -1 } else { 1 };
    let parts: Vec<&str> = dur.trim_start_matches('-').split('.').collect();
    let ns = match parts.as_slice() {
        [days, hours, ticks] => {
            let days_ = parse_segment(days);
            let ticks_ = parse_segment(ticks);
            let (hours, minutes, seconds) = destructure_time(hours);
            to_nanoseconds(days_, hours, minutes, seconds, ticks_)
        }
        [first, ticks] => {
            let ticks_ = parse_segment(ticks);
            let (hours, minutes, seconds) = destructure_time(first);
            to_nanoseconds(0, hours, minutes, seconds, ticks_)
        }
        [one] => {
            let (hours, minutes, seconds) = destructure_time(one);
            to_nanoseconds(0, hours, minutes, seconds, 0)
        }
        _ => 0,
    };
    Some(factor * ns)
}

fn convert_array_string(values: Vec<serde_json::Value>) -> ArrayRef {
    let strings: Vec<Option<String>> =
        serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    let strings: Vec<Option<&str>> = strings.iter().map(|opt| opt.as_deref()).collect();
    Arc::new(StringArray::from(strings))
}

// TODO provide a safe variant for datetime conversions (chrono panics)
fn convert_array_datetime_unsafe(values: Vec<serde_json::Value>) -> ArrayRef {
    let strings: Vec<Option<String>> =
        serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    let strings: Vec<Option<&str>> = strings.iter().map(|opt| opt.as_deref()).collect();
    let string_array: ArrayRef = Arc::new(StringArray::from(strings));
    cast(
        &string_array,
        &DataType::Timestamp(TimeUnit::Nanosecond, None),
    )
    .unwrap()
}

fn safe_map_f64(value: serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::String(val) if val == "NaN" => None,
        serde_json::Value::String(val) if val == "Infinity" => Some(f64::INFINITY),
        serde_json::Value::String(val) if val == "-Infinity" => Some(-f64::INFINITY),
        _ => serde_json::from_value(value).unwrap(),
    }
}

fn convert_array_float(values: Vec<serde_json::Value>) -> ArrayRef {
    let reals: Vec<Option<f64>> = values.into_iter().map(safe_map_f64).collect();
    Arc::new(Float64Array::from(reals))
}

fn convert_array_timespan(values: Vec<serde_json::Value>) -> ArrayRef {
    let strings: Vec<Option<String>> =
        serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    let durations: Vec<Option<i64>> = strings
        .iter()
        .map(|opt| opt.as_deref())
        .map(string_to_duration_i64)
        .collect();
    Arc::new(DurationNanosecondArray::from(durations))
}

fn convert_array_bool(values: Vec<serde_json::Value>) -> ArrayRef {
    let bools: Vec<Option<bool>> =
        serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    Arc::new(BooleanArray::from(bools))
}

fn convert_array_i32(values: Vec<serde_json::Value>) -> ArrayRef {
    let ints: Vec<Option<i32>> = serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    Arc::new(Int32Array::from(ints))
}

fn convert_array_i64(values: Vec<serde_json::Value>) -> ArrayRef {
    let ints: Vec<Option<i64>> = serde_json::from_value(serde_json::Value::Array(values)).unwrap();
    Arc::new(Int64Array::from(ints))
}

pub fn convert_column(data: Vec<serde_json::Value>, column: Column) -> (Field, ArrayRef) {
    match column.column_type {
        ColumnType::String => (
            Field::new(column.column_name.as_str(), DataType::Utf8, true),
            convert_array_string(data),
        ),
        ColumnType::Bool | ColumnType::Boolean => (
            Field::new(column.column_name.as_str(), DataType::Boolean, true),
            convert_array_bool(data),
        ),
        ColumnType::Int => (
            Field::new(column.column_name.as_str(), DataType::Int32, true),
            convert_array_i32(data),
        ),
        ColumnType::Long => (
            Field::new(column.column_name.as_str(), DataType::Int64, true),
            convert_array_i64(data),
        ),
        ColumnType::Real => (
            Field::new(column.column_name.as_str(), DataType::Float64, true),
            convert_array_float(data),
        ),
        ColumnType::Datetime => (
            Field::new(
                column.column_name.as_str(),
                DataType::Timestamp(TimeUnit::Nanosecond, None),
                true,
            ),
            convert_array_datetime_unsafe(data),
        ),
        ColumnType::Timespan => (
            Field::new(
                column.column_name.as_str(),
                DataType::Duration(TimeUnit::Nanosecond),
                true,
            ),
            convert_array_timespan(data),
        ),
        _ => todo!(),
    }
}

pub fn convert_table(table: DataTable) -> RecordBatch {
    let mut buffer: Vec<Vec<serde_json::Value>> = Vec::with_capacity(table.columns.len());
    let mut fields: Vec<Field> = Vec::with_capacity(table.columns.len());
    let mut columns: Vec<ArrayRef> = Vec::with_capacity(table.columns.len());

    for _ in 0..table.columns.len() {
        buffer.push(Vec::with_capacity(table.rows.len()));
    }
    table.rows.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(idx, value)| buffer[idx].push(value))
    });

    buffer
        .into_iter()
        .zip(table.columns.into_iter())
        .map(|(data, column)| convert_column(data, column))
        .for_each(|(field, array)| {
            fields.push(field);
            columns.push(array);
        });

    RecordBatch::try_new(Arc::new(Schema::new(fields)), columns).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_column() {
        let data = r#" {
            "ColumnName": "int_col",
            "ColumnType": "int"
        } "#;

        let c: Column = serde_json::from_str(data).expect("deserialize error");
        let ref_col = Column {
            column_name: "int_col".to_string(),
            column_type: ColumnType::Int,
        };
        assert_eq!(c, ref_col)
    }

    #[test]
    fn deserialize_table() {
        let data = r#" {
            "FrameType": "DataTable",
            "TableId": 1,
            "TableName": "Deft",
            "TableKind": "PrimaryResult",
            "Columns": [
                {
                    "ColumnName": "int_col",
                    "ColumnType": "int"
                }
            ],
            "Rows": []
        } "#;

        let t: DataTable = serde_json::from_str(data).expect("deserialize error");
        let ref_tbl = DataTable {
            table_id: 1,
            table_name: "Deft".to_string(),
            table_kind: TableKind::PrimaryResult,
            columns: vec![Column {
                column_name: "int_col".to_string(),
                column_type: ColumnType::Int,
            }],
            rows: vec![],
        };
        assert_eq!(t, ref_tbl)
    }

    #[test]
    fn string_conversion() {
        let refs: Vec<(&str, i64)> = vec![
            ("1.00:00:00.0000000", 86400000000000),
            ("01:00:00.0000000", 3600000000000),
            ("01:00:00", 3600000000000),
            ("00:05:00.0000000", 300000000000),
            ("00:00:00.0000001", 100),
            ("-01:00:00", -3600000000000),
            ("-1.00:00:00.0000000", -86400000000000),
        ];

        for (from, to) in refs {
            assert_eq!(string_to_duration_i64(Some(from)), Some(to));
        }
    }
}
