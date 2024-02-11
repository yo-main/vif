use crate::errors::NativeError;
use chrono;

pub fn get_time() -> Result<i64, NativeError> {
    Ok(chrono::Utc::now().timestamp_micros())
}
