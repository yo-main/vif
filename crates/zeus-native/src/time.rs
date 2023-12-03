use crate::errors::NativeError;
use zeus_values::value::Value;

pub fn get_time(stack: &mut Vec<Value>) -> Result<i64, NativeError> {
    Ok(1)
}
