use crate::errors::NativeError;
use zeus_objects::value::Value;

pub fn print(iter: &[Value<'_>]) -> Result<(), NativeError> {
    println!(
        "{}",
        iter.iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(())
}
