use crate::errors::NativeError;
use chrono;
use vif_objects::stack_value::StackValue;

pub fn get_time() -> Result<i64, NativeError> {
    Ok(chrono::Utc::now().timestamp_micros())
}
pub fn sleep(value: &StackValue<'_>) -> Result<(), NativeError> {
    let duration = match value {
        StackValue::Integer(i) => std::time::Duration::from_secs(*i as u64),
        StackValue::Float(i) => std::time::Duration::from_secs(*i as u64),
        v => {
            return Err(NativeError::Generic(format!(
                "Argument error on sleep builtin: {}",
                v
            )))
        }
    };

    Ok(std::thread::sleep(duration))
}
