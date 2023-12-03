use crate::errors::NativeError;
use zeus_values::value::Value;

pub fn print(iter: &[Value<'_>]) -> Result<(), NativeError> {
    for value in iter {
        print!("{value}");
    }

    Ok(())
}
